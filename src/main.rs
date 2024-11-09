use actix_files::Files;
use actix_web::*;
use leptos::*;
use leptos_actix::{generate_route_list, LeptosRoutes};
use sol_tools::app::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    let routes = generate_route_list(App);

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .leptos_routes(leptos_options.clone(), routes.clone(), App)
            .service(Files::new("/", site_root))
            .wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

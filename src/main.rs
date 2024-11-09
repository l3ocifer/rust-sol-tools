use leptos::*;
use sol_tools::app::App;

#[cfg(feature = "ssr")]
use leptos_actix::{generate_route_list, LeptosRoutes};
#[cfg(feature = "ssr")]
use actix_web::{web, App as ActixApp, HttpServer, middleware::Logger, middleware::Compress};
#[cfg(feature = "ssr")]
use sol_tools::routes::{metadata::upload_metadata, contract::create_token_route};

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use leptos_config::get_configuration;
    use actix_web::middleware;

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;

    HttpServer::new(move || {
        let leptos_options = conf.leptos_options.clone();
        
        ActixApp::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .service(upload_metadata)
            .service(create_token_route)
            .leptos_routes(
                leptos_options.clone(),
                generate_route_list(|| view! { <App/> }),
                || view! { <App/> }
            )
            .app_data(web::Data::new(leptos_options))
    })
    .workers(2)
    .backlog(1024)
    .bind(&addr)?
    .run()
    .await
}

#[cfg(not(feature = "ssr"))]
fn main() {
    // Client-side entry point (can be left empty)
}

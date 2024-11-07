use leptos::*;
use leptos_actix::*;
use actix_web::{web, App, HttpServer};
use crate::routes::metadata::upload_metadata;
use crate::routes::contract::create_token_route;
use sol_tools::app::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;

    HttpServer::new(move || {
        let leptos_options = conf.leptos_options.clone();
        
        App::new()
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .service(upload_metadata)
            .service(create_token_route)
            .leptos_routes(
                leptos_options.clone(),
                generate_route_list,
                sol_tools::app::App::new,
            )
            .app_data(web::Data::new(leptos_options))
    })
    .bind(&addr)?
    .run()
    .await
}

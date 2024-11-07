use leptos::*;
#[cfg(feature = "ssr")]
use leptos_actix::*;
#[cfg(feature = "ssr")]
use actix_web::{web, App, HttpServer};
#[cfg(feature = "ssr")]
use sol_tools::routes::{metadata::upload_metadata, contract::create_token_route};

#[cfg(feature = "ssr")]
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
                generate_route_list(|cx| view! { cx, <sol_tools::app::App/> }),
                |cx| view! { cx, <sol_tools::app::App/> }
            )
            .app_data(web::Data::new(leptos_options))
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    mount_to_body(|| view! { <sol_tools::app::App/> });
}

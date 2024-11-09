use leptos::*;
use crate::app::App;

#[cfg(feature = "ssr")]
use leptos_actix::{generate_route_list, LeptosRoutes};
#[cfg(feature = "ssr")]
use actix_web::{web, App as ActixApp, HttpServer, middleware::Logger};
#[cfg(feature = "ssr")]
use crate::routes::{metadata::upload_metadata, contract::create_token_route};

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use leptos_config::get_configuration;

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;

    HttpServer::new(move || {
        let leptos_options = conf.leptos_options.clone();
        
        ActixApp::new()
            .wrap(Logger::default())
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .service(upload_metadata)
            .service(create_token_route)
            .leptos_routes(
                leptos_options.clone(),
                generate_route_list(|cx| view! { cx, <App/> }),
                |cx| view! { cx, <App/> }
            )
            .app_data(web::Data::new(leptos_options))
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(not(feature = "ssr"))]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    use leptos::*;

    leptos::mount_to_body(|cx| view! { cx, <App/> });
}

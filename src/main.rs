use leptos::*;
use sol_tools::app::App;

#[cfg(not(target_arch = "wasm32"))]
use {
    actix_web::{web, App as ActixApp, HttpServer, middleware::Logger},
    leptos_actix::{generate_route_list, LeptosRoutes},
    sol_tools::routes::{metadata::upload_metadata, contract::create_token_route},
    env_logger::Env,
    leptos_config::get_configuration,
};

#[cfg(not(target_arch = "wasm32"))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options.clone();
    let addr = leptos_options.site_addr;

    let routes = generate_route_list(|cx| view! { cx, <App/> });

    HttpServer::new(move || {
        ActixApp::new()
            .wrap(Logger::default())
            .service(upload_metadata)
            .service(create_token_route)
            .leptos_routes(
                &leptos_options,
                routes.clone(),
                |cx| view! { cx, <App/> },
            )
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    use app::*;
    use leptos::*;

    _ = console_error_panic_hook::set_once();

    leptos::mount_to_body(move || {
        view! { <App/> }
    });
}

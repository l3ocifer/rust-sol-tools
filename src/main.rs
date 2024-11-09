use leptos::*;
use sol_tools::app::App;

#[cfg(feature = "ssr")]
use {
    actix_web::{
        web, 
        App as ActixApp, 
        HttpServer,
        middleware::Logger,
    },
    leptos_actix::{generate_route_list, LeptosRoutes},
    sol_tools::routes::{metadata::upload_metadata, contract::create_token_route},
    env_logger::Env,
    leptos_config::get_configuration,
};

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use leptos_config::get_configuration;
    use env_logger::Env;

    env_logger::init_from_env(Env::default().default_filter_or("info"));

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
                generate_route_list(|| view! { <App/> }),
                || view! { <App/> }
            )
            .app_data(web::Data::new(leptos_options))
    })
    .workers(2)
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

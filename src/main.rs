use leptos::*;
use sol_tools::app::App;

#[cfg(not(target_arch = "wasm32"))]
use {
    actix_files::Files,
    actix_web::{App as ActixApp, HttpServer, middleware::Logger},
    leptos_actix::{generate_route_list, LeptosRoutes, handle_server_fns},
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
    let routes = generate_route_list(|| view! { <App/> });

    HttpServer::new(move || {
        let leptos_options = &leptos_options;
        let site_root = leptos_options.site_root.clone();

        ActixApp::new()
            .wrap(Logger::default())
            .service(handle_server_fns())
            .service(Files::new("/pkg", format!("{}/pkg", site_root)))
            .service(Files::new("/public", format!("{}/public", site_root)))
            .service(Files::new("/assets", format!("{}/assets", site_root)))
            .service(upload_metadata)
            .service(create_token_route)
            .leptos_routes(
                leptos_options.clone(),
                routes.clone(),
                || view! { <App/> }
            )
            .service(Files::new("/", site_root).index_file("index.html"))
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(target_arch = "wasm32")]
fn main() {}

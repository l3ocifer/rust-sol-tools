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
    sol_tools::routes::{metadata::upload_metadata, contract::create_token_route}
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

#[cfg(not(feature = "ssr"))]
fn main() {
    // no-op for client-side
}

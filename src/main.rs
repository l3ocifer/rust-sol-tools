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
                generate_route_list(|| view! { <sol_tools::app::App/> }),
                || view! { <sol_tools::app::App/> }
            )
            .app_data(web::Data::new(leptos_options))
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use crate::App;
    use leptos::*;

    leptos::mount_to_body(|| {
        view! { <App/> }
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use actix_files::Files;
    use actix_web::{middleware, App as ActixApp, HttpServer};
    use crate::routes::create_token_route;

    #[actix_web::main]
    async fn server_main() -> std::io::Result<()> {
        HttpServer::new(|| {
            ActixApp::new()
                .wrap(middleware::Logger::default())
                .service(create_token_route)
                .service(Files::new("/", "./static").index_file("index.html"))
            // ... Add other services or middleware
        })
        .bind("127.0.0.1:3000")?
        .run()
        .await
    }

    if let Err(e) = server_main().await {
        eprintln!("Server error: {}", e);
    }
}

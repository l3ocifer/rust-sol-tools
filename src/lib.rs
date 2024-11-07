pub mod app;
pub mod wallet;
pub mod upload;
pub mod token;
pub mod routes;
pub mod utils;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    use leptos::*;

    _ = console_error_panic_hook::set_once();
    
    logging::log!("Initializing application...");
    
    mount_to_body(move || {
        view! {
            <App/>
        }
    });
}

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    _ = CreateToken::register();
    _ = UploadMetadata::register();
}

pub mod app;
pub mod wallet;
pub mod token;

#[cfg(feature = "ssr")]
pub mod upload;

#[cfg(feature = "ssr")]
pub mod routes;

#[cfg(feature = "ssr")]
pub mod utils;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn hydrate() {
    use app::*;
    use leptos::*;

    _ = console_error_panic_hook::set_once();
    
    logging::log!("Initializing application...");
    
    mount_to_body(|| {
        view! {
            <App/>
        }
    });
}

pub mod app;
pub mod wallet;
pub mod token;

#[cfg(not(target_arch = "wasm32"))]
pub mod upload;

#[cfg(not(target_arch = "wasm32"))]
pub mod routes;

#[cfg(not(target_arch = "wasm32"))]
pub mod utils;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn hydrate() {
    use app::*;
    use leptos::*;

    _ = console_error_panic_hook::set_once();
    
    mount_to_body(|| {
        view! {
            <App/>
        }
    });
}

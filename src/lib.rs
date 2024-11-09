pub mod app;
pub mod wallet;
pub mod token;

#[cfg(target_arch = "wasm32")]
pub mod utils {
    pub mod pinata;
}

#[cfg(not(target_arch = "wasm32"))]
pub mod utils;

#[cfg(not(target_arch = "wasm32"))]
pub mod routes;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn hydrate() {
    use app::*;
    use leptos::*;

    _ = console_error_panic_hook::set_once();
    
    #[cfg(debug_assertions)]
    console_log::init_with_level(log::Level::Debug)
        .expect("Failed to initialize logger");
    
    mount_to_body(move || {
        view! { <App/> }
    });
}

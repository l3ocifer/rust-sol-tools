mod phantom;
mod metamask;

use leptos::*;
use serde::{Deserialize, Serialize};

pub use phantom::connect_phantom;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WalletState {
    pub connected: bool,
    pub address: Option<String>,
    pub wallet_type: Option<WalletType>,
    pub error: Option<String>,
    pub connecting: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum WalletType {
    Phantom,
    MetaMask,
}

#[derive(Clone)]
pub struct WalletContext {
    pub state: ReadSignal<WalletState>,
    pub(crate) set_state: WriteSignal<WalletState>,
}

impl WalletContext {
    pub async fn connect(&self, wallet_type: WalletType) -> Result<(), String> {
        match wallet_type {
            WalletType::Phantom => connect_phantom(self).await,
            WalletType::MetaMask => connect_metamask(self).await,
        }
    }

    pub fn disconnect(&self) {
        self.set_state.update(|state| {
            state.connected = false;
            state.address = None;
            state.wallet_type = None;
            state.error = None;
            state.connecting = false;
        });
    }

    pub fn set_error(&self, error: &str) {
        self.set_state.update(|state| {
            state.error = Some(error.to_string());
        });
    }
}

#[component]
pub fn WalletProvider(children: Children) -> impl IntoView {
    let (state, set_state) = create_signal(WalletState {
        connected: false,
        address: None,
        wallet_type: None,
        error: None,
        connecting: false,
    });

    provide_context(WalletContext {
        state,
        set_state,
    });

    children()
}

pub async fn connect_metamask(wallet_context: &WalletContext) -> Result<(), String> {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen_futures::JsFuture;
        use web_sys::window;
        use js_sys::{Function, Promise, Reflect};
        use leptos::SignalUpdate;

        let window = window().ok_or("No window object")?;
        let ethereum = Reflect::get(&window, &JsValue::from_str("ethereum"))
            .map_err(|_| "No ethereum object in window")?;

        // Check if MetaMask is installed
        let is_metamask = Reflect::get(&ethereum, &JsValue::from_str("isMetaMask"))
            .map_err(|_| "Failed to access isMetaMask property")?
            .as_bool()
            .unwrap_or(false);

        if !is_metamask {
            wallet_context.set_error("MetaMask not installed");
            return Err("MetaMask not installed".to_string());
        }

        // Request accounts
        let request_fn = Reflect::get(&ethereum, &JsValue::from_str("request"))
            .map_err(|_| "Failed to get request function")?
            .dyn_into::<Function>()
            .map_err(|_| "Request is not a function")?;

        let params = JsValue::NULL;
        let method = JsValue::from_str("eth_requestAccounts");

        let request_params = js_sys::Object::new();
        Reflect::set(&request_params, &JsValue::from_str("method"), &method)
            .map_err(|_| "Failed to set method")?;
        if !params.is_null() {
            Reflect::set(&request_params, &JsValue::from_str("params"), &params)
                .map_err(|_| "Failed to set params")?;
        }

        let promise = request_fn.call1(&ethereum, &request_params)
            .map_err(|_| "Failed to call request")?;

        let accounts_js = JsFuture::from(Promise::from(promise))
            .await
            .map_err(|e| format!("Connection rejected: {:?}", e))?;

        let accounts_array = js_sys::Array::from(&accounts_js);
        if accounts_array.length() > 0 {
            let address = accounts_array.get(0).as_string()
                .ok_or("Invalid address format")?;
            wallet_context.set_state.update(|state| {
                state.connected = true;
                state.address = Some(address);
                state.wallet_type = Some(WalletType::MetaMask);
                state.error = None;
            });
            Ok(())
        } else {
            wallet_context.set_error("No accounts returned");
            Err("No accounts returned".to_string())
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = wallet_context;
        Err("MetaMask connection not supported on this platform".to_string())
    }
} 
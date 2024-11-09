use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;
use js_sys::{Function, Promise, Reflect};
use crate::wallet::{WalletContext, WalletType};
use leptos::SignalUpdate;

pub async fn connect_metamask(wallet_context: &WalletContext) -> Result<(), String> {
    #[cfg(target_arch = "wasm32")]
    {
        let window = window().ok_or("No window object")?;
        let ethereum = Reflect::get(&window, &JsValue::from_str("ethereum"))
            .map_err(|_| "No ethereum object in window")?;
        
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
        
        let request_params = js_sys::Object::new();
        js_sys::Reflect::set(&request_params, &"method".into(), &"eth_requestAccounts".into())
            .map_err(|_| "Failed to set request method")?;
        
        let promise = request_fn.call1(&ethereum, &request_params)
            .map_err(|_| "Failed to call request")?;
        
        let accounts = JsFuture::from(Promise::from(promise))
            .await
            .map_err(|e| format!("Connection rejected: {:?}", e))?;
        
        let accounts_array = js_sys::Array::from(&accounts);
        let address = accounts_array.get(0)
            .as_string()
            .ok_or("No account returned")?;
        
        wallet_context.set_state.update(|state| {
            state.connected = true;
            state.address = Some(address);
            state.wallet_type = Some(WalletType::MetaMask);
            state.error = None;
        });
        
        Ok(())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = wallet_context;
        Err("MetaMask wallet connection not supported on this platform".to_string())
    }
} 
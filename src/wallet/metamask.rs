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
        let request_method = JsValue::from_str("eth_requestAccounts");
        let params = js_sys::Array::new();

        let request_params = js_sys::Object::new();
        Reflect::set(&request_params, &JsValue::from_str("method"), &request_method)
            .map_err(|_| "Failed to set method")?;
        Reflect::set(&request_params, &JsValue::from_str("params"), &params)
            .map_err(|_| "Failed to set params")?;

        let request_fn = Reflect::get(&ethereum, &JsValue::from_str("request"))?
            .dyn_into::<Function>()
            .map_err(|_| "Request is not a function")?;

        let promise = request_fn.call1(&ethereum, &request_params)
            .map_err(|_| "Failed to call request")?;
        let accounts = JsFuture::from(promise.dyn_into::<Promise>().unwrap())
            .await
            .map_err(|_| "Connection rejected")?;

        let accounts_array = js_sys::Array::from(&accounts);
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
        Err("MetaMask connection not supported on this platform".to_string())
    }
} 
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::JsCast;
use leptos::SignalUpdate;
use js_sys::{Function, Promise, Reflect};
use super::{WalletContext, WalletType, JsValueWrapper};

pub async fn connect_phantom(wallet_context: &WalletContext) -> Result<(), String> {
    #[cfg(target_arch = "wasm32")]
    {
        let window = web_sys::window().ok_or("No window object")?;
        let solana = Reflect::get(&window, &JsValue::from_str("solana"))
            .map_err(|e| String::from(JsValueWrapper::from(e)))?;
        
        let is_phantom = Reflect::get(&solana, &JsValue::from_str("isPhantom"))
            .map_err(|e| String::from(JsValueWrapper::from(e)))?
            .as_bool()
            .unwrap_or(false);
        
        if !is_phantom {
            wallet_context.set_error("Phantom wallet not installed");
            return Err("Phantom wallet not installed".to_string());
        }
        
        let connect_fn = Reflect::get(&solana, &JsValue::from_str("connect"))
            .map_err(|_| "Failed to get connect function")?
            .dyn_into::<Function>()
            .map_err(|_| "Connect is not a function")?;
        
        let promise = connect_fn.call0(&solana)
            .map_err(|_| "Failed to call connect")?;
        
        let result = JsFuture::from(Promise::from(promise))
            .await
            .map_err(|e| format!("Connection rejected: {:?}", e))?;
        
        let public_key = Reflect::get(&result, &JsValue::from_str("publicKey"))
            .map_err(|_| "Failed to get public key")?;
        
        let to_base58_fn = Reflect::get(&public_key, &JsValue::from_str("toBase58"))
            .map_err(|_| "Failed to get toBase58 function")?
            .dyn_into::<Function>()
            .map_err(|_| "toBase58 is not a function")?;
        
        let address_js_value = to_base58_fn.call0(&public_key)
            .map_err(|_| "Failed to call toBase58")?;
        
        let address = address_js_value.as_string()
            .ok_or("Invalid address format")?;
        
        wallet_context.state.update(|state| {
            state.connected = true;
            state.address = Some(address);
            state.wallet_type = Some(WalletType::Phantom);
            state.error = None;
            state.connecting = false;
        });
        
        Ok(())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = wallet_context;
        Err("Phantom wallet connection not supported on this platform".to_string())
    }
}
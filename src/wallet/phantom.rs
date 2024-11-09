use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;
use js_sys::{Function, Promise, Reflect};
use crate::wallet::{WalletContext, WalletType};
use leptos::SignalUpdate;

pub async fn connect_phantom(wallet_context: &WalletContext) -> Result<(), String> {
    #[cfg(target_arch = "wasm32")]
    {
        let window = window().ok_or("No window object")?;
        let solana = Reflect::get(&window, &JsValue::from_str("solana"))
            .map_err(|_| "No solana object in window")?;
        
        let is_phantom = Reflect::get(&solana, &JsValue::from_str("isPhantom"))
            .map_err(|_| "Failed to access isPhantom property")?
            .as_bool()
            .unwrap_or(false);
        
        if !is_phantom {
            wallet_context.set_error("Phantom wallet not installed");
            return Err("Phantom wallet not installed".to_string());
        }
        
        // Request connection
        let connect_fn = Reflect::get(&solana, &JsValue::from_str("connect"))?
            .dyn_into::<Function>()
            .map_err(|_| "Connect is not a function")?;
        
        let promise = connect_fn.call0(&solana)
            .map_err(|_| "Failed to call connect")?;
        
        let result = JsFuture::from(promise.dyn_into::<Promise>().unwrap())
            .await
            .map_err(|_| "Connection rejected")?;
        
        let public_key = Reflect::get(&result, &JsValue::from_str("publicKey"))
            .map_err(|_| "Failed to get public key")?;
        
        let address = Reflect::get(&public_key, &JsValue::from_str("toBase58"))?
            .dyn_into::<Function>()?
            .call0(&public_key)?
            .as_string()
            .ok_or("Invalid address format")?;
        
        wallet_context.set_state.update(|state| {
            state.connected = true;
            state.address = Some(address);
            state.wallet_type = Some(WalletType::Phantom);
            state.error = None;
        });
        Ok(())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        Err("Phantom wallet connection not supported on this platform".to_string())
    }
}
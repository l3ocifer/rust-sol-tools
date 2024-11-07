use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;
use js_sys::{Function, Promise};
use leptos::SignalUpdate;
use super::WalletContext;

pub async fn connect_phantom(wallet_context: &WalletContext) {
    match window() {
        Some(window) => {
            if let Some(solana) = js_sys::Reflect::get(&window, &JsValue::from_str("solana")).ok() {
                if js_sys::Reflect::get(&solana, &JsValue::from_str("isPhantom")).ok().is_some() {
                    match connect_phantom_wallet(solana).await {
                        Ok(address) => {
                            wallet_context.set_state.update(|state| {
                                state.connected = true;
                                state.address = Some(address);
                                state.wallet_type = Some(super::WalletType::Phantom);
                                state.error = None;
                            });
                        }
                        Err(e) => wallet_context.set_error(&format!("Failed to connect: {}", e)),
                    }
                    return;
                }
            }
        }
        None => (),
    }
    wallet_context.set_error("Phantom wallet not found");
}

async fn connect_phantom_wallet(solana: JsValue) -> Result<String, String> {
    let connect_promise = js_sys::Reflect::get(&solana, &JsValue::from_str("connect"))
        .map_err(|_| "No connect method")?;
    
    let connect_fn = connect_promise.dyn_ref::<Function>()
        .ok_or("Connect is not a function")?;
    
    let promise = connect_fn.call0(&solana)
        .map_err(|_| "Failed to call connect")?;
    
    let _result = JsFuture::from(promise.dyn_into::<Promise>().unwrap())
        .await
        .map_err(|_| "Connection rejected")?;
    
    let public_key = js_sys::Reflect::get(&solana, &JsValue::from_str("publicKey"))
        .map_err(|_| "No public key")?;
    
    let address = js_sys::Reflect::get(&public_key, &JsValue::from_str("toString"))
        .map_err(|_| "No toString method")?;
    
    let address_fn = address.dyn_ref::<Function>()
        .ok_or("ToString is not a function")?;
    
    let address_str = address_fn.call0(&public_key)
        .map_err(|_| "Failed to get address string")?;
    
    Ok(address_str.as_string().unwrap_or_default())
} 
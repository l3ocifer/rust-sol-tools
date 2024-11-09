use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Window};
use js_sys::{Function, Promise, Reflect, Object, Array, ArrayBuffer};
use leptos::SignalUpdate;
use super::WalletContext;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct TokenMetadata {
    name: String,
    symbol: String,
    uri: String,
    decimals: u8,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub async fn connect_phantom(wallet_context: &WalletContext) -> Result<(), JsValue> {
    let window = window().ok_or("No window object")?;
    let solana = Reflect::get(&window, &JsValue::from_str("solana"))?;
    
    if !is_phantom_installed(&solana)? {
        wallet_context.set_error("Phantom wallet not installed");
        return Ok(());
    }

    match request_phantom_connection(&solana).await {
        Ok(address) => {
            wallet_context.set_state.update(|state| {
                state.connected = true;
                state.address = Some(address);
                state.wallet_type = Some(super::WalletType::Phantom);
                state.error = None;
            });
            Ok(())
        }
        Err(e) => {
            wallet_context.set_error(&format!("Failed to connect: {}", e.as_string().unwrap_or_default()));
            Ok(())
        }
    }
}

fn is_phantom_installed(solana: &JsValue) -> Result<bool, JsValue> {
    Ok(Reflect::get(solana, &JsValue::from_str("isPhantom"))?.as_bool().unwrap_or(false))
}

async fn request_phantom_connection(solana: &JsValue) -> Result<String, JsValue> {
    let connect_fn = Reflect::get(solana, &JsValue::from_str("connect"))?
        .dyn_into::<Function>()?;
    
    let promise = connect_fn.call0(solana)?
        .dyn_into::<Promise>()?;
    
    let result = JsFuture::from(promise).await?;
    let public_key = Reflect::get(&result, &JsValue::from_str("publicKey"))?;
    
    let address = Reflect::get(&public_key, &JsValue::from_str("toBase58"))?
        .dyn_into::<Function>()?
        .call0(&public_key)?
        .as_string()
        .ok_or_else(|| JsValue::from_str("Invalid address format"))?;

    Ok(address)
} 
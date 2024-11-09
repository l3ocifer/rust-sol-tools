use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;
use js_sys::{Function, Promise, Reflect, Object};
use leptos::SignalUpdate;
use super::WalletContext;

#[wasm_bindgen(module = "/public/phantom.js")]
extern "C" {
    #[wasm_bindgen(js_name = connectPhantomWallet)]
    pub async fn connect_phantom_wallet() -> JsValue;

    #[wasm_bindgen(js_name = createToken)]
    pub async fn create_token(params: JsValue) -> JsValue;
}

pub async fn connect_phantom(wallet_context: &WalletContext) {
    match window() {
        Some(window) => {
            if let Some(solana) = Reflect::get(&window, &JsValue::from_str("solana")).ok() {
                if Reflect::get(&solana, &JsValue::from_str("isPhantom")).ok().is_some() {
                    match request_phantom_connection(&solana).await {
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

async fn request_phantom_connection(solana: &JsValue) -> Result<String, String> {
    let connect_method = JsValue::from_str("connect");
    let request_obj = Object::new();
    Reflect::set(&request_obj, &JsValue::from_str("onlyIfTrusted"), &JsValue::from_bool(false))
        .map_err(|_| "Failed to set request params")?;

    let connect_promise = Reflect::get(solana, &connect_method)
        .map_err(|_| "No connect method")?;
    
    let connect_fn = connect_promise.dyn_ref::<Function>()
        .ok_or("Connect is not a function")?;
    
    let promise = connect_fn.call1(solana, &request_obj)
        .map_err(|_| "Failed to call connect")?;
    
    let result = JsFuture::from(promise.dyn_into::<Promise>().unwrap())
        .await
        .map_err(|_| "Connection rejected")?;
    
    let public_key = Reflect::get(&result, &JsValue::from_str("publicKey"))
        .map_err(|_| "No public key returned")?;
    
    let address = Reflect::get(&public_key, &JsValue::from_str("toBase58"))
        .map_err(|_| "No toBase58 method")?
        .dyn_ref::<Function>()
        .ok_or("toBase58 is not a function")?
        .call0(&public_key)
        .map_err(|_| "Failed to call toBase58")?
        .as_string()
        .ok_or("Invalid address format")?;

    Ok(address)
}

#[wasm_bindgen]
pub async fn create_token_phantom(
    name: String,
    symbol: String,
    uri: String,
    decimals: u8,
) -> Result<JsValue, JsValue> {
    let window = web_sys::window().ok_or("No window object")?;
    let solana = Reflect::get(&window, &JsValue::from_str("solana"))
        .map_err(|_| "No Solana object")?;

    let params = Object::new();
    Reflect::set(&params, &JsValue::from_str("name"), &JsValue::from_str(&name))?;
    Reflect::set(&params, &JsValue::from_str("symbol"), &JsValue::from_str(&symbol))?;
    Reflect::set(&params, &JsValue::from_str("uri"), &JsValue::from_str(&uri))?;
    Reflect::set(&params, &JsValue::from_str("decimals"), &JsValue::from_f64(decimals as f64))?;

    let create_token_fn = Reflect::get(&solana, &JsValue::from_str("createToken"))
        .map_err(|_| "No createToken function")?;
    
    let promise = create_token_fn.dyn_ref::<Function>()
        .ok_or("createToken is not a function")?
        .call1(&solana, &params)
        .map_err(|_| "Failed to call createToken")?;

    JsFuture::from(promise.dyn_into::<Promise>().unwrap()).await
} 
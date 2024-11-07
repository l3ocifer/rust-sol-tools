use wasm_bindgen::prelude::*;
use leptos::SignalUpdate;
use super::WalletContext;

pub async fn connect_metamask(wallet_context: &WalletContext) {
    match web_sys::window() {
        Some(window) => {
            if let Some(ethereum) = js_sys::Reflect::get(&window, &JsValue::from_str("ethereum")).ok() {
                if js_sys::Reflect::get(&ethereum, &JsValue::from_str("isMetaMask")).ok().is_some() {
                    match request_metamask_accounts(ethereum).await {
                        Ok(address) => {
                            wallet_context.set_state.update(|state| {
                                state.connected = true;
                                state.address = Some(address);
                                state.wallet_type = Some(super::WalletType::MetaMask);
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
    wallet_context.set_error("MetaMask wallet not found");
}

async fn request_metamask_accounts(ethereum: JsValue) -> Result<String, String> {
    let request_method = JsValue::from_str("eth_requestAccounts");
    let params = js_sys::Array::new();
    
    let request_obj = js_sys::Object::new();
    js_sys::Reflect::set(&request_obj, &JsValue::from_str("method"), &request_method)
        .map_err(|_| "Failed to set request method")?;
    js_sys::Reflect::set(&request_obj, &JsValue::from_str("params"), &params)
        .map_err(|_| "Failed to set request params")?;

    let request_promise = js_sys::Reflect::get(&ethereum, &JsValue::from_str("request"))
        .map_err(|_| "No request method")?;
    
    let request_fn = request_promise.dyn_ref::<js_sys::Function>()
        .ok_or("Request is not a function")?;
    
    let promise = request_fn.call1(&ethereum, &request_obj)
        .map_err(|_| "Failed to call request")?;
    
    let accounts = wasm_bindgen_futures::JsFuture::from(promise.dyn_into::<js_sys::Promise>().unwrap())
        .await
        .map_err(|_| "Connection rejected")?;
    
    let accounts_array = js_sys::Array::from(&accounts);
    if accounts_array.length() > 0 {
        let address = accounts_array.get(0).as_string()
            .ok_or("Invalid address format")?;
        Ok(address)
    } else {
        Err("No accounts returned".to_string())
    }
} 
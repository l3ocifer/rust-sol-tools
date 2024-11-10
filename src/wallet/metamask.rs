use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;
use js_sys::{Function, Promise, Object, Reflect, Array};
use leptos::SignalUpdate;
use super::{WalletContext, WalletType, TokenBalance, JsValueWrapper};

pub async fn connect_metamask(ctx: &WalletContext) -> Result<(), String> {
    let window = window().ok_or("No window object found")?;
    let ethereum = Reflect::get(&window, &JsValue::from_str("ethereum"))
        .map_err(|e| JsValueWrapper::from(e).into::<String>())?;

    if ethereum.is_undefined() {
        return Err("MetaMask not installed".to_string());
    }

    ctx.state.update(|state| {
        state.connecting = true;
        state.error = None;
    });

    let ethereum_obj = ethereum.dyn_into::<Object>()
        .map_err(|e| JsValueWrapper::from(e).into::<String>())?;

    let request_method = Reflect::get(&ethereum_obj, &JsValue::from_str("request"))
        .map_err(|e| JsValueWrapper::from(e).into::<String>())?
        .dyn_into::<Function>()
        .map_err(|e| JsValueWrapper::from(e).into::<String>())?;

    let params = Array::new();
    let request_obj = Object::new();
    Reflect::set(&request_obj, &JsValue::from_str("method"), &JsValue::from_str("eth_requestAccounts"))
        .map_err(|_| "Failed to set request method")?;
    Reflect::set(&request_obj, &JsValue::from_str("params"), &params)
        .map_err(|_| "Failed to set request params")?;

    let promise = request_method.call1(&ethereum, &request_obj)
        .map_err(|_| "Failed to call request method")?;

    let accounts = JsFuture::from(Promise::from(promise))
        .await
        .map_err(|_| "User rejected connection")?;

    let accounts_array = Array::from(&accounts);
    if accounts_array.length() == 0 {
        return Err("No accounts found".to_string());
    }

    let address = accounts_array.get(0).as_string()
        .ok_or("Invalid address format")?;

    ctx.state.update(|state| {
        state.connected = true;
        state.address = Some(address);
        state.wallet_type = Some(WalletType::MetaMask);
        state.connecting = false;
    });

    Ok(())
}

pub async fn get_token_balances(_address: &str) -> Result<Vec<TokenBalance>, String> {
    // Implementation for token balances
    Ok(Vec::new())
} 
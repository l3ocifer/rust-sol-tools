use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;
use js_sys::{Function, Promise, Object, Reflect, Array};
use leptos::SignalUpdate;
use super::{WalletContext, WalletType, TokenBalance, JsValueWrapper};

pub async fn connect_metamask(ctx: &WalletContext) -> Result<(), String> {
    let window = window().ok_or("No window object found")?;
    let ethereum = Reflect::get(&window, &JsValue::from_str("ethereum"))
        .map_err(|e| JsValueWrapper(e).into())?;

    if ethereum.is_undefined() {
        return Err("MetaMask not installed".to_string());
    }

    ctx.state.update(|state| {
        state.connecting = true;
        state.error = None;
    });

    let ethereum_clone = ethereum.clone();
    let ethereum_obj = ethereum.dyn_into::<Object>()
        .map_err(|e| JsValueWrapper(e).into())?;

    let request_method = Reflect::get(&ethereum_obj, &JsValue::from_str("request"))
        .map_err(|e| JsValueWrapper(e).into())?
        .dyn_into::<Function>()
        .map_err(|e| JsValueWrapper(e).into())?;

    let params = Array::new();
    let request_obj = Object::new();
    Reflect::set(&request_obj, &JsValue::from_str("method"), &JsValue::from_str("eth_requestAccounts"))
        .map_err(|_| "Failed to set request method")?;
    Reflect::set(&request_obj, &JsValue::from_str("params"), &params)
        .map_err(|_| "Failed to set request params")?;

    let promise = request_method.call1(&ethereum_clone, &request_obj)
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

pub async fn get_token_balances(address: &str) -> Result<Vec<TokenBalance>, String> {
    let window = window().ok_or("No window object found")?;
    let ethereum = Reflect::get(&window, &JsValue::from_str("ethereum"))
        .map_err(|_| "MetaMask not installed")?;

    let request_method = Reflect::get(&ethereum, &JsValue::from_str("request"))
        .map_err(|_| "ethereum.request not found")?
        .dyn_into::<Function>()
        .map_err(|_| "Failed to cast request method")?;

    // Get token list from user's wallet
    let params = Array::new();
    let request_obj = Object::new();
    Reflect::set(&request_obj, &JsValue::from_str("method"), &JsValue::from_str("wallet_getTokens"))
        .map_err(|_| "Failed to set request method")?;
    Reflect::set(&request_obj, &JsValue::from_str("params"), &params)
        .map_err(|_| "Failed to set request params")?;

    let promise = request_method.call1(&ethereum, &request_obj)
        .map_err(|_| "Failed to call request method")?;

    let tokens = JsFuture::from(Promise::from(promise))
        .await
        .map_err(|_| "Failed to get tokens")?;

    let tokens_array = Array::from(&tokens);
    let mut token_balances = Vec::new();

    for i in 0..tokens_array.length() {
        if let Some(token) = tokens_array.get(i).dyn_ref::<Object>() {
            let address = Reflect::get(&token, &JsValue::from_str("address"))
                .map_err(|_| "Failed to get token address")?
                .as_string()
                .ok_or("Invalid token address")?;

            let symbol = Reflect::get(&token, &JsValue::from_str("symbol"))
                .map_err(|_| "Failed to get token symbol")?
                .as_string();

            let name = Reflect::get(&token, &JsValue::from_str("name"))
                .map_err(|_| "Failed to get token name")?
                .as_string();

            let decimals = Reflect::get(&token, &JsValue::from_str("decimals"))
                .map_err(|_| "Failed to get token decimals")?
                .as_f64()
                .ok_or("Invalid decimals")? as u8;

            let balance = Reflect::get(&token, &JsValue::from_str("balance"))
                .map_err(|_| "Failed to get token balance")?
                .as_string()
                .ok_or("Invalid balance")?;

            let balance_value = u128::from_str_radix(&balance[2..], 16)
                .map_err(|_| "Invalid balance format")?;

            token_balances.push(TokenBalance {
                mint: address,
                amount: balance_value as f64 / 10f64.powi(decimals as i32),
                decimals,
                symbol,
                name,
            });
        }
    }

    Ok(token_balances)
} 
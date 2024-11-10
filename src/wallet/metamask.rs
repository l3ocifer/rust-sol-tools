use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;
use js_sys::{Function, Promise, Object, Reflect, Array};
use leptos::SignalUpdate;
use super::{WalletContext, WalletType, TokenBalance, JsValueWrapper};

pub async fn connect_metamask(ctx: &WalletContext) -> Result<(), String> {
    let window = window().ok_or("No window object found")?;
    let ethereum = Reflect::get(&window, &JsValue::from_str("ethereum"))
        .map_err(|e| String::from(JsValueWrapper::from(e)))?;

    if ethereum.is_undefined() {
        return Err("MetaMask not installed".to_string());
    }

    ctx.state.update(|state| {
        state.connecting = true;
        state.error = None;
    });

    let ethereum_clone = ethereum.clone();
    let ethereum_obj = ethereum.dyn_into::<Object>()
        .map_err(|e| String::from(JsValueWrapper::from(e)))?;

    let request_method = Reflect::get(&ethereum_obj, &JsValue::from_str("request"))
        .map_err(|e| String::from(JsValueWrapper::from(e)))?
        .dyn_into::<Function>()
        .map_err(|e| String::from(JsValueWrapper::from(e)))?;

    let params = Array::new();
    params.push(&JsValue::from_str("eth_requestAccounts"));

    let request_obj = Object::new();
    Reflect::set(&request_obj, &JsValue::from_str("method"), &JsValue::from_str("eth_requestAccounts"))
        .map_err(|e| String::from(JsValueWrapper::from(e)))?;

    let promise = request_method.call1(&ethereum_clone, &request_obj)
        .map_err(|e| String::from(JsValueWrapper::from(e)))?;

    let accounts = JsFuture::from(Promise::from(promise))
        .await
        .map_err(|e| String::from(JsValueWrapper::from(e)))?;

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
        state.error = None;
        state.connecting = false;
    });

    Ok(())
}

pub async fn get_token_balances(address: &str) -> Result<Vec<TokenBalance>, String> {
    let window = window().ok_or("No window object found")?;
    let ethereum = Reflect::get(&window, &JsValue::from_str("ethereum"))
        .map_err(|e| String::from(JsValueWrapper::from(e)))?;

    if ethereum.is_undefined() {
        return Err("MetaMask not installed".to_string());
    }

    let ethereum_obj = ethereum.dyn_into::<Object>()
        .map_err(|e| String::from(JsValueWrapper::from(e)))?;

    // Get ERC20 token balances using eth_call
    let tokens = get_known_tokens();
    let mut token_balances = Vec::new();

    for token in tokens {
        let balance = get_erc20_balance(&ethereum_obj, address, &token.contract_address).await?;
        if balance > 0.0 {
            token_balances.push(TokenBalance {
                mint: token.contract_address,
                amount: balance,
                decimals: token.decimals,
                symbol: Some(token.symbol),
                name: Some(token.name),
            });
        }
    }

    Ok(token_balances)
}

async fn get_erc20_balance(ethereum: &Object, address: &str, token_address: &str) -> Result<f64, String> {
    let function_signature = "70a08231"; // balanceOf(address)
    let padded_address = format!("{:0>64}", &address[2..]);
    let data = format!("0x{}{}}", function_signature, padded_address);

    let params = Array::new();
    let call_obj = Object::new();
    
    Reflect::set(&call_obj, &JsValue::from_str("to"), &JsValue::from_str(token_address))
        .map_err(|e| String::from(JsValueWrapper::from(e)))?;
    Reflect::set(&call_obj, &JsValue::from_str("data"), &JsValue::from_str(&data))
        .map_err(|e| String::from(JsValueWrapper::from(e)))?;
    
    params.push(&call_obj);
    params.push(&JsValue::from_str("latest"));

    let request = Object::new();
    Reflect::set(&request, &JsValue::from_str("method"), &JsValue::from_str("eth_call"))
        .map_err(|e| String::from(JsValueWrapper::from(e)))?;
    Reflect::set(&request, &JsValue::from_str("params"), &params)
        .map_err(|e| String::from(JsValueWrapper::from(e)))?;

    let request_fn = Reflect::get(ethereum, &JsValue::from_str("request"))
        .map_err(|e| String::from(JsValueWrapper::from(e)))?
        .dyn_into::<Function>()
        .map_err(|e| String::from(JsValueWrapper::from(e)))?;

    let promise = request_fn.call1(ethereum, &request)
        .map_err(|e| String::from(JsValueWrapper::from(e)))?;

    let result = JsFuture::from(Promise::from(promise))
        .await
        .map_err(|e| String::from(JsValueWrapper::from(e)))?;

    let balance_hex = result.as_string()
        .ok_or("Invalid balance format")?;
    
    let balance = u128::from_str_radix(&balance_hex[2..], 16)
        .map_err(|e| e.to_string())?;

    Ok(balance as f64 / 1e18) // Assuming 18 decimals for ERC20
}

fn get_known_tokens() -> Vec<TokenInfo> {
    vec![
        TokenInfo {
            contract_address: "0x...".to_string(), // Add known token addresses
            symbol: "TOKEN".to_string(),
            name: "Token Name".to_string(),
            decimals: 18,
        }
    ]
}

struct TokenInfo {
    contract_address: String,
    symbol: String,
    name: String,
    decimals: u8,
} 
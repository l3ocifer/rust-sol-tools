#[cfg(target_arch = "wasm32")]
mod phantom;
#[cfg(target_arch = "wasm32")]
mod metamask;

use leptos::*;
use serde::{Deserialize, Serialize};
use leptos::SignalUpdate;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::{Array, Object, Promise, Reflect, Function};
use web_sys::window;

#[cfg(target_arch = "wasm32")]
pub use phantom::connect_phantom;
#[cfg(target_arch = "wasm32")]
pub use metamask::connect_metamask;

#[derive(Clone)]
pub struct JsValueWrapper(JsValue);

impl From<JsValueWrapper> for String {
    fn from(wrapper: JsValueWrapper) -> Self {
        wrapper.0.as_string().unwrap_or_else(|| format!("{:?}", wrapper.0))
    }
}

impl From<JsValue> for JsValueWrapper {
    fn from(value: JsValue) -> Self {
        JsValueWrapper(value)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenBalance {
    pub mint: String,
    pub amount: f64,
    pub decimals: u8,
    pub symbol: Option<String>,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WalletState {
    pub connected: bool,
    pub address: Option<String>,
    pub wallet_type: Option<WalletType>,
    pub error: Option<String>,
    pub connecting: bool,
    pub sol_balance: f64,
    pub token_balances: Vec<TokenBalance>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum WalletType {
    Phantom,
    MetaMask,
}

#[derive(Clone)]
pub struct WalletContext {
    pub state: RwSignal<WalletState>,
}

impl WalletContext {
    pub fn new(state: RwSignal<WalletState>) -> Self {
        Self { state }
    }

    pub fn set_error(&self, error: &str) {
        self.state.update(|state| {
            state.error = Some(error.to_string());
        });
    }

    pub async fn get_balance(&self) -> Result<f64, String> {
        if let Some(address) = self.state.get().address {
            match self.state.get().wallet_type {
                Some(WalletType::Phantom) => {
                    let window = window().ok_or("No window object")?;
                    let solana = Reflect::get(&window, &JsValue::from_str("solana"))
                        .map_err(|e| JsValueWrapper::from(e).into())?;
                    
                    let connection = Reflect::get(&solana, &JsValue::from_str("connection"))
                        .map_err(|e| JsValueWrapper::from(e).into())?;
                    
                    let get_balance = Reflect::get(&connection, &JsValue::from_str("getBalance"))
                        .map_err(|e| JsValueWrapper::from(e).into())?
                        .dyn_into::<Function>()
                        .map_err(|e| JsValueWrapper::from(e).into())?;
                    
                    let public_key = Reflect::get(&solana, &JsValue::from_str("publicKey"))
                        .map_err(|e| JsValueWrapper::from(e).into())?;
                    
                    let promise = get_balance.call1(&connection, &public_key)
                        .map_err(|e| JsValueWrapper::from(e).into())?;
                    
                    let balance = JsFuture::from(Promise::from(promise))
                        .await
                        .map_err(|e| JsValueWrapper::from(e).into())?;
                    
                    let balance_number = balance.as_f64().ok_or("Invalid balance format")?;
                    Ok(balance_number / 1e9) // Convert lamports to SOL
                }
                Some(WalletType::MetaMask) => {
                    let window = window().ok_or("No window object")?;
                    let ethereum = Reflect::get(&window, &JsValue::from_str("ethereum"))
                        .map_err(|e| JsValueWrapper::from(e).into())?;
                    
                    let params = Array::new();
                    params.push(&JsValue::from_str(&address));
                    params.push(&JsValue::from_str("latest"));
                    
                    let request = Object::new();
                    Reflect::set(&request, &JsValue::from_str("method"), &JsValue::from_str("eth_getBalance"))
                        .map_err(|e| e.to_string())?;
                    Reflect::set(&request, &JsValue::from_str("params"), &params)
                        .map_err(|e| e.to_string())?;
                    
                    let request_fn = Reflect::get(&ethereum, &JsValue::from_str("request"))
                        .map_err(|e| e.to_string())?
                        .dyn_into::<js_sys::Function>()
                        .map_err(|e| e.to_string())?;
                    
                    let promise = request_fn.call1(&ethereum, &request)
                        .map_err(|e| e.to_string())?;
                    let balance = JsFuture::from(Promise::from(promise))
                        .await
                        .map_err(|e| e.to_string())?;
                    
                    let hex_balance = balance.as_string().ok_or("Invalid balance format")?;
                    let wei = u128::from_str_radix(&hex_balance[2..], 16).map_err(|_| "Invalid hex balance")?;
                    Ok(wei as f64 / 1e18) // Convert wei to ETH
                }
                None => Err("No wallet connected".to_string()),
            }
        } else {
            Err("Wallet not connected".to_string())
        }
    }

    pub async fn get_token_balances(&self) -> Result<Vec<TokenBalance>, String> {
        if let Some(address) = self.state.get().address {
            match self.state.get().wallet_type {
                Some(WalletType::Phantom) => {
                    let window = web_sys::window().ok_or("No window object")?;
                    let solana = js_sys::Reflect::get(&window, &JsValue::from_str("solana"))
                        .map_err(|_| "No solana object")?;
                    
                    let connection = js_sys::Reflect::get(&solana, &JsValue::from_str("connection"))
                        .map_err(|_| "No connection object")?;
                    
                    let get_token_accounts = js_sys::Reflect::get(&connection, &JsValue::from_str("getTokenAccountsByOwner"))
                        .map_err(|_| "No getTokenAccountsByOwner method")?
                        .dyn_into::<js_sys::Function>()
                        .map_err(|_| "getTokenAccountsByOwner is not a function")?;
                    
                    let public_key = js_sys::Reflect::get(&solana, &JsValue::from_str("publicKey"))
                        .map_err(|_| "No publicKey")?;
                    
                    let filter_obj = Object::new();
                    Reflect::set(&filter_obj, &JsValue::from_str("programId"), &JsValue::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"))?;
                    
                    let promise = get_token_accounts.call2(&connection, &public_key, &filter_obj)
                        .map_err(|_| "Failed to call getTokenAccountsByOwner")?;
                    
                    let accounts = JsFuture::from(Promise::from(promise))
                        .await
                        .map_err(|_| "Failed to get token accounts")?;
                    
                    let accounts_array = Array::from(&accounts);
                    let mut token_balances = Vec::new();
                    
                    for i in 0..accounts_array.length() {
                        if let Some(account) = accounts_array.get(i).dyn_ref::<Object>() {
                            let account_info = Reflect::get(&account, &JsValue::from_str("account"))?;
                            let data = Reflect::get(&account_info, &JsValue::from_str("data"))?;
                            
                            let mint = Reflect::get(&data, &JsValue::from_str("mint"))?
                                .as_string()
                                .ok_or("Invalid mint address")?;
                            
                            let amount = Reflect::get(&data, &JsValue::from_str("amount"))?
                                .as_f64()
                                .ok_or("Invalid token amount")?;
                            
                            let decimals = Reflect::get(&data, &JsValue::from_str("decimals"))?
                                .as_f64()
                                .ok_or("Invalid decimals")? as u8;
                            
                            token_balances.push(TokenBalance {
                                mint,
                                amount: amount / 10f64.powi(decimals as i32),
                                decimals,
                                symbol: None,
                                name: None,
                            });
                        }
                    }
                    
                    Ok(token_balances)
                }
                Some(WalletType::MetaMask) => {
                    // MetaMask token balance implementation would go here
                    Ok(Vec::new())
                }
                None => Err("No wallet connected".to_string()),
            }
        } else {
            Err("Wallet not connected".to_string())
        }
    }

    pub fn disconnect(&self) {
        self.state.update(|state| {
            state.connected = false;
            state.address = None;
            state.wallet_type = None;
            state.error = None;
            state.connecting = false;
            state.sol_balance = 0.0;
            state.token_balances = Vec::new();
        });
    }

    pub async fn connect(&self, wallet_type: WalletType) -> Result<(), String> {
        match wallet_type {
            WalletType::Phantom => connect_phantom(self).await,
            WalletType::MetaMask => connect_metamask(self).await,
        }
    }
}

#[component]
pub fn WalletProvider(children: Children) -> impl IntoView {
    let state = create_rw_signal(WalletState {
        connected: false,
        address: None,
        wallet_type: None,
        error: None,
        connecting: false,
        sol_balance: 0.0,
        token_balances: Vec::new(),
    });

    provide_context(WalletContext::new(state));
    children()
} 
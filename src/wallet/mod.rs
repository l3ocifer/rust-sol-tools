#[cfg(target_arch = "wasm32")]
mod phantom;
#[cfg(target_arch = "wasm32")]
mod metamask;

use leptos::*;
use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
use {
    wasm_bindgen::prelude::*,
    wasm_bindgen_futures::JsFuture,
    js_sys::{Array, Object, Promise, Reflect, Function},
    web_sys::window,
};

#[cfg(target_arch = "wasm32")]
use self::{
    phantom::connect_phantom,
    metamask::connect_metamask,
};

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

#[cfg(target_arch = "wasm32")]
#[derive(Clone)]
pub struct JsValueWrapper(JsValue);

#[cfg(target_arch = "wasm32")]
impl From<JsValueWrapper> for String {
    fn from(wrapper: JsValueWrapper) -> Self {
        wrapper.0.as_string().unwrap_or_else(|| format!("{:?}", wrapper.0))
    }
}

#[cfg(target_arch = "wasm32")]
impl From<JsValue> for JsValueWrapper {
    fn from(value: JsValue) -> Self {
        JsValueWrapper(value)
    }
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

    #[cfg(target_arch = "wasm32")]
    pub async fn get_balance(&self) -> Result<f64, String> {
        if let Some(address) = self.state.get().address {
            match self.state.get().wallet_type {
                Some(WalletType::Phantom) => {
                    let window = window().ok_or("No window object")?;
                    let solana = Reflect::get(&window, &JsValue::from_str("solana"))
                        .map_err(|e| String::from(JsValueWrapper::from(e)))?;
                    
                    let connection = Reflect::get(&solana, &JsValue::from_str("connection"))
                        .map_err(|e| String::from(JsValueWrapper::from(e)))?;
                    
                    let get_balance = Reflect::get(&connection, &JsValue::from_str("getBalance"))
                        .map_err(|e| String::from(JsValueWrapper::from(e)))?
                        .dyn_into::<Function>()
                        .map_err(|e| String::from(JsValueWrapper::from(e)))?;
                    
                    let public_key = Reflect::get(&solana, &JsValue::from_str("publicKey"))
                        .map_err(|e| String::from(JsValueWrapper::from(e)))?;
                    
                    let promise = get_balance.call1(&connection, &public_key)
                        .map_err(|e| String::from(JsValueWrapper::from(e)))?;
                    
                    let balance = JsFuture::from(Promise::from(promise))
                        .await
                        .map_err(|e| String::from(JsValueWrapper::from(e)))?;
                    
                    let balance_number = balance.as_f64().ok_or("Invalid balance format")?;
                    Ok(balance_number / 1e9) // Convert lamports to SOL
                }
                Some(WalletType::MetaMask) => {
                    let window = window().ok_or("No window object")?;
                    let ethereum = Reflect::get(&window, &JsValue::from_str("ethereum"))
                        .map_err(|e| String::from(JsValueWrapper::from(e)))?;
                    
                    let params = Array::new();
                    params.push(&JsValue::from_str(&address));
                    params.push(&JsValue::from_str("latest"));
                    
                    let request = Object::new();
                    Reflect::set(&request, &JsValue::from_str("method"), &JsValue::from_str("eth_getBalance"))
                        .map_err(|e| String::from(JsValueWrapper::from(e)))?;
                    Reflect::set(&request, &JsValue::from_str("params"), &params)
                        .map_err(|e| String::from(JsValueWrapper::from(e)))?;
                    
                    let request_fn = Reflect::get(&ethereum, &JsValue::from_str("request"))
                        .map_err(|e| String::from(JsValueWrapper::from(e)))?
                        .dyn_into::<js_sys::Function>()
                        .map_err(|e| String::from(JsValueWrapper::from(e)))?;
                    
                    let promise = request_fn.call1(&ethereum, &request)
                        .map_err(|e| String::from(JsValueWrapper::from(e)))?;
                    let balance = JsFuture::from(Promise::from(promise))
                        .await
                        .map_err(|e| String::from(JsValueWrapper::from(e)))?;
                    
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

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_balance(&self) -> Result<f64, String> {
        Err("Wallet operations not supported in server environment".to_string())
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_token_balances(&self) -> Result<Vec<TokenBalance>, String> {
        if self.state.get().connected {
            match self.state.get().wallet_type {
                Some(WalletType::Phantom) => {
                    let window = window().ok_or("No window object found")?;
                    let solana = Reflect::get(&window, &JsValue::from_str("solana"))
                        .map_err(|e| String::from(JsValueWrapper::from(e)))?;

                    let get_token_accounts = Reflect::get(&solana, &JsValue::from_str("getTokenAccounts"))
                        .map_err(|e| String::from(JsValueWrapper::from(e)))?;

                    let promise = Reflect::apply(
                        &get_token_accounts.dyn_into::<Function>().map_err(|e| String::from(JsValueWrapper::from(e)))?,
                        &solana,
                        &Array::new(),
                    ).map_err(|e| String::from(JsValueWrapper::from(e)))?;

                    let result = JsFuture::from(Promise::from(promise))
                        .await
                        .map_err(|e| String::from(JsValueWrapper::from(e)))?;

                    let accounts = Reflect::get(&result, &JsValue::from_str("value"))
                        .map_err(|e| String::from(JsValueWrapper::from(e)))?;

                    let mut token_balances = Vec::new();
                    let accounts_array = Array::from(&accounts);

                    for i in 0..accounts_array.length() {
                        if let Some(account) = accounts_array.get(i).dyn_into::<Object>().ok() {
                            let account_data = Reflect::get(&account, &JsValue::from_str("account"))
                                .map_err(|e| String::from(JsValueWrapper::from(e)))?;
                            let data = Reflect::get(&account_data, &JsValue::from_str("data"))
                                .map_err(|e| String::from(JsValueWrapper::from(e)))?;
                            let mint = Reflect::get(&data, &JsValue::from_str("mint"))
                                .map_err(|e| String::from(JsValueWrapper::from(e)))?
                                .as_string()
                                .ok_or("Invalid mint")?;
                            let amount = Reflect::get(&data, &JsValue::from_str("amount"))
                                .map_err(|e| String::from(JsValueWrapper::from(e)))?
                                .as_f64()
                                .ok_or("Invalid amount")?;
                            let decimals = Reflect::get(&data, &JsValue::from_str("decimals"))
                                .map_err(|e| String::from(JsValueWrapper::from(e)))?
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
                Some(WalletType::MetaMask) => Ok(Vec::new()),
                None => Err("No wallet connected".to_string()),
            }
        } else {
            Err("Wallet not connected".to_string())
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_token_balances(&self) -> Result<Vec<TokenBalance>, String> {
        Err("Wallet operations not supported in server environment".to_string())
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

    #[cfg(target_arch = "wasm32")]
    pub async fn connect(&self, wallet_type: WalletType) -> Result<(), String> {
        match wallet_type {
            WalletType::Phantom => connect_phantom(self).await,
            WalletType::MetaMask => connect_metamask(self).await,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn connect(&self, _wallet_type: WalletType) -> Result<(), String> {
        Err("Wallet operations not supported in server environment".to_string())
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
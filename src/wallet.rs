use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use gloo_utils::format::JsValueSerdeExt;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WalletState {
    pub connected: bool,
    pub address: Option<String>,
    pub wallet_type: Option<WalletType>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum WalletType {
    Phantom,
    MetaMask,
}

#[derive(Clone)]
pub struct WalletContext {
    pub state: ReadSignal<WalletState>,
    pub set_state: WriteSignal<WalletState>,
}

impl WalletContext {
    fn load_stored_state() -> Option<WalletState> {
        if let Some(window) = web_sys::window() {
            if let Some(storage) = window.local_storage().ok()? {
                if let Some(data) = storage.get_item("wallet_state").ok()? {
                    return serde_json::from_str(&data).ok();
                }
            }
        }
        None
    }

    fn save_state(&self, state: &WalletState) {
        if let Some(window) = web_sys::window() {
            if let Some(storage) = window.local_storage().ok() {
                if let Ok(data) = serde_json::to_string(state) {
                    let _ = storage.set_item("wallet_state", &data);
                }
            }
        }
    }

    pub fn connect(&self, wallet_type: WalletType) {
        let window = web_sys::window().unwrap();
        let set_state = self.set_state;
        let this = self.clone();
        
        match wallet_type {
            WalletType::Phantom => {
                if let Some(solana) = window.get("solana") {
                    wasm_bindgen_futures::spawn_local(async move {
                        let connect_result = js_sys::Reflect::get(&solana, &"connect".into())
                            .unwrap()
                            .dyn_into::<js_sys::Function>()
                            .unwrap()
                            .call0(&solana);
                            
                        if let Ok(_) = connect_result {
                            let new_state = WalletState {
                                connected: true,
                                wallet_type: Some(WalletType::Phantom),
                                address: js_sys::Reflect::get(&solana, &"publicKey".into())
                                    .ok()
                                    .and_then(|key| key.as_string()),
                            };
                            set_state.set(new_state.clone());
                            this.save_state(&new_state);
                        }
                    });
                }
            },
            WalletType::MetaMask => {
                if let Some(ethereum) = window.get("ethereum") {
                    wasm_bindgen_futures::spawn_local(async move {
                        let request_value = js_sys::Object::new();
                        js_sys::Reflect::set(
                            &request_value,
                            &"method".into(),
                            &"eth_requestAccounts".into(),
                        ).unwrap();

                        let accounts = js_sys::Reflect::get(&ethereum, &"request".into())
                            .unwrap()
                            .dyn_into::<js_sys::Function>()
                            .unwrap()
                            .call1(&ethereum, &request_value);
                            
                        if let Ok(accounts) = accounts {
                            if let Some(first_account) = js_sys::Reflect::get(&accounts, &0.into()).ok() {
                                let new_state = WalletState {
                                    connected: true,
                                    wallet_type: Some(WalletType::MetaMask),
                                    address: Some(first_account.as_string().unwrap()),
                                };
                                set_state.set(new_state.clone());
                                self.save_state(&new_state);
                            }
                        }
                    });
                }
            }
        }
    }

    pub fn disconnect(&self) {
        self.set_state.update(|state| {
            state.connected = false;
            state.address = None;
            state.wallet_type = None;
        });
        if let Some(window) = web_sys::window() {
            if let Some(storage) = window.local_storage().ok() {
                let _ = storage.remove_item("wallet_state");
            }
        }
    }
}

#[component]
pub fn WalletProvider(children: Children) -> impl IntoView {
    let initial_state = WalletContext::load_stored_state().unwrap_or(WalletState {
        connected: false,
        address: None,
        wallet_type: None,
    });
    
    let (state, set_state) = create_signal(initial_state);
    
    provide_context(
        WalletContext {
            state,
            set_state,
        }
    );
    
    view! { {children()} }
} 
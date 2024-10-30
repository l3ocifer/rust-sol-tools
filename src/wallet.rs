use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

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
    state: ReadSignal<WalletState>,
    set_state: WriteSignal<WalletState>,
}

impl WalletContext {
    pub fn connect(&self, wallet_type: WalletType) {
        let window = web_sys::window().unwrap();
        let set_state = self.set_state;
        
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
                            set_state.update(|state| {
                                state.connected = true;
                                state.wallet_type = Some(WalletType::Phantom);
                                if let Some(public_key) = js_sys::Reflect::get(&solana, &"publicKey".into()).ok() {
                                    state.address = Some(public_key.as_string().unwrap());
                                }
                            });
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
                            set_state.update(|state| {
                                state.connected = true;
                                state.wallet_type = Some(WalletType::MetaMask);
                                if let Some(account) = js_sys::Reflect::get(&accounts, &0.into()).ok() {
                                    state.address = Some(account.as_string().unwrap());
                                }
                            });
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
    }
}

#[component]
pub fn WalletProvider(children: Children) -> impl IntoView {
    let initial_state = WalletState {
        connected: false,
        address: None,
        wallet_type: None,
    };
    
    let (state, set_state) = create_signal(initial_state);
    
    provide_context(
        WalletContext {
            state,
            set_state,
        }
    );
    
    view! { {children()} }
} 
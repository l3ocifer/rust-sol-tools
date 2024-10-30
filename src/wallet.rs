use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::CustomEvent;

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
                            // Update wallet state
                            self.set_state.update(|state| {
                                state.connected = true;
                                state.wallet_type = Some(WalletType::Phantom);
                                // Get public key
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
                        let accounts = js_sys::Reflect::get(&ethereum, &"request".into())
                            .unwrap()
                            .dyn_into::<js_sys::Function>()
                            .unwrap()
                            .call1(
                                &ethereum,
                                &JsValue::from_serde(&serde_json::json!({
                                    "method": "eth_requestAccounts"
                                })).unwrap(),
                            );
                            
                        if let Ok(accounts) = accounts {
                            // Update wallet state
                            self.set_state.update(|state| {
                                state.connected = true;
                                state.wallet_type = Some(WalletType::MetaMask);
                                state.address = Some(accounts[0].as_string().unwrap());
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
pub fn WalletProvider(cx: Scope, children: Children) -> impl IntoView {
    let initial_state = WalletState {
        connected: false,
        address: None,
        wallet_type: None,
    };
    
    let (state, set_state) = create_signal(cx, initial_state);
    
    provide_context(
        cx,
        WalletContext {
            state,
            set_state,
        }
    );
    
    view! { cx, {children()} }
} 
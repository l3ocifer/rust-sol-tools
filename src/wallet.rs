use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;

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
        #[cfg(target_arch = "wasm32")]
        {
            web_sys::window()?.local_storage().ok()?
                .and_then(|storage| storage.get_item("wallet_state").ok()?)
                .and_then(|data| serde_json::from_str(&data).ok())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            None
        }
    }

    pub async fn connect(&self, _wallet_type: WalletType) {
        #[cfg(target_arch = "wasm32")]
        {
            let window = web_sys::window().unwrap();
            let set_state = self.set_state;
            
            match _wallet_type {
                WalletType::Phantom => {
                    if let Some(solana) = window.get("solana") {
                        let connect_result = js_sys::Reflect::get(&solana, &"connect".into())
                            .ok()
                            .and_then(|connect| connect.dyn_into::<js_sys::Function>().ok())
                            .and_then(|func| func.call0(&solana).ok());
                            
                        if connect_result.is_some() {
                            let new_state = WalletState {
                                connected: true,
                                wallet_type: Some(WalletType::Phantom),
                                address: js_sys::Reflect::get(&solana, &"publicKey".into())
                                    .ok()
                                    .and_then(|key| key.as_string()),
                            };
                            set_state.set(new_state);
                        }
                    }
                },
                WalletType::MetaMask => {
                    if let Some(ethereum) = window.get("ethereum") {
                        let request_value = js_sys::Object::new();
                        let _ = js_sys::Reflect::set(
                            &request_value,
                            &"method".into(),
                            &"eth_requestAccounts".into(),
                        );

                        if let Some(request) = js_sys::Reflect::get(&ethereum, &"request".into())
                            .ok()
                            .and_then(|req| req.dyn_into::<js_sys::Function>().ok())
                            .and_then(|func| func.call1(&ethereum, &request_value).ok()) {
                            
                            if let Some(first_account) = js_sys::Reflect::get(&request, &0.into()).ok() {
                                let new_state = WalletState {
                                    connected: true,
                                    wallet_type: Some(WalletType::MetaMask),
                                    address: first_account.as_string(),
                                };
                                set_state.set(new_state);
                            }
                        }
                    }
                }
            }
        }
    }

    pub async fn disconnect(&self) {
        self.set_state.update(|state| {
            state.connected = false;
            state.address = None;
            state.wallet_type = None;
        });
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Some(storage) = window.local_storage().ok().flatten() {
                    let _ = storage.remove_item("wallet_state");
                }
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
    
    provide_context(WalletContext {
        state,
        set_state,
    });
    
    children()
} 
#[cfg(target_arch = "wasm32")]
mod phantom;
#[cfg(target_arch = "wasm32")]
mod metamask;

use leptos::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
pub use phantom::connect_phantom;
#[cfg(target_arch = "wasm32")]
pub use metamask::connect_metamask;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WalletState {
    pub connected: bool,
    pub address: Option<String>,
    pub wallet_type: Option<WalletType>,
    pub error: Option<String>,
    pub connecting: bool,
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

    pub fn disconnect(&self) {
        self.state.update(|state| {
            state.connected = false;
            state.address = None;
            state.wallet_type = None;
            state.error = None;
            state.connecting = false;
        });
    }

    pub async fn connect(&self, wallet_type: WalletType) -> Result<(), String> {
        self.state.update(|state| {
            state.connecting = true;
            state.error = None;
        });

        let result = match wallet_type {
            WalletType::Phantom => connect_phantom(self).await,
            WalletType::MetaMask => connect_metamask(self).await,
        };

        if let Err(e) = &result {
            self.state.update(|state| {
                state.error = Some(e.clone());
                state.connecting = false;
            });
        }

        result
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
    });

    provide_context(WalletContext::new(state));
    children()
} 
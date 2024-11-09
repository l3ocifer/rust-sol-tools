#[cfg(target_arch = "wasm32")]
mod phantom;
#[cfg(target_arch = "wasm32")]
mod metamask;

use leptos::*;
use serde::{Deserialize, Serialize};

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
    pub state: ReadSignal<WalletState>,
    pub(crate) set_state: WriteSignal<WalletState>,
}

impl WalletContext {
    #[cfg(target_arch = "wasm32")]
    pub async fn connect(&self, wallet_type: WalletType) -> Result<(), String> {
        match wallet_type {
            WalletType::Phantom => connect_phantom(self).await,
            WalletType::MetaMask => connect_metamask(self).await,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn connect(&self, _wallet_type: WalletType) -> Result<(), String> {
        Err("Wallet connection not supported in server environment".to_string())
    }

    pub fn disconnect(&self) {
        self.set_state.update(|state| {
            state.connected = false;
            state.address = None;
            state.wallet_type = None;
            state.error = None;
            state.connecting = false;
        });
    }

    pub fn set_error(&self, error: &str) {
        self.set_state.update(|state| {
            state.error = Some(error.to_string());
        });
    }
}

#[component]
pub fn WalletProvider(children: Children) -> impl IntoView {
    let (state, set_state) = create_signal(WalletState {
        connected: false,
        address: None,
        wallet_type: None,
        error: None,
        connecting: false,
    });

    provide_context(WalletContext {
        state,
        set_state,
    });

    children()
} 
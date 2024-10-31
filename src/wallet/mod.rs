mod phantom;
mod metamask;

use leptos::*;
use serde::{Deserialize, Serialize};
use web_sys::console;
use wasm_bindgen::JsValue;

pub use phantom::connect_phantom;
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
    pub async fn connect(&self, wallet_type: WalletType) {
        match wallet_type {
            WalletType::Phantom => connect_phantom(self).await,
            WalletType::MetaMask => connect_metamask(self).await,
        }
    }

    pub async fn disconnect(&self) {
        self.set_state.update(|state| {
            state.connected = false;
            state.address = None;
            state.wallet_type = None;
            state.error = None;
        });
    }

    pub(crate) fn set_error(&self, message: &str) {
        self.set_state.update(|state| {
            state.error = Some(message.to_string());
        });
        console::error_1(&JsValue::from_str(message));
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
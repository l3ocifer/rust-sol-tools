#[cfg(target_arch = "wasm32")]
mod phantom;
#[cfg(target_arch = "wasm32")]
mod metamask;

use leptos::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use solana_sdk::{
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
};
use wasm_bindgen::prelude::*;

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
    connection: Arc<Connection>,
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
        self.state.update(|state| {
            state.connected = false;
            state.address = None;
            state.wallet_type = None;
            state.error = None;
            state.connecting = false;
        });
    }

    pub fn set_error(&self, error: &str) {
        self.state.update(|state| {
            state.error = Some(error.to_string());
        });
    }

    pub async fn get_sol_balance(&self) -> Result<f64, String> {
        if let Some(pubkey) = self.state.get().address {
            let balance = self.connection
                .get_balance(&pubkey)
                .await
                .map_err(|e| e.to_string())?;
            Ok(balance as f64 / 1_000_000_000.0) // Convert lamports to SOL
        } else {
            Err("Wallet not connected".to_string())
        }
    }

    pub async fn get_token_balance(&self) -> Result<f64, String> {
        if let Some(pubkey) = self.state.get().address {
            let token_balance = self.connection
                .get_token_account_balance(&pubkey)
                .await
                .map_err(|e| e.to_string())?;
            Ok(token_balance.ui_amount.unwrap_or(0.0))
        } else {
            Err("Wallet not connected".to_string())
        }
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
        connection: Arc::new(Connection::new()),
    });

    children()
} 
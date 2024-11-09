#[cfg(target_arch = "wasm32")]
mod phantom;
#[cfg(target_arch = "wasm32")]
mod metamask;

use leptos::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use solana_sdk::pubkey::Pubkey;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_filter::TokenAccountsFilter;
use std::str::FromStr;

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
    connection: Arc<RpcClient>,
}

impl WalletContext {
    pub fn new(state: RwSignal<WalletState>) -> Self {
        Self {
            state,
            connection: Arc::new(RpcClient::new("https://api.devnet.solana.com".to_string())),
        }
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

    pub async fn get_sol_balance(&self) -> Result<f64, String> {
        if let Some(address) = &self.state.get().address {
            let pubkey = Pubkey::from_str(address)
                .map_err(|e| e.to_string())?;
            let balance = self.connection
                .get_balance(&pubkey)
                .map_err(|e| e.to_string())?;
            Ok(balance as f64 / 1_000_000_000.0)
        } else {
            Err("Wallet not connected".to_string())
        }
    }

    pub async fn get_token_balance(&self) -> Result<f64, String> {
        if let Some(address) = &self.state.get().address {
            let pubkey = Pubkey::from_str(address)
                .map_err(|e| e.to_string())?;
            let token_accounts = self.connection
                .get_token_accounts_by_owner(&pubkey, TokenAccountsFilter::ProgramId(spl_token::id()))
                .map_err(|e| e.to_string())?;
            
            let total: f64 = token_accounts.iter()
                .map(|account| account.account.data.token_amount.ui_amount.unwrap_or(0.0))
                .sum();
            Ok(total)
        } else {
            Err("Wallet not connected".to_string())
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
    });

    provide_context(WalletContext::new(state));
    children()
} 
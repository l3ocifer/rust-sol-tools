use serde::{Serialize, Deserialize};

// Re-export NetworkType and TokenCreationResult
pub use crate::utils::contract::{NetworkType, TokenCreationResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTokenParams {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub metadata_uri: String,
    pub decimals: u8,
    pub initial_supply: u64,
    pub is_mutable: bool,
    pub freeze_authority: bool,
    pub rate_limit: Option<u64>,
    pub transfer_fee: Option<u16>,
    pub max_transfer_amount: Option<u64>,
    pub network: NetworkType,
    #[cfg(not(target_arch = "wasm32"))]
    pub payer: Option<solana_sdk::signature::Keypair>,
}

#[cfg(not(target_arch = "wasm32"))]
mod server;

#[cfg(not(target_arch = "wasm32"))]
pub use server::create_token;

#[cfg(target_arch = "wasm32")]
pub async fn create_token(_params: CreateTokenParams) -> Result<TokenCreationResult, Box<dyn std::error::Error>> {
    Err("Token creation not supported in browser".into())
} 
use serde::{Serialize, Deserialize};

#[cfg(not(target_arch = "wasm32"))]
use solana_sdk::signature::Keypair;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    #[serde(skip)]
    pub payer: Option<Keypair>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum NetworkType {
    Devnet,
    Mainnet,
}

impl NetworkType {
    pub fn rpc_url(&self) -> &str {
        match self {
            NetworkType::Devnet => "https://api.devnet.solana.com",
            NetworkType::Mainnet => "https://api.mainnet-beta.solana.com",
        }
    }

    pub fn explorer_url(&self) -> &str {
        match self {
            NetworkType::Devnet => "https://explorer.solana.com/address/{}?cluster=devnet",
            NetworkType::Mainnet => "https://explorer.solana.com/address/{}",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenCreationResult {
    pub status: String,
    pub mint: String,
    pub explorer_url: String,
    pub signature: String,
}

#[cfg(not(target_arch = "wasm32"))]
mod server;

#[cfg(not(target_arch = "wasm32"))]
pub use server::create_token;

#[cfg(target_arch = "wasm32")]
pub async fn create_token(_params: CreateTokenParams) -> Result<TokenCreationResult, Box<dyn std::error::Error>> {
    Err("Token creation not supported in browser".into())
} 
use serde::{Serialize, Deserialize};

#[cfg(not(target_arch = "wasm32"))]
use {
    solana_sdk::signature::Keypair,
    solana_program::program_pack::Pack,
    spl_token::state::Mint,
    mpl_token_metadata::instruction::create_metadata_accounts_v3,
    mpl_token_metadata::pda::find_metadata_account,
};

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
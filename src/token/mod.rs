use serde::{Serialize, Deserialize};
use crate::utils::contract::{self, NetworkType, TokenConfig, TokenCreationResult};

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
pub async fn create_token(params: CreateTokenParams) -> Result<TokenCreationResult, Box<dyn std::error::Error>> {
    let config = TokenConfig {
        name: params.name,
        symbol: params.symbol,
        uri: params.metadata_uri,
        decimals: params.decimals,
        initial_supply: params.initial_supply,
        is_mutable: params.is_mutable,
        freeze_authority: params.freeze_authority,
        rate_limit: params.rate_limit,
        transfer_fee: params.transfer_fee,
        max_transfer_amount: params.max_transfer_amount,
        network: params.network,
        #[cfg(not(target_arch = "wasm32"))]
        payer: params.payer,
    };
    
    contract::create_token(config).await
}

#[cfg(target_arch = "wasm32")]
pub async fn create_token(_params: CreateTokenParams) -> Result<TokenCreationResult, Box<dyn std::error::Error>> {
    Err("Token creation not supported in browser".into())
} 
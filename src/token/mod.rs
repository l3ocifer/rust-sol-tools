use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTokenParams {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub metadata_uri: String,
    pub decimals: u8,
    pub initial_supply: u64,
    pub is_mutable: bool,
    pub freeze_authority: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenCreationResult {
    pub status: String,
    pub mint: String,
    pub explorer_url: String,
    pub signature: String,
}

#[cfg(not(target_arch = "wasm32"))]
mod solana {
    use super::*;
    use crate::utils::contract;
    
    pub async fn create_token(params: CreateTokenParams) -> Result<TokenCreationResult, Box<dyn std::error::Error>> {
        let config = contract::TokenConfig {
            name: params.name,
            symbol: params.symbol,
            uri: params.metadata_uri,
            decimals: params.decimals,
            initial_supply: params.initial_supply,
            is_mutable: params.is_mutable,
            freeze_authority: params.freeze_authority,
        };
        
        contract::create_token(config).await
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use solana::create_token;

#[cfg(target_arch = "wasm32")]
pub async fn create_token(_params: CreateTokenParams) -> Result<TokenCreationResult, Box<dyn std::error::Error>> {
    Err("Token creation not supported in browser".into())
} 
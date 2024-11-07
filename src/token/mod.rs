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
        let signature = contract::create_token(params.metadata_uri).await?;
        Ok(TokenCreationResult {
            status: "success".to_string(),
            mint: "".to_string(), // TODO: Return from create_token
            explorer_url: format!("https://explorer.solana.com/tx/{}?cluster=devnet", signature),
            signature,
        })
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use solana::create_token;

#[cfg(target_arch = "wasm32")]
pub async fn create_token(_params: CreateTokenParams) -> Result<TokenCreationResult, Box<dyn std::error::Error>> {
    Err("Token creation not supported in browser".into())
} 
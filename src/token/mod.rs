use serde::{Serialize, Deserialize};

#[cfg(not(target_arch = "wasm32"))]
use solana_sdk::signature::Keypair;

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
    pub rate_limit: Option<u64>,
    pub transfer_fee: Option<u16>,
    pub max_transfer_amount: Option<u64>,
    #[cfg(not(target_arch = "wasm32"))]
    #[serde(skip)]
    pub payer: Option<Keypair>,
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
    use crate::utils::contract::{self, TokenConfig};
    
    pub async fn create_token(params: CreateTokenParams) -> Result<TokenCreationResult, Box<dyn std::error::Error>> {
        let payer = params.payer.as_ref().ok_or("Payer keypair is required")?;
        
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
        };
        
        let result = contract::create_token(payer, config).await?;
        Ok(TokenCreationResult {
            status: result.status,
            mint: result.mint,
            explorer_url: result.explorer_url,
            signature: result.signature,
        })
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use solana::create_token;

#[cfg(target_arch = "wasm32")]
pub async fn create_token(_params: CreateTokenParams) -> Result<TokenCreationResult, Box<dyn std::error::Error>> {
    Err("Token creation not supported in browser".into())
} 
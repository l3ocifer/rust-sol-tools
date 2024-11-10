use crate::utils::contract::{TokenConfig, TokenCreationResult, NetworkType};
use solana_sdk::signature::Keypair;

#[derive(Clone, Debug)]
pub struct CreateTokenParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
    pub initial_supply: u64,
    pub is_mutable: bool,
    pub freeze_authority: bool,
    pub rate_limit: Option<u64>,
    pub transfer_fee: Option<u16>,
    pub max_transfer_amount: Option<u64>,
    pub network: NetworkType,
    #[cfg(not(target_arch = "wasm32"))]
    pub payer: Option<Keypair>,
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn create_token(params: CreateTokenParams) -> Result<TokenCreationResult, Box<dyn std::error::Error>> {
    let config = TokenConfig {
        name: params.name,
        symbol: params.symbol,
        uri: params.uri,
        decimals: params.decimals,
        initial_supply: params.initial_supply,
        is_mutable: params.is_mutable,
        freeze_authority: params.freeze_authority,
        rate_limit: params.rate_limit,
        transfer_fee: params.transfer_fee,
        max_transfer_amount: params.max_transfer_amount,
        network: params.network,
    };
    
    crate::utils::contract::create_token(config).await
} 
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq)]
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
            NetworkType::Devnet => "https://solscan.io/token/{}?cluster=devnet",
            NetworkType::Mainnet => "https://solscan.io/token/{}",
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
pub struct TokenConfig {
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
    pub payer: Option<solana_sdk::signature::Keypair>,
} 
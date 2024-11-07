#[cfg(not(target_arch = "wasm32"))]
use solana_client::rpc_client::RpcClient;
#[cfg(not(target_arch = "wasm32"))]
use solana_sdk::{
    commitment_config::CommitmentConfig,
};

#[cfg(not(target_arch = "wasm32"))]
pub async fn create_token(metadata_uri: String) -> Result<String, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    
    let env = envy::from_env::<Env>()
        .map_err(|e| format!("Failed to load environment variables: {}", e))?;

    let client = RpcClient::new_with_commitment(
        env.rpc_url,
        CommitmentConfig::confirmed(),
    );

    // TODO: Implement token creation logic directly here
    todo!("Implement token creation")
}

#[cfg(target_arch = "wasm32")]
pub async fn create_token(_metadata_uri: String) -> Result<String, Box<dyn std::error::Error>> {
    Err("Token creation not supported in browser".into())
}

#[derive(serde::Deserialize)]
pub struct Env {
    pub rpc_url: String,
    pub signer_keypair_path: String,
    pub token_name: String,
    pub token_symbol: String,
    pub token_uri: String,
    pub token_decimals: u8,
    pub initial_supply: u64,
    pub recipient_address: Option<String>,
    pub sample_amount: Option<u64>,
} 
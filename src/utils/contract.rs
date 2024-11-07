#[cfg(not(target_arch = "wasm32"))]
use solana_client::rpc_client::RpcClient;
#[cfg(not(target_arch = "wasm32"))]
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signer},
    transaction::Transaction,
};
use std::env;

#[cfg(not(target_arch = "wasm32"))]
pub async fn create_token(metadata_uri: String) -> Result<String, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    
    let env = envy::from_env::<crate::bin::create_spl::Env>()
        .map_err(|e| format!("Failed to load environment variables: {}", e))?;

    let client = RpcClient::new_with_commitment(
        env.rpc_url,
        CommitmentConfig::confirmed(),
    );

    // Reuse create_spl.rs implementation
    let result = crate::bin::create_spl::create_token(
        &client,
        &env,
        metadata_uri,
    )?;

    Ok(result.signature.to_string())
}

#[cfg(target_arch = "wasm32")]
pub async fn create_token(_metadata_uri: String) -> Result<String, Box<dyn std::error::Error>> {
    Err("Token creation not supported in browser".into())
} 
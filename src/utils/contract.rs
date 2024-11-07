#[cfg(not(target_arch = "wasm32"))]
use solana_client::rpc_client::RpcClient;
#[cfg(not(target_arch = "wasm32"))]
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    message::Message,
    pubkey::Pubkey,
    transaction::Transaction,
    signer::Signer,
};

#[cfg(not(target_arch = "wasm32"))]
pub async fn create_token(metadata_uri: String) -> Result<String, Box<dyn std::error::Error>> {
    let rpc_client = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    // Implementation using create_spl.rs logic
    todo!("Implement token creation using create_spl.rs logic")
}

#[cfg(target_arch = "wasm32")]
pub async fn create_token(_metadata_uri: String) -> Result<String, Box<dyn std::error::Error>> {
    Err("Token creation not supported in browser".into())
} 
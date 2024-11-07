use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    message::Message,
    pubkey::Pubkey,
    transaction::Transaction,
};
use std::str::FromStr;

pub async fn prepare_token_transaction(metadata_uri: String, wallet_pubkey: String) -> Result<Transaction, Box<dyn std::error::Error>> {
    let rpc_client = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    let wallet_pubkey = Pubkey::from_str(&wallet_pubkey)?;
    let program_id = Pubkey::from_str("Your_Program_ID")?;

    let instruction = Instruction::new_with_bytes(
        program_id,
        metadata_uri.as_bytes(),
        vec![],
    );

    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    let message = Message::new(&[instruction], Some(&wallet_pubkey));
    
    Ok(Transaction::new_unsigned(message))
} 
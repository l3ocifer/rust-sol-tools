use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    pubkey::Pubkey,
    signer::{keypair::Keypair, Signer},
    transaction::Transaction,
};
use spl_associated_token_account::instruction as associated_token_instruction;

#[derive(serde::Deserialize)]
struct Env {
    rpc_url: url::Url,
    signer_keypair_path: String,
    mint_account_pubkey: String,
    receiver_pubkey: String,
    amount: u64,
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = envy::from_env::<Env>()?;
    let rpc_url = env.rpc_url.to_string();
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    let payer = solana_sdk::signature::read_keypair_file(&env.signer_keypair_path)
        .map_err(|e| format!("Failed to read keypair file: {}", e))?;
    
    let receiver_pubkey: Pubkey = env.receiver_pubkey.parse()
        .map_err(|e| format!("Invalid receiver pubkey: {}", e))?;
    let mint_account_pubkey: Pubkey = env.mint_account_pubkey.parse()
        .map_err(|e| format!("Invalid mint account pubkey: {}", e))?;

    println!("Minting {} tokens to {}", env.amount, receiver_pubkey);

    // Get or create associated token account
    let associated_token_account = spl_associated_token_account::get_associated_token_address(
        &receiver_pubkey,
        &mint_account_pubkey,
    );

    let create_ata_ix = associated_token_instruction::create_associated_token_account(
        &payer.pubkey(),
        &receiver_pubkey,
        &mint_account_pubkey,
    );

    let mint_to_ix = spl_token::instruction::mint_to(
        &spl_token::id(),
        &mint_account_pubkey,
        &associated_token_account,
        &payer.pubkey(),
        &[],
        env.amount,
    ).map_err(|e| format!("Failed to create mint instruction: {}", e))?;

    // Build and send transaction
    let recent_blockhash = client
        .get_latest_blockhash()
        .map_err(|e| format!("Failed to get recent blockhash: {}", e))?;

    let mut transaction = Transaction::new_with_payer(
        &[create_ata_ix, mint_to_ix],
        Some(&payer.pubkey()),
    );

    transaction.sign(&[&payer], recent_blockhash);

    client
        .send_and_confirm_transaction(&transaction)
        .map_err(|e| format!("Failed to send transaction: {}", e))?;

    println!("Tokens minted successfully!");
    println!("Amount: {}", env.amount);
    println!("Receiver: {}", receiver_pubkey);
    println!("Associated Token Account: {}", associated_token_account);

    Ok(())
} 
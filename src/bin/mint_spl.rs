use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signer::{keypair::Keypair, Signer},
    transaction::Transaction,
};
use spl_associated_token_account::instruction as associated_token_instruction;

#[derive(serde::Deserialize)]
struct Env {
    rpc_url: url::Url,
    signer_keypair: String,
    mint_account_pubkey: String,
    receiver_pubkey: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = envy::from_env::<Env>()?;
    let signer_wallet = Keypair::from_base58_string(&env.signer_keypair);
    let client = RpcClient::new(env.rpc_url.to_string());
    let receiver_pubkey: Pubkey = env.receiver_pubkey.parse()?;
    let mint_account_pubkey: Pubkey = env.mint_account_pubkey.parse()?;

    let amount = 10_000;

    let assoc = spl_associated_token_account::get_associated_token_address(
        &receiver_pubkey,
        &mint_account_pubkey,
    );

    let assoc_instruction = associated_token_instruction::create_associated_token_account(
        &signer_wallet.pubkey(), // Payer
        &receiver_pubkey,        // Wallet address
        &mint_account_pubkey,    // Mint address
    );

    let mint_to_instruction: Instruction = spl_token::instruction::mint_to(
        &spl_token::id(),
        &mint_account_pubkey,
        &assoc,
        &signer_wallet.pubkey(),
        &[],
        amount,
    )?;

    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction: Transaction = Transaction::new_signed_with_payer(
        &[assoc_instruction, mint_to_instruction],
        Some(&signer_wallet.pubkey()),
        &[&signer_wallet],
        recent_blockhash,
    );

    client.send_and_confirm_transaction(&transaction)?;

    println!("SPL Tokens minted successfully.");
    println!("Amount: {}", amount);
    println!("Receiver pubkey: {}", receiver_pubkey.to_string());
    println!("Associated token account: {}", assoc.to_string());

    Ok(())
} 
use solana_client::rpc_client::RpcClient;
use solana_program::system_instruction;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signer},
    transaction::Transaction,
};
use spl_associated_token_account::{self, get_associated_token_address};
use spl_token::state::Mint;
use mpl_token_metadata::{instruction as token_metadata_instruction, ID as TOKEN_METADATA_PROGRAM_ID};
use mpl_token_metadata::state::Data;
use borsh::BorshSerialize;

#[derive(serde::Deserialize)]
struct Env {
    rpc_url: url::Url,
    signer_keypair_path: String,
    token_name: String,
    token_symbol: String,
    token_uri: String,
    token_decimals: u8,
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = envy::from_env::<Env>()?;
    let rpc_url = env.rpc_url.to_string();
    let client = RpcClient::new_with_commitment(rpc_url.clone(), CommitmentConfig::confirmed());
    let payer = read_keypair_file(&env.signer_keypair_path)
        .map_err(|e| format!("Failed to read keypair file: {}", e))?;

    println!("Creating new token with name: {}", env.token_name);

    // Generate a new keypair for the mint account
    let mint_account = Keypair::new();
    
    // Calculate the minimum balance for rent exemption
    let mint_rent = client
        .get_minimum_balance_for_rent_exemption(Mint::LEN)
        .map_err(|e| format!("Failed to get rent exemption: {}", e))?;

    // Create transaction instructions
    let create_mint_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint_account.pubkey(),
        mint_rent,
        Mint::LEN as u64,
        &spl_token::id(),
    );

    let initialize_mint_ix = spl_token::instruction::initialize_mint(
        &spl_token::id(),
        &mint_account.pubkey(),
        &payer.pubkey(),
        Some(&payer.pubkey()),
        env.token_decimals,
    )?;

    // Create the associated token account for the payer
    let ata = get_associated_token_address(&payer.pubkey(), &mint_account.pubkey());
    
    let create_ata_ix = spl_associated_token_account::instruction::create_associated_token_account(
        &payer.pubkey(),
        &payer.pubkey(),
        &mint_account.pubkey(),
    );

    // Create metadata account
    let (metadata_account, _) = Pubkey::find_program_address(
        &[
            b"metadata",
            TOKEN_METADATA_PROGRAM_ID.as_ref(),
            mint_account.pubkey().as_ref(),
        ],
        &TOKEN_METADATA_PROGRAM_ID,
    );

    let metadata_data = Data {
        name: env.token_name.clone(),
        symbol: env.token_symbol.clone(),
        uri: env.token_uri.clone(),
        seller_fee_basis_points: 0,
        creators: None,
    };

    let create_metadata_accounts_ix = token_metadata_instruction::create_metadata_accounts_v3(
        TOKEN_METADATA_PROGRAM_ID,
        metadata_account,
        mint_account.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
        metadata_data.name,
        metadata_data.symbol,
        metadata_data.uri,
        None,
        0,
        true,
        true,  // Is mutable
        None,  // Collection
        None,  // Uses
        None,  // Collection Details
    );

    // Build and send transaction
    let recent_blockhash = client
        .get_latest_blockhash()
        .map_err(|e| format!("Failed to get recent blockhash: {}", e))?;

    let mut transaction = Transaction::new_with_payer(
        &[
            create_mint_account_ix,
            initialize_mint_ix,
            create_ata_ix,
            create_metadata_accounts_ix,
        ],
        Some(&payer.pubkey()),
    );

    transaction.sign(&[&payer, &mint_account], recent_blockhash);

    client
        .send_and_confirm_transaction(&transaction)
        .map_err(|e| format!("Failed to send transaction: {}", e))?;

    println!("Token created successfully!");
    println!("Mint Address: {}", mint_account.pubkey());
    println!("Metadata Address: {}", metadata_account);

    Ok(())
} 
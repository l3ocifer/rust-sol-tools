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
    let payer = read_keypair_file(env.signer_keypair_path.clone())?;

    // Generate a new keypair for the mint account
    let mint_account = Keypair::new();

    // Calculate the minimum balance for rent exemption
    let mint_rent = client.get_minimum_balance_for_rent_exemption(Mint::LEN)?;

    // Create the mint account
    let create_mint_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint_account.pubkey(),
        mint_rent,
        Mint::LEN as u64,
        &spl_token::id(),
    );

    // Initialize the mint
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

    // Mint tokens to the associated token account (Optional)
    let amount = 1; // Adjust the amount as needed
    let mint_to_ix = spl_token::instruction::mint_to(
        &spl_token::id(),
        &mint_account.pubkey(),
        &ata,
        &payer.pubkey(),
        &[],
        amount,
    )?;

    // Create Metadata account
    let metadata_account = Pubkey::find_program_address(
        &[
            b"metadata",
            TOKEN_METADATA_PROGRAM_ID.as_ref(),
            mint_account.pubkey().as_ref(),
        ],
        &TOKEN_METADATA_PROGRAM_ID,
    )
    .0;

    let metadata_data = Data {
        name: env.token_name.clone(),
        symbol: env.token_symbol.clone(),
        uri: env.token_uri.clone(),
        seller_fee_basis_points: 0,
        creators: None,
    };

    let create_metadata_accounts_ix = token_metadata_instruction::create_metadata_accounts(
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
        false,
    );

    // Build the transaction
    let mut transaction = Transaction::new_with_payer(
        &[
            create_mint_account_ix,
            initialize_mint_ix,
            create_ata_ix,
            mint_to_ix,
            create_metadata_accounts_ix,
        ],
        Some(&payer.pubkey()),
    );

    let recent_blockhash = client.get_latest_blockhash()?;
    transaction.sign(&[&payer, &mint_account], recent_blockhash);

    // Send the transaction
    client.send_and_confirm_transaction(&transaction)?;

    println!("Token mint created successfully.");
    println!("Mint Address: {}", mint_account.pubkey());
    println!("Token Name: {}", env.token_name);
    println!("Token Symbol: {}", env.token_symbol);
    println!("Token URI: {}", env.token_uri);

    Ok(())
} 
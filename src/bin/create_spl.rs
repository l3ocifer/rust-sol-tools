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
use mpl_token_metadata::instructions as token_metadata_instruction;
use mpl_token_metadata::types::DataV2;
use borsh::ser::BorshSerialize;

#[derive(serde::Deserialize)]
struct Env {
    rpc_url: String,
    signer_keypair_path: String,
    token_name: String,
    token_symbol: String,
    token_uri: String,
    token_decimals: u8,
    initial_supply: u64,
    recipient_address: Option<String>,
    sample_amount: Option<u64>,
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = envy::from_env::<Env>()?;
    let rpc_url = env.rpc_url;
    let client = RpcClient::new_with_commitment(rpc_url.clone(), CommitmentConfig::confirmed());
    let payer = read_keypair_file(&env.signer_keypair_path)
        .map_err(|e| format!("Failed to read keypair file: {}", e))?;

    println!("Creating new token with name: {}", env.token_name);

    // Generate mint account
    let mint_account = Keypair::new();
    let mint_rent = client.get_minimum_balance_for_rent_exemption(Mint::LEN)?;

    // Determine recipient
    let recipient = if let Some(addr) = env.recipient_address {
        addr.parse::<Pubkey>()?
    } else {
        payer.pubkey()
    };

    // Calculate amounts with decimal adjustment
    let decimals_multiplier = 10u64.pow(env.token_decimals as u32);
    let initial_supply = env.initial_supply.saturating_mul(decimals_multiplier);
    let sample_amount = env.sample_amount.unwrap_or(1000).saturating_mul(decimals_multiplier);

    // Create all necessary instructions
    let mut instructions = vec![
        // Create mint account
        system_instruction::create_account(
            &payer.pubkey(),
            &mint_account.pubkey(),
            mint_rent,
            Mint::LEN as u64,
            &spl_token::id(),
        ),
        // Initialize mint
        spl_token::instruction::initialize_mint(
            &spl_token::id(),
            &mint_account.pubkey(),
            &payer.pubkey(),
            Some(&payer.pubkey()),
            env.token_decimals,
        )?,
    ];

    // Create recipient's ATA
    let recipient_ata = get_associated_token_address(&recipient, &mint_account.pubkey());
    instructions.push(
        spl_associated_token_account::instruction::create_associated_token_account(
            &payer.pubkey(),
            &recipient,
            &mint_account.pubkey(),
        ),
    );

    // Add mint instructions
    if initial_supply > 0 {
        instructions.push(
            spl_token::instruction::mint_to(
                &spl_token::id(),
                &mint_account.pubkey(),
                &recipient_ata,
                &payer.pubkey(),
                &[],
                initial_supply,
            )?,
        );
    }

    // Add sample amount mint instruction if different from initial supply
    if sample_amount > 0 && sample_amount != initial_supply {
        instructions.push(
            spl_token::instruction::mint_to(
                &spl_token::id(),
                &mint_account.pubkey(),
                &recipient_ata,
                &payer.pubkey(),
                &[],
                sample_amount,
            )?,
        );
    }

    // Create metadata
    let (metadata_account, _) = Pubkey::find_program_address(
        &[
            b"metadata",
            mpl_token_metadata::ID.as_ref(),
            mint_account.pubkey().as_ref(),
        ],
        &mpl_token_metadata::ID,
    );

    let metadata = DataV2 {
        name: env.token_name.clone(),
        symbol: env.token_symbol.clone(),
        uri: env.token_uri.clone(),
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    instructions.push(
        token_metadata_instruction::CreateMetadataAccountV3 {
            metadata: metadata_account,
            mint: mint_account.pubkey(),
            mint_authority: payer.pubkey(),
            payer: payer.pubkey(),
            update_authority: payer.pubkey(),
            data: metadata,
            is_mutable: true,
            collection_details: None,
            rule_set: None,
        }.instruction(),
    );

    // Execute transaction
    let recent_blockhash = client.get_latest_blockhash()?;
    let mut transaction = Transaction::new_with_payer(&instructions, Some(&payer.pubkey()));
    transaction.sign(&[&payer, &mint_account], recent_blockhash);

    let signature = client.send_and_confirm_transaction(&transaction)?;

    println!("Token created and minted successfully!");
    println!("Mint Address: {}", mint_account.pubkey());
    println!("Metadata Address: {}", metadata_account);
    println!("Recipient ATA: {}", recipient_ata);
    println!("Transaction: {}", signature);
    println!("Initial Supply: {} tokens", initial_supply);
    if sample_amount > 0 && sample_amount != initial_supply {
        println!("Sample Amount: {} tokens", sample_amount);
    }

    Ok(())
} 
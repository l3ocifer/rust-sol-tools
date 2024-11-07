#[cfg(not(target_arch = "wasm32"))]
use {
    solana_client::rpc_client::RpcClient,
    solana_program::system_instruction,
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signer},
        transaction::Transaction,
    },
    spl_associated_token_account::get_associated_token_address,
    spl_token::state::Mint,
    mpl_token_metadata::{
        instructions as token_metadata_instruction,
        types::DataV2,
    },
};

#[derive(serde::Deserialize)]
pub struct TokenConfig {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
    pub initial_supply: u64,
    pub is_mutable: bool,
    pub freeze_authority: bool,
}

#[derive(serde::Serialize)]
pub struct TokenCreationResult {
    pub status: String,
    pub mint: String,
    pub explorer_url: String,
    pub signature: String,
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn create_token(config: TokenConfig) -> Result<TokenCreationResult, Box<dyn std::error::Error>> {
    let rpc_client = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    let payer = read_keypair_file(&std::env::var("SOLANA_KEYPAIR_PATH")?)?;
    let mint_account = Keypair::new();
    let mint_rent = rpc_client.get_minimum_balance_for_rent_exemption(Mint::LEN)?;

    let mut instructions = vec![
        system_instruction::create_account(
            &payer.pubkey(),
            &mint_account.pubkey(),
            mint_rent,
            Mint::LEN as u64,
            &spl_token::id(),
        ),
        spl_token::instruction::initialize_mint(
            &spl_token::id(),
            &mint_account.pubkey(),
            &payer.pubkey(),
            if config.freeze_authority { Some(&payer.pubkey()) } else { None },
            config.decimals,
        )?,
    ];

    let recipient_ata = get_associated_token_address(&payer.pubkey(), &mint_account.pubkey());
    instructions.push(
        spl_associated_token_account::instruction::create_associated_token_account(
            &payer.pubkey(),
            &payer.pubkey(),
            &mint_account.pubkey(),
        ),
    );

    if config.initial_supply > 0 {
        instructions.push(
            spl_token::instruction::mint_to(
                &spl_token::id(),
                &mint_account.pubkey(),
                &recipient_ata,
                &payer.pubkey(),
                &[],
                config.initial_supply,
            )?,
        );
    }

    let metadata = DataV2 {
        name: config.name,
        symbol: config.symbol,
        uri: config.uri,
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    let (metadata_account, _) = Pubkey::find_program_address(
        &[
            b"metadata",
            mpl_token_metadata::ID.as_ref(),
            mint_account.pubkey().as_ref(),
        ],
        &mpl_token_metadata::ID,
    );

    instructions.push(
        token_metadata_instruction::CreateMetadataAccountV3 {
            metadata: metadata_account,
            mint: mint_account.pubkey(),
            mint_authority: payer.pubkey(),
            payer: payer.pubkey(),
            update_authority: payer.pubkey(),
            data: metadata,
            is_mutable: config.is_mutable,
            collection_details: None,
            rule_set: None,
        }
        .instruction(),
    );

    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &instructions,
        Some(&payer.pubkey()),
        &[&payer, &mint_account],
        recent_blockhash,
    );

    let signature = rpc_client.send_and_confirm_transaction(&transaction)?;
    let mint_address = mint_account.pubkey().to_string();
    
    Ok(TokenCreationResult {
        status: "success".to_string(),
        mint: mint_address.clone(),
        explorer_url: format!("https://solscan.io/token/{}?cluster=devnet", mint_address),
        signature: signature.to_string(),
    })
}

#[cfg(target_arch = "wasm32")]
pub async fn create_token(_config: TokenConfig) -> Result<TokenCreationResult, Box<dyn std::error::Error>> {
    Err("Token creation not supported in browser".into())
} 
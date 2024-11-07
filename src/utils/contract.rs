use borsh::{BorshSerialize, BorshDeserialize}; // Traits
use borsh_derive::{BorshSerialize, BorshDeserialize}; // Derive macros

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
    spl_token_2022::state::Mint,
    mpl_token_metadata::{
        instruction as token_metadata_instruction,
        state::{DataV2, TokenStandard},
        pda::find_metadata_account,
    },
    mpl_token_auth_rules::{
        instruction as auth_rules_instruction,
        state::{RuleSetV1, RuleSetV2},
    },
};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct TokenConfig {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
    pub initial_supply: u64,
    pub is_mutable: bool,
    pub freeze_authority: bool,
    pub rate_limit: Option<u64>,
    pub transfer_fee: Option<u16>,
    pub max_transfer_amount: Option<u64>,
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
            &spl_token_2022::id(),
        ),
    ];

    // Initialize mint with Token2022 extensions
    let mut mint_extensions = vec![];
    
    if config.rate_limit.is_some() {
        mint_extensions.push(spl_token_2022::extension::rate_limit::instruction::initialize(
            &spl_token_2022::id(),
            &mint_account.pubkey(),
            Some(&payer.pubkey()),
            config.rate_limit.unwrap(),
        )?);
    }

    if config.transfer_fee.is_some() {
        mint_extensions.push(spl_token_2022::extension::transfer_fee::instruction::initialize(
            &spl_token_2022::id(),
            &mint_account.pubkey(),
            Some(&payer.pubkey()),
            config.transfer_fee.unwrap(),
            0, // Transfer fee denominator
        )?);
    }

    instructions.extend(mint_extensions);

    // Initialize base mint
    instructions.push(
        spl_token_2022::instruction::initialize_mint2(
            &spl_token_2022::id(),
            &mint_account.pubkey(),
            &payer.pubkey(),
            if config.freeze_authority { Some(&payer.pubkey()) } else { None },
            config.decimals,
        )?,
    );

    // Create and initialize metadata
    let (metadata_account, _) = find_metadata_account(&mint_account.pubkey());
    
    let metadata = DataV2 {
        name: config.name,
        symbol: config.symbol,
        uri: config.uri,
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    // Create rule set if needed
    let rule_set = if config.max_transfer_amount.is_some() || 
                     config.rate_limit.is_some() || 
                     config.transfer_fee.is_some() {
        Some(RuleSetV1 {
            max_transfer_amount: config.max_transfer_amount,
            rate_limit: config.rate_limit,
            transfer_fee: config.transfer_fee,
        })
    } else {
        None
    };

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
            rule_set,
        }
        .instruction(),
    );

    // Execute transaction
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
use anyhow::Result;
use solana_sdk::{
    signature::Keypair,
    signer::Signer,
    commitment_config::CommitmentConfig,
    transaction::Transaction,
};
use solana_client::rpc_client::RpcClient;
use spl_token::state::Mint;
use solana_program::{program_pack::Pack, system_instruction};
use mpl_token_metadata::{
    instruction::create_metadata_accounts_v3,
    types::DataV2,
    ID as TOKEN_METADATA_PROGRAM_ID,
};
use super::{CreateTokenParams, TokenCreationResult, NetworkType};

pub async fn create_token(params: CreateTokenParams) -> Result<TokenCreationResult> {
    let payer = params.payer.unwrap_or_else(Keypair::new);
    let mint = Keypair::new();
    
    let client = RpcClient::new_with_commitment(
        params.network.rpc_url().to_string(),
        CommitmentConfig::confirmed(),
    );
    
    let mint_rent = client.get_minimum_balance_for_rent_exemption(Mint::LEN)?;
    
    let mut instructions = vec![
        // Create mint account
        system_instruction::create_account(
            &payer.pubkey(),
            &mint.pubkey(),
            mint_rent,
            Mint::LEN as u64,
            &spl_token::id(),
        ),
        // Initialize mint
        spl_token::instruction::initialize_mint(
            &spl_token::id(),
            &mint.pubkey(),
            &payer.pubkey(),
            Some(&payer.pubkey()),
            params.decimals,
        )?,
    ];

    // Create metadata account
    let (metadata_account, _) = mpl_token_metadata::pda::find_metadata_account(&mint.pubkey());
    
    let metadata_instruction = create_metadata_accounts_v3(
        TOKEN_METADATA_PROGRAM_ID,
        metadata_account,
        mint.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
        params.name,
        params.symbol,
        params.metadata_uri,
        None,
        0,
        params.is_mutable,
        None,
        None,
        None,
    );
    
    instructions.push(metadata_instruction);

    let recent_blockhash = client.get_latest_blockhash()?;
    let mut transaction = Transaction::new_with_payer(&instructions, Some(&payer.pubkey()));
    transaction.sign(&[&payer, &mint], recent_blockhash);

    let signature = client.send_and_confirm_transaction(&transaction)?;
    
    Ok(TokenCreationResult {
        status: "Created".to_string(),
        mint: mint.pubkey().to_string(),
        explorer_url: format!("{}", params.network.explorer_url().replace("{}", &mint.pubkey().to_string())),
        signature: signature.to_string(),
    })
} 
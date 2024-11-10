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
    instructions::CreateMetadataAccountV3InstructionArgs,
    state::DataV2,
    pda::find_metadata_account,
    ID as TOKEN_METADATA_PROGRAM_ID,
};
use super::{CreateTokenParams, TokenCreationResult};

pub async fn create_token(params: CreateTokenParams) -> Result<TokenCreationResult> {
    let payer = params.payer.unwrap_or_else(Keypair::new);
    let mint = Keypair::new();
    
    let client = RpcClient::new_with_commitment(
        params.network.rpc_url(),
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
    let (metadata_account, _) = find_metadata_account(&mint.pubkey());
    
    let metadata_instruction = CreateMetadataAccountV3InstructionArgs {
        data: DataV2 {
            name: params.name,
            symbol: params.symbol,
            uri: params.metadata_uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        is_mutable: params.is_mutable,
        collection_details: None,
        metadata_account,
        mint: mint.pubkey(),
        mint_authority: payer.pubkey(),
        payer: payer.pubkey(),
        update_authority: payer.pubkey(),
        system_program: solana_program::system_program::id(),
        rent: solana_program::sysvar::rent::id(),
    }.instruction();
    
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
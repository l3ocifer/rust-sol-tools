use borsh::{BorshSerialize, BorshDeserialize};

#[cfg(not(target_arch = "wasm32"))]
use {
    solana_client::rpc_client::RpcClient,
    solana_program::{pubkey::Pubkey, system_instruction, system_program, sysvar},
    solana_sdk::{
        commitment_config::CommitmentConfig,
        program_pack::Pack,
        signature::{Keypair, Signer},
        transaction::Transaction,
    },
    spl_token_2022::{
        instruction as token_instruction,
        state::Mint,
    },
    mpl_token_metadata::{
        instruction::create_metadata_accounts_v3,
        state::DataV2,
        ID as TOKEN_METADATA_PROGRAM_ID,
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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TokenCreationResult {
    pub status: String,
    pub mint: String,
    pub explorer_url: String,
    pub signature: String,
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn create_token(payer: &Keypair, config: TokenConfig) -> Result<TokenCreationResult, Box<dyn std::error::Error>> {
    let rpc_client = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

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

    instructions.push(
        token_instruction::initialize_mint2(
            &spl_token_2022::id(),
            &mint_account.pubkey(),
            &payer.pubkey(),
            Some(&payer.pubkey()),
            config.decimals,
        )?,
    );

    let (metadata_account, _) = Pubkey::find_program_address(
        &[
            b"metadata",
            TOKEN_METADATA_PROGRAM_ID.as_ref(),
            mint_account.pubkey().as_ref(),
        ],
        &TOKEN_METADATA_PROGRAM_ID,
    );
    
    instructions.push(
        create_metadata_accounts_v3(
            TOKEN_METADATA_PROGRAM_ID,
            metadata_account,
            mint_account.pubkey(),
            payer.pubkey(),
            payer.pubkey(),
            payer.pubkey(),
            config.name,
            config.symbol,
            config.uri,
            None,
            0,
            config.is_mutable,
            None,
            None,
            None,
            None,
            None,
        ),
    );

    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &instructions,
        Some(&payer.pubkey()),
        &[payer, &mint_account],
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
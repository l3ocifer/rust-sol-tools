use borsh::{BorshSerialize, BorshDeserialize};
use solana_sdk::program_pack::Pack;
use spl_token::state::Mint as TokenMint;

#[cfg(not(target_arch = "wasm32"))]
use {
    solana_client::rpc_client::RpcClient,
    solana_program::{
        pubkey::Pubkey,
        system_instruction,
    },
    solana_sdk::{
        commitment_config::CommitmentConfig,
        signature::{Keypair, Signer},
        transaction::Transaction,
    },
    spl_token_2022::instruction as token_instruction,
    mpl_token_metadata::{
        ID as TOKEN_METADATA_PROGRAM_ID,
        types::DataV2,
        instructions::CreateMetadataAccountV3Builder,
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
    pub network: NetworkType,
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum NetworkType {
    Devnet,
    Mainnet,
}

impl NetworkType {
    pub fn rpc_url(&self) -> &str {
        match self {
            NetworkType::Devnet => "https://api.devnet.solana.com",
            NetworkType::Mainnet => "https://api.mainnet-beta.solana.com",
        }
    }

    pub fn explorer_url(&self) -> &str {
        match self {
            NetworkType::Devnet => "https://solscan.io/token/{}?cluster=devnet",
            NetworkType::Mainnet => "https://solscan.io/token/{}",
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TokenCreationResult {
    pub status: String,
    pub mint: String,
    pub explorer_url: String,
    pub signature: String,
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn create_token(config: TokenConfig) -> Result<TokenCreationResult, Box<dyn std::error::Error>> {
    let rpc_client = RpcClient::new(config.network.rpc_url());
    let payer = Keypair::new();

    let mint_account = Keypair::new();
    let mint_rent = rpc_client.get_minimum_balance_for_rent_exemption(TokenMint::LEN)?;

    let mut instructions = vec![
        system_instruction::create_account(
            &payer.pubkey(),
            &mint_account.pubkey(),
            mint_rent,
            TokenMint::LEN as u64,
            &spl_token_2022::id(),
        ),
        token_instruction::initialize_mint(
            &spl_token_2022::id(),
            &mint_account.pubkey(),
            &payer.pubkey(),
            Some(&payer.pubkey()),
            config.decimals,
        )?,
    ];

    // Prepare metadata data
    let metadata_data = DataV2 {
        name: config.name.clone(),
        symbol: config.symbol.clone(),
        uri: config.uri.clone(),
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    // Find metadata account PDA
    let (metadata_account, _) = Pubkey::find_program_address(
        &[
            b"metadata",
            TOKEN_METADATA_PROGRAM_ID.as_ref(),
            mint_account.pubkey().as_ref(),
        ],
        &TOKEN_METADATA_PROGRAM_ID,
    );

    // Create the metadata account instruction using the builder
    let create_metadata_ix = CreateMetadataAccountV3Builder::new()
        .metadata(metadata_account)
        .mint(mint_account.pubkey())
        .mint_authority(payer.pubkey())
        .payer(payer.pubkey())
        .update_authority(payer.pubkey(), false)
        .data(metadata_data)
        .is_mutable(config.is_mutable)
        .instruction();

    instructions.push(create_metadata_ix);

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
        explorer_url: format!("{}", config.network.explorer_url().replace("{}", &mint_address)),
        signature: signature.to_string(),
    })
}

#[cfg(target_arch = "wasm32")]
pub async fn create_token(
    _config: TokenConfig,
) -> Result<TokenCreationResult, Box<dyn std::error::Error>> {
    Err("Token creation not supported in browser".into())
} 
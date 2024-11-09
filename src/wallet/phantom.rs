use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;
use js_sys::{Function, Promise};
use leptos::SignalUpdate;
use super::WalletContext;

use solana_program::pubkey::Pubkey;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signer::Signer,
    transaction::Transaction,
    system_instruction,
};
use spl_token::instruction as token_instruction;
use mpl_token_metadata::{
    instruction::create_metadata_accounts_v3,
    state::{DataV2, CollectionDetails},
    ID as TOKEN_METADATA_PROGRAM_ID,
};

pub async fn connect_phantom(wallet_context: &WalletContext) {
    match window() {
        Some(window) => {
            if let Some(solana) = js_sys::Reflect::get(&window, &JsValue::from_str("solana")).ok() {
                if js_sys::Reflect::get(&solana, &JsValue::from_str("isPhantom")).ok().is_some() {
                    match connect_phantom_wallet(solana).await {
                        Ok(address) => {
                            wallet_context.set_state.update(|state| {
                                state.connected = true;
                                state.address = Some(address);
                                state.wallet_type = Some(super::WalletType::Phantom);
                                state.error = None;
                            });
                        }
                        Err(e) => wallet_context.set_error(&format!("Failed to connect: {}", e)),
                    }
                    return;
                }
            }
        }
        None => (),
    }
    wallet_context.set_error("Phantom wallet not found");
}

async fn connect_phantom_wallet(solana: JsValue) -> Result<String, String> {
    let connect_promise = js_sys::Reflect::get(&solana, &JsValue::from_str("connect"))
        .map_err(|_| "No connect method")?;
    
    let connect_fn = connect_promise.dyn_ref::<Function>()
        .ok_or("Connect is not a function")?;
    
    let promise = connect_fn.call0(&solana)
        .map_err(|_| "Failed to call connect")?;
    
    let _result = JsFuture::from(promise.dyn_into::<Promise>().unwrap())
        .await
        .map_err(|_| "Connection rejected")?;
    
    let public_key = js_sys::Reflect::get(&solana, &JsValue::from_str("publicKey"))
        .map_err(|_| "No public key")?;
    
    let address = js_sys::Reflect::get(&public_key, &JsValue::from_str("toString"))
        .map_err(|_| "No toString method")?;
    
    let address_fn = address.dyn_ref::<Function>()
        .ok_or("ToString is not a function")?;
    
    let address_str = address_fn.call0(&public_key)
        .map_err(|_| "Failed to get address string")?;
    
    Ok(address_str.as_string().unwrap_or_default())
}

pub async fn create_token(
    wallet_context: &WalletContext,
    name: String,
    symbol: String,
    uri: String,
    decimals: u8,
) -> Result<Pubkey, JsValue> {
    let wallet = get_wallet()?;
    let public_key = get_public_key(&wallet)?;
    let connection = get_connection()?;

    // Generate a new keypair for the mint account
    let mint = Keypair::new();
    let mint_pubkey = mint.pubkey();

    // Create the mint account
    let lamports = connection
        .get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN)
        .await
        .map_err(|e| JsValue::from_str(&format!("Error getting rent exemption: {}", e)))?;

    let create_account_ix = system_instruction::create_account(
        &public_key,
        &mint_pubkey,
        lamports,
        spl_token::state::Mint::LEN as u64,
        &spl_token::ID,
    );

    // Initialize the mint
    let initialize_mint_ix = token_instruction::initialize_mint(
        &spl_token::ID,
        &mint_pubkey,
        &public_key,
        None,
        decimals,
    )?;

    // Create associated token account
    let ata_pubkey = spl_associated_token_account::get_associated_token_address(
        &public_key,
        &mint_pubkey,
    );

    let create_ata_ix = spl_associated_token_account::instruction::create_associated_token_account(
        &public_key,
        &public_key,
        &mint_pubkey,
    );

    // Mint to the associated token account
    let mint_to_ix = token_instruction::mint_to(
        &spl_token::ID,
        &mint_pubkey,
        &ata_pubkey,
        &public_key,
        &[],
        1, // Adjust the amount as needed
    )?;

    // Create metadata account
    let metadata_pubkey = Pubkey::find_program_address(
        &[
            b"metadata",
            &TOKEN_METADATA_PROGRAM_ID.to_bytes(),
            &mint_pubkey.to_bytes(),
        ],
        &TOKEN_METADATA_PROGRAM_ID,
    )
    .0;

    let data = DataV2 {
        name,
        symbol,
        uri,
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    let create_metadata_ix = create_metadata_accounts_v3(
        TOKEN_METADATA_PROGRAM_ID,
        metadata_pubkey,
        mint_pubkey,
        public_key,
        public_key,
        public_key,
        data,
        true,
        true,
        None,
        None,
        None,
    );

    // Build the transaction
    let transaction = Transaction::new_signed_with_payer(
        &[
            create_account_ix,
            initialize_mint_ix,
            create_ata_ix,
            mint_to_ix,
            create_metadata_ix,
        ],
        Some(&public_key),
        &[&mint],
        connection
            .get_latest_blockhash()
            .await
            .map_err(|e| JsValue::from_str(&format!("Error getting blockhash: {}", e)))?,
    );

    // Send and confirm the transaction
    connection
        .send_and_confirm_transaction(&transaction)
        .await
        .map_err(|e| JsValue::from_str(&format!("Transaction error: {}", e)))?;

    Ok(mint_pubkey)
} 
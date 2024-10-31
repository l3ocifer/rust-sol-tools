use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use anyhow::{Result, anyhow};
use std::str::FromStr;
use solana_program::pubkey::Pubkey;

#[derive(Serialize, Deserialize)]
pub struct CreateTokenParams {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub metadata_uri: String,
    pub decimals: u8,
    pub initial_supply: u64,
    pub is_mutable: bool,
    pub freeze_authority: bool,
}

#[derive(Serialize, Deserialize)]
pub struct TokenCreationResult {
    pub signature: String,
    pub mint: String,
    pub metadata: String,
    pub explorer_url: String,
    pub status: String,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn solana_request(method: &str, params: JsValue) -> js_sys::Promise;
}

pub fn update_status(status: &str) {
    let window = web_sys::window().expect("no global `window` exists");
    let status_element = window
        .document()
        .expect("should have a document on window")
        .get_element_by_id("creation-status")
        .expect("should have creation-status element");
    
    status_element.set_text_content(Some(status));
}

pub async fn create_token(params: CreateTokenParams) -> Result<TokenCreationResult> {
    update_status("Initializing token creation...");
    
    // Validate parameters
    if params.decimals > 9 {
        return Err(anyhow!("Decimals must be between 0 and 9"));
    }
    
    if params.initial_supply == 0 {
        return Err(anyhow!("Initial supply must be greater than 0"));
    }

    let js_params = serde_wasm_bindgen::to_value(&params)
        .map_err(|e| anyhow!("Failed to serialize params: {}", e))?;

    let promise = solana_request("createToken", js_params);
    let result = wasm_bindgen_futures::JsFuture::from(promise).await
        .map_err(|e| anyhow!("Failed to create token: {:?}", e))?;

    let result: TokenCreationResult = serde_wasm_bindgen::from_value(result)
        .map_err(|e| anyhow!("Failed to parse response: {}", e))?;

    // Validate mint address
    if Pubkey::from_str(&result.mint).is_err() {
        return Err(anyhow!("Invalid mint address returned"));
    }

    update_status(&result.status);
    Ok(result)
} 
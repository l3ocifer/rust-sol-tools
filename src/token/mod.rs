use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use anyhow::{Result, anyhow};

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
    #[wasm_bindgen(js_namespace = window)]
    fn update_status(status: &str);
}

pub async fn create_token(params: CreateTokenParams) -> Result<TokenCreationResult> {
    let js_params = serde_wasm_bindgen::to_value(&params)
        .map_err(|e| anyhow!("Failed to serialize params: {}", e))?;

    update_status("Initializing token creation...");
    
    let promise = solana_request("createToken", js_params);
    let result = wasm_bindgen_futures::JsFuture::from(promise).await
        .map_err(|e| anyhow!("Failed to create token: {:?}", e))?;

    let result: TokenCreationResult = serde_wasm_bindgen::from_value(result)
        .map_err(|e| anyhow!("Failed to parse response: {}", e))?;

    update_status(&result.status);
    Ok(result)
} 
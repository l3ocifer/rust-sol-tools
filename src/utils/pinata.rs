use anyhow::Result;
use serde_json::Value;
use wasm_bindgen::prelude::*;
use web_sys::File;

#[wasm_bindgen(module = "/public/pinata.js")]
extern "C" {
    #[wasm_bindgen(js_name = uploadToPinata)]
    async fn upload_to_pinata(api_key: &str, api_secret: &str, data: JsValue) -> JsValue;
}

pub async fn upload_file_to_pinata(file: File, api_key: &str, api_secret: &str) -> Result<String> {
    let result = upload_to_pinata(api_key, api_secret, JsValue::from(file))
        .await
        .as_string()
        .ok_or_else(|| anyhow::anyhow!("Failed to get response from Pinata"))?;

    Ok(result)
}

pub async fn upload_metadata_to_pinata(metadata: Value, api_key: &str, api_secret: &str) -> Result<String> {
    let js_metadata = serde_wasm_bindgen::to_value(&metadata)
        .map_err(|e| anyhow::anyhow!("Failed to serialize metadata: {}", e))?;

    let result = upload_to_pinata(api_key, api_secret, js_metadata)
        .await
        .as_string()
        .ok_or_else(|| anyhow::anyhow!("Failed to get response from Pinata"))?;

    Ok(result)
} 
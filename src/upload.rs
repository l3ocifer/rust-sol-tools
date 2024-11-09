use web_sys::File;
use serde_json::Value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use gloo_utils::format::JsValueSerdeExt;
use anyhow::Result;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(module = "/public/pinata.js")]
extern "C" {
    #[wasm_bindgen(js_name = uploadToPinata)]
    pub async fn upload_to_pinata(api_key: &str, api_secret: &str, data: JsValue) -> JsValue;
}

#[cfg(target_arch = "wasm32")]
pub async fn upload_file(file: File) -> Result<String, JsValue> {
    let api_key = get_api_key()?;
    let api_secret = get_api_secret()?;
    
    let result = upload_to_pinata(&api_key, &api_secret, file.into())
        .await
        .as_string()
        .ok_or_else(|| anyhow::anyhow!("Failed to get response from Pinata"))?;

    Ok(result)
}

#[cfg(target_arch = "wasm32")]
pub async fn upload_metadata(metadata: Value) -> Result<String> {
    let api_key = get_api_key()?;
    let api_secret = get_api_secret()?;

    let metadata_js = JsValue::from_serde(&metadata)?;

    let result_js = upload_to_pinata(&api_key, &api_secret, metadata_js).await;
    let result_str = result_js.as_string().ok_or_else(|| anyhow::anyhow!("Failed to get response from Pinata"))?;

    Ok(result_str)
}

#[cfg(target_arch = "wasm32")]
fn get_api_key() -> Result<String> {
    // Retrieve API key from a secure source or configuration
    Ok("your_api_key".to_string())
}

#[cfg(target_arch = "wasm32")]
fn get_api_secret() -> Result<String> {
    // Retrieve API secret from a secure source or configuration
    Ok("your_api_secret".to_string())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn upload_file() {
    // This module is intended for wasm32 target only.
    // No implementation needed for non-wasm targets.
}
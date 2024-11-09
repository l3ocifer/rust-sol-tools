use web_sys::File;
use serde_json::Value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use gloo_utils::format::JsValueSerdeExt;
use anyhow::Result;

#[wasm_bindgen(module = "/public/pinata.js")]
extern "C" {
    #[wasm_bindgen(js_name = uploadToPinata)]
    pub async fn upload_to_pinata(api_key: &str, api_secret: &str, data: JsValue) -> JsValue;
}

pub async fn upload_image(file: File) -> Result<String> {
    let api_key = std::env::var("PINATA_API_KEY")?;
    let api_secret = std::env::var("PINATA_API_SECRET")?;
    
    let result = upload_to_pinata(&api_key, &api_secret, file.into())
        .await
        .as_string()
        .ok_or_else(|| anyhow::anyhow!("Failed to get response from Pinata"))?;

    Ok(result)
}

pub async fn upload_metadata(metadata: Value) -> Result<String> {
    let api_key = std::env::var("PINATA_API_KEY")?;
    let api_secret = std::env::var("PINATA_API_SECRET")?;

    let metadata_js = JsValue::from_serde(&metadata)?;

    let result_js = upload_to_pinata(&api_key, &api_secret, metadata_js).await;
    let result_str = result_js.as_string().ok_or_else(|| anyhow::anyhow!("Failed to get response from Pinata"))?;

    Ok(result_str)
} 
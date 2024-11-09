use anyhow::Result;
use web_sys::File;

#[cfg(target_arch = "wasm32")]
pub async fn upload_file_to_pinata(file: File, api_key: &str, api_secret: &str) -> Result<String> {
    let window = web_sys::window().ok_or_else(|| anyhow::anyhow!("No window object"))?;
    let pinata = js_sys::Reflect::get(&window, &JsValue::from_str("uploadToPinata"))
        .map_err(|_| anyhow::anyhow!("uploadToPinata function not found"))?;

    let form_data = web_sys::FormData::new()
        .map_err(|_| anyhow::anyhow!("Failed to create FormData"))?;
    form_data.append_with_blob("file", &file)
        .map_err(|_| anyhow::anyhow!("Failed to append file to FormData"))?;

    let promise = js_sys::Function::from(pinata)
        .call2(
            &JsValue::NULL,
            &JsValue::from_str(api_key),
            &JsValue::from_str(api_secret),
        )
        .map_err(|_| anyhow::anyhow!("Failed to call uploadToPinata"))?;

    let result = wasm_bindgen_futures::JsFuture::from(js_sys::Promise::from(promise))
        .await
        .map_err(|e| anyhow::anyhow!("Upload failed: {:?}", e))?;

    result
        .as_string()
        .ok_or_else(|| anyhow::anyhow!("Invalid response format"))
}

#[cfg(not(target_arch = "wasm32"))]
pub mod pinata_client {
    use reqwest::multipart::{Form, Part};
    use anyhow::Result;
    use serde_json::Value;

    pub async fn upload_metadata_to_pinata(api_key: &str, api_secret: &str, metadata: &Value) -> Result<String> {
        let client = reqwest::Client::new();
        let json_str = serde_json::to_string(metadata)?;
        let part = Part::text(json_str).file_name("metadata.json");
        let form = Form::new().part("file", part);

        let response = client
            .post("https://api.pinata.cloud/pinning/pinFileToIPFS")
            .header("pinata_api_key", api_key)
            .header("pinata_secret_api_key", api_secret)
            .multipart(form)
            .send()
            .await?;

        let result: Value = response.json().await?;
        let ipfs_hash = result["IpfsHash"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid response from Pinata"))?;

        Ok(format!("https://gateway.pinata.cloud/ipfs/{}", ipfs_hash))
    }
} 
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::File;
use anyhow::Result;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(module = "/public/pinata.js")]
extern "C" {
    #[wasm_bindgen(js_name = uploadToPinata)]
    pub async fn upload_to_pinata(api_key: &str, api_secret: &str, data: JsValue) -> JsValue;
}

#[cfg(target_arch = "wasm32")]
pub async fn upload_file(file: File) -> Result<String, String> {
    // Implementation for browser environment
    let form_data = web_sys::FormData::new()
        .map_err(|_| "Failed to create FormData")?;
    
    form_data.append_with_blob("file", &file.into())
        .map_err(|_| "Failed to append file")?;

    let request = web_sys::Request::new_with_str_and_init(
        "https://api.pinata.cloud/pinning/pinFileToIPFS",
        web_sys::RequestInit::new()
            .method("POST")
            .body(Some(&form_data.into()))
    ).map_err(|_| "Failed to create request")?;

    let window = web_sys::window().ok_or("No window found")?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| "Failed to fetch")?;

    let resp: web_sys::Response = resp_value.dyn_into()
        .map_err(|_| "Failed to convert response")?;

    if !resp.ok() {
        return Err(format!("HTTP error! status: {}", resp.status()));
    }

    let json = JsFuture::from(resp.json().map_err(|_| "Failed to parse JSON")?)
        .await
        .map_err(|_| "Failed to await JSON")?;

    let result: js_sys::Object = json.dyn_into()
        .map_err(|_| "Failed to convert to Object")?;
    
    let ipfs_hash = js_sys::Reflect::get(&result, &JsValue::from_str("IpfsHash"))
        .map_err(|_| "Failed to get IpfsHash")?
        .as_string()
        .ok_or("Invalid IpfsHash format")?;

    Ok(format!("https://gateway.pinata.cloud/ipfs/{}", ipfs_hash))
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn upload_file(_file: Vec<u8>) -> Result<String, String> {
    Err("File upload not supported in server environment".to_string())
}

#[cfg(target_arch = "wasm32")]
pub async fn upload_metadata(metadata: JsValue) -> Result<String> {
    let api_key = get_api_key()?;
    let api_secret = get_api_secret()?;

    let result_js = upload_to_pinata(&api_key, &api_secret, metadata).await;
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
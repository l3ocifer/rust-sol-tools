#[cfg(target_arch = "wasm32")]
use {
    wasm_bindgen::prelude::*,
    wasm_bindgen_futures::JsFuture,
    web_sys::File,
    js_sys::{Object, Reflect},
};
use anyhow::Result;

#[cfg(target_arch = "wasm32")]
pub async fn upload_file(file: File) -> Result<String, String> {
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

    let result: Object = json.dyn_into()
        .map_err(|_| "Failed to convert to Object")?;
    
    let ipfs_hash = Reflect::get(&result, &JsValue::from_str("IpfsHash"))
        .map_err(|_| "Failed to get IpfsHash")?
        .as_string()
        .ok_or("Invalid IpfsHash format")?;

    Ok(format!("https://gateway.pinata.cloud/ipfs/{}", ipfs_hash))
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn upload_file(_file: Vec<u8>) -> Result<String, String> {
    Err("File upload not supported in server environment".to_string())
}
use serde::{Serialize, Deserialize};
use anyhow::Result;

#[cfg(target_arch = "wasm32")]
use reqwasm::http::Request;

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub description: String,
    pub image: String,
}

pub async fn upload_file_to_pinata(file: web_sys::File, api_key: &str, api_secret: &str) -> Result<String> {
    #[cfg(target_arch = "wasm32")]
    {
        let url = "https://api.pinata.cloud/pinning/pinFileToIPFS";
        let mut form = web_sys::FormData::new().unwrap();
        form.append_with_blob("file", &file)?;

        let response = Request::new(url)
            .method("POST")
            .header("pinata_api_key", api_key)
            .header("pinata_secret_api_key", api_secret)
            .body(form)
            .send()
            .await?;
        
        if response.ok() {
            let result = response.json::<serde_json::Value>().await?;
            let ipfs_hash = result["IpfsHash"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid response from Pinata"))?;
            Ok(format!("https://gateway.pinata.cloud/ipfs/{}", ipfs_hash))
        } else {
            Err(anyhow::anyhow!("Failed to upload file to Pinata"))
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        Err(anyhow::anyhow!("Not implemented for non-wasm targets"))
    }
} 
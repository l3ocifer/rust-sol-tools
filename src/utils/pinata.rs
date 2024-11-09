use serde_json::Value;

#[cfg(target_arch = "wasm32")]
use reqwasm::http::Request;

#[cfg(not(target_arch = "wasm32"))]
use reqwest::Client;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub description: String,
    pub image: String,
    // Add any additional fields as needed
}

pub async fn upload_file_to_pinata(file: web_sys::File, api_key: &str, api_secret: &str) -> anyhow::Result<String> {
    #[cfg(target_arch = "wasm32")]
    {
        // Use reqwasm for WASM target
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
            let result = response.json::<Value>().await?;
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
        // Use reqwest for native target
        // ... existing code ...
        Ok(String::new()) // Placeholder
    }
}

#[cfg(feature = "ssr")]
pub mod pinata_client {
    use anyhow::Result;
    use reqwest::Client;
    use serde_json::Value;

    pub async fn upload_metadata_to_pinata(
        api_key: &str,
        api_secret: &str,
        metadata: &Value,
    ) -> Result<String> {
        let client = Client::new();
        let url = "https://api.pinata.cloud/pinning/pinJSONToIPFS";

        let res = client
            .post(url)
            .header("pinata_api_key", api_key)
            .header("pinata_secret_api_key", api_secret)
            .json(metadata)
            .send()
            .await?;

        let json: Value = res.json().await?;
        let ipfs_hash = json["IpfsHash"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid response from Pinata"))?;

        Ok(format!("https://gateway.pinata.cloud/ipfs/{}", ipfs_hash))
    }
}

#[cfg(target_arch = "wasm32")]
pub async fn upload_metadata_to_pinata(metadata: &Metadata, api_key: &str, api_secret: &str) -> anyhow::Result<String> {
    // Client-side implementation using reqwasm
    let url = "https://api.pinata.cloud/pinning/pinJSONToIPFS";

    let response = Request::post(url)
        .header("Content-Type", "application/json")
        .header("pinata_api_key", api_key)
        .header("pinata_secret_api_key", api_secret)
        .body(serde_json::to_string(metadata)?)
        .send()
        .await?;

    if response.ok() {
        let result = response.json::<serde_json::Value>().await?;
        let ipfs_hash = result["IpfsHash"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid response from Pinata"))?;
        Ok(format!("https://gateway.pinata.cloud/ipfs/{}", ipfs_hash))
    } else {
        Err(anyhow::anyhow!("Failed to upload metadata to Pinata"))
    }
} 
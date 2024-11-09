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

pub async fn upload_metadata_to_pinata(metadata: &Metadata, api_key: &str, api_secret: &str) -> anyhow::Result<String> {
    #[cfg(target_arch = "wasm32")]
    {
        // Use reqwasm for WASM target
        let url = "https://api.pinata.cloud/pinning/pinJSONToIPFS";

        let response = reqwasm::http::Request::post(url)
            .header("Content-Type", "application/json")
            .header("pinata_api_key", api_key)
            .header("pinata_secret_api_key", api_secret)
            .body(serde_json::to_string(metadata)?)
            .send()
            .await?;

        if response.status() == 200 {
            let result: serde_json::Value = response.json().await?;
            let ipfs_hash = result["IpfsHash"].as_str().ok_or_else(|| anyhow::anyhow!("Invalid response from Pinata"))?;
            Ok(format!("https://gateway.pinata.cloud/ipfs/{}", ipfs_hash))
        } else {
            Err(anyhow::anyhow!("Failed to upload metadata to Pinata"))
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // Use reqwest for native target
        let client = Client::new();
        let url = "https://api.pinata.cloud/pinning/pinJSONToIPFS";

        let response = client.post(url)
            .header("Content-Type", "application/json")
            .header("pinata_api_key", api_key)
            .header("pinata_secret_api_key", api_secret)
            .json(metadata)
            .send()
            .await?;

        if response.status().is_success() {
            let result: serde_json::Value = response.json().await?;
            let ipfs_hash = result["IpfsHash"].as_str().ok_or_else(|| anyhow::anyhow!("Invalid response from Pinata"))?;
            Ok(format!("https://gateway.pinata.cloud/ipfs/{}", ipfs_hash))
        } else {
            Err(anyhow::anyhow!("Failed to upload metadata to Pinata"))
        }
    }
} 
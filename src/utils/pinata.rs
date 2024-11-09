use serde_json::Value;

#[cfg(target_arch = "wasm32")]
use reqwasm::http::Request;

#[cfg(not(target_arch = "wasm32"))]
use reqwest::Client;

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
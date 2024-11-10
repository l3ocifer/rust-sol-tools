#[cfg(target_arch = "wasm32")]
pub mod wasm {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/public/pinata.js")]
    extern "C" {
        #[wasm_bindgen(js_name = uploadToPinata)]
        pub async fn upload_to_pinata(api_key: &str, api_secret: &str, data: JsValue) -> JsValue;
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub mod server {
    use reqwest::Client;
    use anyhow::Result;
    use serde_json::Value;
    use reqwest::multipart::{Form, Part};

    pub async fn upload_metadata_to_pinata(api_key: &str, api_secret: &str, metadata: &Value) -> Result<String> {
        let client = Client::new();
        let json_str = serde_json::to_string(metadata)?;
        let part = Part::text(json_str).file_name("metadata.json");
        let form = Form::new().part("file", part);

        let result = client
            .post("https://api.pinata.cloud/pinning/pinFileToIPFS")
            .header("pinata_api_key", api_key)
            .header("pinata_secret_api_key", api_secret)
            .multipart(form)
            .send()
            .await?
            .json::<Value>()
            .await?;

        let ipfs_hash = result["IpfsHash"].as_str()
            .ok_or_else(|| anyhow::anyhow!("Failed to get IPFS hash"))?;

        Ok(format!("ipfs://{}", ipfs_hash))
    }
} 
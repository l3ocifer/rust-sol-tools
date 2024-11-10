#[cfg(target_arch = "wasm32")]
pub mod wasm {
    use wasm_bindgen::prelude::*;
    use serde_json::Value;
    use js_sys::{Promise, Error, Reflect};
    use wasm_bindgen_futures::JsFuture;
    use std::error::Error as StdError;

    #[wasm_bindgen(module = "/public/pinata.js")]
    extern "C" {
        #[wasm_bindgen(js_name = uploadToPinata)]
        pub async fn upload_to_pinata(api_key: &str, api_secret: &str, data: &JsValue) -> Promise;
    }

    #[derive(Debug)]
    struct JsError(String);

    impl std::fmt::Display for JsError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl StdError for JsError {}

    pub async fn upload_metadata_to_pinata(api_key: &str, api_secret: &str, metadata: &Value) -> Result<String, Box<dyn StdError>> {
        let js_metadata = serde_wasm_bindgen::to_value(metadata)?;
        let promise = upload_to_pinata(api_key, api_secret, &js_metadata);
        let result = JsFuture::from(promise)
            .await
            .map_err(|e| Box::new(JsError(e.as_string().unwrap_or_default())) as Box<dyn StdError>)?;
        
        let ipfs_hash = Reflect::get(&result, &"IpfsHash".into())
            .map_err(|e| Box::new(JsError(e.as_string().unwrap_or_default())))?
            .as_string()
            .ok_or_else(|| Box::new(JsError("Failed to get IPFS hash".to_string())))?;
            
        Ok(format!("ipfs://{}", ipfs_hash))
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

#[cfg(target_arch = "wasm32")]
pub use self::wasm::upload_metadata_to_pinata;
#[cfg(not(target_arch = "wasm32"))]
pub use self::server::upload_metadata_to_pinata;
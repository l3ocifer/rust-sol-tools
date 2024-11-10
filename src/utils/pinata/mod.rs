#[cfg(target_arch = "wasm32")]
pub mod wasm {
    use wasm_bindgen::prelude::*;
    use serde_json::Value;
    use js_sys::{Promise, Reflect};
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

    pub async fn upload_metadata_to_pinata(
        api_key: &str,
        secret_key: &str,
        metadata: &serde_json::Value,
    ) -> Result<String, String> {
        #[cfg(target_arch = "wasm32")]
        {
            let window = web_sys::window().ok_or("No window object found")?;
            let fetch = window.fetch_with_request(&request)?;
            
            // Convert the Promise directly to JsFuture
            let response = JsFuture::from(fetch)
                .await
                .map_err(|e| format!("Failed to fetch: {:?}", e))?;

            let response: web_sys::Response = response
                .dyn_into()
                .map_err(|_| "Failed to convert response")?;

            // Convert the Promise returned by text() to JsFuture
            let text = JsFuture::from(
                response
                    .text()
                    .map_err(|_| "Failed to get response text")?
            )
            .await
            .map_err(|_| "Failed to read response text")?;

            let ipfs_hash = text
                .as_string()
                .ok_or("Invalid response format")?;

            Ok(format!("https://gateway.pinata.cloud/ipfs/{}", ipfs_hash))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Err("Pinata upload not supported in non-WASM environment".to_string())
        }
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
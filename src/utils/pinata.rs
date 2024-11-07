use dotenv::dotenv;
use reqwest::Client;
use serde::Serialize;
use std::env;

#[derive(Serialize)]
pub struct Metadata {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub image: String,
}

pub async fn upload_metadata_to_pinata(metadata: &Metadata) -> Result<String, Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("PINATA_API_KEY")?;
    let secret_api_key = env::var("PINATA_SECRET_API_KEY")?;

    let client = Client::new();
    let res = client
        .post("https://api.pinata.cloud/pinning/pinJSONToIPFS")
        .header("pinata_api_key", api_key)
        .header("pinata_secret_api_key", secret_api_key)
        .json(metadata)
        .send()
        .await?;

    let res_json: serde_json::Value = res.json().await?;
    let ipfs_hash = res_json["IpfsHash"].as_str().ok_or("Failed to get IPFS hash")?;
    Ok(format!("ipfs://{}", ipfs_hash))
} 
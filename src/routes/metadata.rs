use actix_web::{post, web, HttpResponse};
use crate::utils::pinata::{upload_metadata_to_pinata, Metadata};

#[post("/upload-metadata")]
pub async fn upload_metadata(metadata: web::Json<Metadata>) -> HttpResponse {
    // Get API credentials from environment variables
    let api_key = std::env::var("PINATA_API_KEY").expect("PINATA_API_KEY not set");
    let api_secret = std::env::var("PINATA_API_SECRET").expect("PINATA_API_SECRET not set");

    match upload_metadata_to_pinata(&metadata, &api_key, &api_secret).await {
        Ok(url) => HttpResponse::Ok().json(serde_json::json!({ "url": url })),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
} 
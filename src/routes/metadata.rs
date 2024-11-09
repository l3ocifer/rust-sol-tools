#[cfg(not(target_arch = "wasm32"))]
use actix_web::{post, web, HttpResponse};
#[cfg(not(target_arch = "wasm32"))]
use crate::utils::pinata::pinata_client::upload_metadata_to_pinata;

#[cfg(not(target_arch = "wasm32"))]
#[post("/upload-metadata")]
pub async fn upload_metadata(
    metadata: web::Json<serde_json::Value>,
    api_keys: web::Data<(String, String)>,
) -> HttpResponse {
    match upload_metadata_to_pinata(&api_keys.0, &api_keys.1, &metadata.0).await {
        Ok(url) => HttpResponse::Ok().json(serde_json::json!({ "url": url })),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
} 
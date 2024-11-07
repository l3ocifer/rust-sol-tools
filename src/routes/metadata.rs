use crate::utils::pinata::{upload_metadata_to_pinata, Metadata};

#[cfg(feature = "ssr")]
use actix_web::{post, web, HttpResponse, Responder};

#[cfg(feature = "ssr")]
#[post("/upload-metadata")]
pub async fn upload_metadata(metadata: web::Json<Metadata>) -> impl Responder {
    match upload_metadata_to_pinata(&metadata).await {
        Ok(uri) => HttpResponse::Ok().json(serde_json::json!({ "uri": uri })),
        Err(e) => {
            eprintln!("Error uploading metadata: {}", e);
            HttpResponse::InternalServerError().body("Failed to upload metadata")
        }
    }
} 
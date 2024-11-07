#[cfg(feature = "ssr")]
use actix_web::{post, web, HttpResponse, Responder};
use crate::utils::contract::create_token;

#[cfg(feature = "ssr")]
#[derive(serde::Deserialize)]
pub struct CreateTokenRequest {
    pub metadata_uri: String,
}

#[cfg(feature = "ssr")]
#[post("/create-token")]
pub async fn create_token_route(req: web::Json<CreateTokenRequest>) -> impl Responder {
    match create_token(req.metadata_uri.clone()).await {
        Ok(signature) => HttpResponse::Ok().json(&serde_json::json!({ "signature": signature })),
        Err(e) => {
            eprintln!("Error creating token: {}", e);
            HttpResponse::InternalServerError().body("Failed to create token")
        }
    }
} 
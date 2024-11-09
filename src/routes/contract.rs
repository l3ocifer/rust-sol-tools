#[cfg(feature = "ssr")]
use actix_web::{post, web, HttpResponse, Responder};

#[cfg(feature = "ssr")]
#[derive(serde::Deserialize)]
pub struct CreateTokenRequest {
    pub metadata_uri: String,
}

#[cfg(feature = "ssr")]
#[post("/create-token")]
pub async fn create_token_route(req: web::Json<CreateTokenRequest>) -> impl Responder {
    use crate::token::create_token;

    let params = crate::token::CreateTokenParams {
        name: "TokenName".to_string(),
        symbol: "SYMBOL".to_string(),
        description: "Description".to_string(),
        metadata_uri: req.metadata_uri.clone(),
        decimals: 9,
        initial_supply: 1000,
        is_mutable: true,
        freeze_authority: false,
        rate_limit: None,
        transfer_fee: None,
        max_transfer_amount: None,
        #[cfg(not(target_arch = "wasm32"))]
        payer: Some(crate::utils::load_keypair()?),
    };

    match create_token(params).await {
        Ok(result) => HttpResponse::Ok().json(&serde_json::json!(result)),
        Err(e) => {
            eprintln!("Error creating token: {}", e);
            HttpResponse::InternalServerError().body("Failed to create token")
        }
    }
} 
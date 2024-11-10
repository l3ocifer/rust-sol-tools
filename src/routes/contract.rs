#[cfg(not(target_arch = "wasm32"))]
use actix_web::{post, web, HttpResponse, Responder};

#[cfg(not(target_arch = "wasm32"))]
#[derive(serde::Deserialize)]
pub struct CreateTokenRequest {
    pub metadata_uri: String,
}

#[cfg(not(target_arch = "wasm32"))]
#[post("/create-token")]
pub async fn create_token_route(req: web::Json<CreateTokenRequest>) -> impl Responder {
    use crate::token::create_token;
    use crate::token::NetworkType;
    use solana_sdk::signer::keypair::Keypair;

    let _payer = match crate::utils::load_env_keypair("SOLANA_KEYPAIR_PATH") {
        Ok(keypair) => Some(keypair),
        Err(e) => {
            eprintln!("Error loading keypair: {}", e);
            return HttpResponse::InternalServerError().body("Failed to load keypair");
        }
    };

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
        network: NetworkType::Devnet,
        #[cfg(not(target_arch = "wasm32"))]
        payer: None::<Keypair>,
    };

    match create_token(params).await {
        Ok(result) => HttpResponse::Ok().json(&serde_json::json!(result)),
        Err(e) => {
            eprintln!("Error creating token: {}", e);
            HttpResponse::InternalServerError().body("Failed to create token")
        }
    }
} 
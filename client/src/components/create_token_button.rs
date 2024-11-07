use leptos::*;
use crate::token::TokenCreationResult;

#[server(CreateToken, "/api")]
pub async fn create_token(metadata_uri: String) -> Result<TokenCreationResult, ServerFnError> {
    // Server-side implementation
    crate::utils::contract::create_token(metadata_uri).await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[component]
pub fn CreateTokenButton(metadata_uri: String) -> impl IntoView {
    let on_click = move |_| {
        spawn_local(async move {
            match create_token(metadata_uri).await {
                Ok(result) => {
                    web_sys::window()
                        .unwrap()
                        .alert_with_message(&format!(
                            "Token created!\nMint: {}\nTransaction: {}\nExplorer: {}", 
                            result.mint, result.signature, result.explorer_url
                        ))
                        .unwrap();
                }
                Err(e) => {
                    web_sys::window()
                        .unwrap()
                        .alert_with_message(&format!("Error: {}", e))
                        .unwrap();
                }
            }
        });
    };

    view! {
        <button on:click=on_click>
            "Create Token"
        </button>
    }
} 
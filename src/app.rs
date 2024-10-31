use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use web_sys::{File, Event, SubmitEvent, HtmlInputElement};
use crate::wallet::{WalletProvider, WalletContext, WalletType};
use crate::upload::{upload_image, upload_metadata};
use crate::token::create_token;
use serde_json::json;
use wasm_bindgen::JsCast;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/sol-tools.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Title text="Token Tools"/>

        <WalletProvider>
            <Router>
                <main>
                    <nav class="header">
                        <div class="header-content">
                            <A href="/" class="home-link">"🏠 Home"</A>
                            <h1>"Token Tools"</h1>
                            <WalletConnect/>
                        </div>
                    </nav>

                    <Routes>
                        <Route path="/" view=HomePage/>
                        <Route path="/create" view=CreateTokenPage/>
                        <Route path="/send" view=SendTokenPage/>
                    </Routes>
                </main>
            </Router>
        </WalletProvider>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="container">
            <h2>"Token Management"</h2>
            <div class="features">
                <div class="feature-card">
                    <h3>"Create Token"</h3>
                    <p>"Create new token with metadata"</p>
                    <A href="/create" class="button">"Create"</A>
                </div>
                
                <div class="feature-card">
                    <h3>"Send Token"</h3>
                    <p>"Send tokens to any address"</p>
                    <A href="/send" class="button">"Send"</A>
                </div>
            </div>
        </div>
    }
}

#[component]
fn CreateTokenPage() -> impl IntoView {
    let wallet_ctx = use_context::<WalletContext>().expect("WalletContext not found");
    let (token_name, set_token_name) = create_signal(String::new());
    let (token_symbol, set_token_symbol) = create_signal(String::new());
    let (token_description, set_token_description) = create_signal(String::new());
    let (token_image, set_token_image) = create_signal(None::<File>);
    let (metadata_uri, set_metadata_uri) = create_signal(String::new());
    let (decimals, set_decimals) = create_signal(9u8);
    let (initial_supply, set_initial_supply) = create_signal(1_000_000_000u64);
    let (is_mutable, set_is_mutable) = create_signal(true);
    let (freeze_authority, set_freeze_authority) = create_signal(true);
    let (loading, set_loading) = create_signal(false);
    let (error, set_error) = create_signal(Option::<String>::None);
    let (success, set_success) = create_signal(Option::<String>::None);

    let handle_image_upload = move |ev: Event| {
        let input: HtmlInputElement = ev.target().unwrap().unchecked_into();
        if let Some(files) = input.files() {
            if let Some(file) = files.get(0) {
                set_token_image.set(Some(file));
            }
        }
    };

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        if !wallet_ctx.state.get().connected {
            set_error.set(Some("Please connect your wallet first".to_string()));
            return;
        }

        if metadata_uri.get().is_empty() && token_image.get().is_none() {
            set_error.set(Some("Either metadata URI or image file is required".to_string()));
            return;
        }

        set_loading.set(true);
        set_error.set(None);
        set_success.set(None);

        spawn_local(async move {
            let metadata_url = if !metadata_uri.get().is_empty() {
                metadata_uri.get()
            } else if let Some(image_file) = token_image.get() {
                match upload_image(image_file).await {
                    Ok(image_url) => {
                        let metadata = json!({
                            "name": token_name.get(),
                            "symbol": token_symbol.get(),
                            "description": token_description.get(),
                            "image": image_url,
                            "attributes": [],
                            "properties": {
                                "files": [{
                                    "uri": image_url,
                                    "type": "image/png"
                                }]
                            }
                        });

                        match upload_metadata(metadata).await {
                            Ok(url) => url,
                            Err(e) => {
                                set_error.set(Some(format!("Failed to upload metadata: {}", e)));
                                set_loading.set(false);
                                return;
                            }
                        }
                    }
                    Err(e) => {
                        set_error.set(Some(format!("Failed to upload image: {}", e)));
                        set_loading.set(false);
                        return;
                    }
                }
            } else {
                set_error.set(Some("Either metadata URI or image file is required".to_string()));
                set_loading.set(false);
                return;
            };

            set_success.set(Some("Creating token...".to_string()));
            
            let result = create_token(CreateTokenParams {
                name: token_name.get(),
                symbol: token_symbol.get(),
                description: token_description.get(),
                metadata_uri: metadata_url,
                decimals: decimals.get(),
                initial_supply: initial_supply.get(),
                is_mutable: is_mutable.get(),
                freeze_authority: freeze_authority.get(),
            }).await;

            match result {
                Ok(signature) => {
                    set_success.set(Some(format!("Token created successfully! Transaction: {}", signature)));
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to create token: {}", e)));
                }
            }
            set_loading.set(false);
        });
    };

    view! {
        <div class="container">
            <h2>"Create New Token"</h2>
            
            <form class="token-form" on:submit=on_submit>
                <div class="form-group">
                    <label for="token_name">"Token Name"</label>
                    <input
                        type="text"
                        id="token_name"
                        required
                        placeholder="Enter token name"
                        on:input=move |ev| {
                            set_token_name.set(event_target_value(&ev));
                        }
                    />
                </div>

                <div class="form-group">
                    <label for="token_symbol">"Token Symbol"</label>
                    <input
                        type="text"
                        id="token_symbol"
                        required
                        placeholder="Enter token symbol"
                        on:input=move |ev| {
                            set_token_symbol.set(event_target_value(&ev));
                        }
                    />
                </div>

                <div class="form-group">
                    <label for="token_description">"Token Description"</label>
                    <textarea
                        id="token_description"
                        required
                        placeholder="Enter token description"
                        on:input=move |ev| {
                            set_token_description.set(event_target_value(&ev));
                        }
                    />
                </div>

                <div class="form-group">
                    <label for="metadata_uri">"Metadata URI"</label>
                    <input
                        type="text"
                        id="metadata_uri"
                        placeholder="Enter metadata URI from Pinata or similar service"
                        on:input=move |ev| {
                            let uri = event_target_value(&ev);
                            set_metadata_uri.set(uri.clone());
                            if !uri.is_empty() {
                                set_token_image.set(None);
                            }
                        }
                    />
                </div>

                <div class="form-group" class:hidden=move || !metadata_uri.get().is_empty()>
                    <label for="token_image">"Token Image"</label>
                    <input
                        type="file"
                        id="token_image"
                        accept="image/*"
                        on:change=handle_image_upload
                    />
                </div>

                <div class="form-group">
                    <label for="decimals">"Decimals (0-9)"</label>
                    <input
                        type="number"
                        id="decimals"
                        min="0"
                        max="9"
                        required
                        value="9"
                        on:input=move |ev| {
                            if let Ok(value) = event_target_value(&ev).parse() {
                                set_decimals.set(value);
                            }
                        }
                    />
                </div>

                <div class="form-group">
                    <label for="initial_supply">"Initial Supply"</label>
                    <input
                        type="number"
                        id="initial_supply"
                        min="0"
                        required
                        value="1000000000"
                        on:input=move |ev| {
                            if let Ok(value) = event_target_value(&ev).parse() {
                                set_initial_supply.set(value);
                            }
                        }
                    />
                </div>

                <div class="form-group checkbox-group">
                    <label>
                        <input
                            type="checkbox"
                            checked=true
                            on:change=move |ev| {
                                set_is_mutable.set(event_target_checked(&ev));
                            }
                        />
                        "Token metadata is mutable"
                    </label>
                </div>

                <div class="form-group checkbox-group">
                    <label>
                        <input
                            type="checkbox"
                            checked=true
                            on:change=move |ev| {
                                set_freeze_authority.set(event_target_checked(&ev));
                            }
                        />
                        "Enable freeze authority"
                    </label>
                </div>

                {move || error.get().map(|err| view! {
                    <div class="error-message">
                        {err}
                    </div>
                })}

                {move || success.get().map(|msg| view! {
                    <div class="success-message">
                        {msg}
                    </div>
                })}

                <button 
                    type="submit" 
                    class="button"
                    disabled=move || loading.get()
                >
                    {move || if loading.get() {
                        "Creating Token..."
                    } else if wallet_ctx.state.get().connected {
                        "Create Token"
                    } else {
                        "Connect Wallet First"
                    }}
                </button>
            </form>
        </div>
    }
}

#[component]
fn SendTokenPage() -> impl IntoView {
    let wallet_ctx = use_context::<WalletContext>().expect("WalletContext not found");
    let (token_address, set_token_address) = create_signal(String::new());
    let (recipient_address, set_recipient_address) = create_signal(String::new());
    let (amount, set_amount) = create_signal(0u64);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        if !wallet_ctx.state.get().connected {
            logging::warn!("Please connect your wallet first");
            return;
        }
        logging::log!(
            "Sending {} tokens to {} from {}", 
            amount.get(),
            recipient_address.get(),
            token_address.get()
        );
    };

    view! {
        <div class="container">
            <h2>"Send Tokens"</h2>
            
            <form class="token-form" on:submit=on_submit>
                <div class="form-group">
                    <label for="token_address">"Token Address"</label>
                    <input
                        type="text"
                        id="token_address"
                        required
                        placeholder="Enter token address"
                        on:input=move |ev| {
                            set_token_address.set(event_target_value(&ev));
                        }
                    />
                </div>

                <div class="form-group">
                    <label for="recipient_address">"Recipient Address"</label>
                    <input
                        type="text"
                        id="recipient_address"
                        required
                        placeholder="Enter recipient address"
                        on:input=move |ev| {
                            set_recipient_address.set(event_target_value(&ev));
                        }
                    />
                </div>

                <div class="form-group">
                    <label for="amount">"Amount"</label>
                    <input
                        type="number"
                        id="amount"
                        min="1"
                        required
                        placeholder="Enter amount to send"
                        on:input=move |ev| {
                            if let Ok(value) = event_target_value(&ev).parse() {
                                set_amount.set(value);
                            }
                        }
                    />
                </div>

                <button type="submit" class="button">
                    {move || if wallet_ctx.state.get().connected {
                        "Send Tokens"
                    } else {
                        "Connect Wallet First"
                    }}
                </button>
            </form>
        </div>
    }
}

#[component]
fn WalletConnect() -> impl IntoView {
    let wallet_ctx = use_context::<WalletContext>().expect("WalletContext not found");
    let state = wallet_ctx.state;
    
    let wallet_ctx_phantom = wallet_ctx.clone();
    let connect_phantom = create_action(move |_: &()| {
        let ctx = wallet_ctx_phantom.clone();
        async move {
            ctx.connect(WalletType::Phantom).await;
        }
    });
    
    let wallet_ctx_metamask = wallet_ctx.clone();
    let connect_metamask = create_action(move |_: &()| {
        let ctx = wallet_ctx_metamask.clone();
        async move {
            ctx.connect(WalletType::MetaMask).await;
        }
    });
    
    let wallet_ctx_disconnect = wallet_ctx.clone();
    let disconnect = create_action(move |_: &()| {
        let ctx = wallet_ctx_disconnect.clone();
        async move {
            ctx.disconnect().await;
        }
    });

    view! {
        <div class="wallet-connect">
            {move || if state.get().connected {
                view! {
                    <div class="wallet-info">
                        <span class="wallet-address">{state.get().address.unwrap_or_default()}</span>
                        <button class="button" on:click=move |_| disconnect.dispatch(())>
                            "Disconnect"
                        </button>
                    </div>
                }
            } else {
                view! {
                    <div class="wallet-buttons">
                        <button class="button" on:click=move |_| connect_phantom.dispatch(())>
                            "Connect Phantom"
                        </button>
                        <button class="button" on:click=move |_| connect_metamask.dispatch(())>
                            "Connect MetaMask"
                        </button>
                    </div>
                }
            }}
        </div>
    }
}

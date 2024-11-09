use leptos::*;
use leptos_meta::*;
use leptos_router::*;
#[cfg(target_arch = "wasm32")]
use leptos::ev::SubmitEvent;
use crate::wallet::{WalletProvider, WalletContext, WalletType};
use crate::token::{create_token, CreateTokenParams};

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
    let (metadata_uri, set_metadata_uri) = create_signal(String::new());
    let (decimals, set_decimals) = create_signal(9u8);
    let (initial_supply, set_initial_supply) = create_signal(1_000_000_000u64);
    let (is_mutable, set_is_mutable) = create_signal(true);
    let (freeze_authority, set_freeze_authority) = create_signal(true);
    let (rate_limit, set_rate_limit) = create_signal(Option::<u64>::None);
    let (transfer_fee, set_transfer_fee) = create_signal(Option::<u16>::None);
    let (max_transfer_amount, set_max_transfer_amount) = create_signal(Option::<u64>::None);
    let (loading, set_loading) = create_signal(false);
    let (error, set_error) = create_signal(Option::<String>::None);
    let (success, set_success) = create_signal(Option::<String>::None);
    let (status, set_status) = create_signal(String::new());

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        if !wallet_ctx.state.get().connected {
            set_error.set(Some("Please connect your wallet first".to_string()));
            return;
        }

        set_loading.set(true);
        set_error.set(None);
        set_success.set(None);

        spawn_local(async move {
            let params = CreateTokenParams {
                name: token_name.get_untracked(),
                symbol: token_symbol.get_untracked(),
                description: token_description.get_untracked(),
                metadata_uri: metadata_uri.get_untracked(),
                decimals: decimals.get_untracked(),
                initial_supply: initial_supply.get_untracked(),
                is_mutable: is_mutable.get_untracked(),
                freeze_authority: freeze_authority.get_untracked(),
                rate_limit: rate_limit.get_untracked(),
                transfer_fee: transfer_fee.get_untracked(),
                max_transfer_amount: max_transfer_amount.get_untracked(),
                #[cfg(not(target_arch = "wasm32"))]
                payer: None,
            };

            match create_token(params).await {
                Ok(result) => {
                    set_status.set(result.status);
                    set_success.set(Some(format!(
                        "Token created successfully!\n\
                         Mint Address: {}\n\
                         View on Solscan: {}\n\
                         Transaction: {}",
                        result.mint,
                        result.explorer_url,
                        result.signature
                    )));
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
                            set_metadata_uri.set(event_target_value(&ev));
                        }
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

                <div class="form-group">
                    <label>"Smart Contract Settings"</label>
                    
                    <div class="form-row">
                        <label for="rate_limit">"Rate Limit (tokens per day)"</label>
                        <input
                            type="number"
                            id="rate_limit"
                            min="0"
                            placeholder="Optional: Enter max tokens per day"
                            on:input=move |ev| {
                                let value = event_target_value(&ev).parse::<u64>().ok();
                                set_rate_limit.set(value);
                            }
                        />
                    </div>

                    <div class="form-row">
                        <label for="transfer_fee">"Transfer Fee (basis points)"</label>
                        <input
                            type="number"
                            id="transfer_fee"
                            min="0"
                            max="10000"
                            placeholder="Optional: Enter fee in basis points (0-10000)"
                            on:input=move |ev| {
                                let value = event_target_value(&ev).parse::<u16>().ok();
                                set_transfer_fee.set(value);
                            }
                        />
                    </div>

                    <div class="form-row">
                        <label for="max_transfer">"Max Transfer Amount"</label>
                        <input
                            type="number"
                            id="max_transfer"
                            min="0"
                            placeholder="Optional: Enter max tokens per transfer"
                            on:input=move |ev| {
                                let value = event_target_value(&ev).parse::<u64>().ok();
                                set_max_transfer_amount.set(value);
                            }
                        />
                    </div>
                </div>

                <div id="creation-status" class="status-message">
                    {move || status.get()}
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
            if let Err(e) = ctx.connect(WalletType::Phantom).await {
                ctx.set_error(&e);
            }
        }
    });
    
    let wallet_ctx_metamask = wallet_ctx.clone();
    let connect_metamask = create_action(move |_: &()| {
        let ctx = wallet_ctx_metamask.clone();
        async move {
            if let Err(e) = ctx.connect(WalletType::MetaMask).await {
                ctx.set_error(&e);
            }
        }
    });
    
    let wallet_ctx_disconnect = wallet_ctx.clone();
    let disconnect = create_action(move |_: &()| {
        let ctx = wallet_ctx_disconnect.clone();
        async move {
            ctx.disconnect();
        }
    });

    view! {
        <div class="wallet-connect">
            {move || if state.get().connected {
                view! {
                    <div class="wallet-info">
                        <span class="wallet-address">{state.get().address.clone().unwrap_or_default()}</span>
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

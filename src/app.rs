use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos::ev::SubmitEvent;
use crate::wallet::{WalletProvider, WalletContext, WalletType};
use crate::token::{create_token, CreateTokenParams, NetworkType};
use crate::utils::pinata::upload_metadata_to_pinata;

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
            <h2 class="token-management">"Token Management"</h2>
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
    let (token_uri, set_token_uri) = create_signal(String::new());
    let (decimals, set_decimals) = create_signal(9u8);
    let (initial_supply, set_initial_supply) = create_signal(1_000_000_000u64);
    let (is_mutable, set_is_mutable) = create_signal(true);
    let (freeze_authority, set_freeze_authority) = create_signal(true);
    let (rate_limit, set_rate_limit) = create_signal(Option::<u64>::None);
    let (transfer_fee, set_transfer_fee) = create_signal(Option::<u16>::None);
    let (max_transfer_amount, set_max_transfer_amount) = create_signal(Option::<u64>::None);
    let (network, set_network) = create_signal(NetworkType::Devnet);
    let (loading, _set_loading) = create_signal(false);
    let (error, _set_error) = create_signal(Option::<String>::None);
    let (success, _set_success) = create_signal(Option::<String>::None);
    let (status, set_status) = create_signal(String::new());

    let handle_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        let token_name = token_name.get_untracked();
        let token_name_clone = token_name.clone();
        let token_symbol = token_symbol.get_untracked();
        let token_uri = token_uri.get_untracked();
        let network = network.get();

        set_status.set("Creating token metadata...".to_string());

        spawn_local(async move {
            let metadata = serde_json::json!({
                "name": token_name,
                "symbol": token_symbol,
                "description": format!("{} token", token_name),
                "image": token_uri,
                "attributes": []
            });

            match upload_metadata_to_pinata(
                &std::env::var("PINATA_API_KEY").unwrap_or_default(),
                &std::env::var("PINATA_SECRET_KEY").unwrap_or_default(),
                &metadata
            ).await {
                Ok(metadata_uri) => {
                    set_status.set("Creating token...".to_string());

                    let params = CreateTokenParams {
                        name: token_name,
                        symbol: token_symbol,
                        description: format!("{} token", token_name_clone),
                        metadata_uri,
                        decimals: decimals.get_untracked(),
                        initial_supply: initial_supply.get_untracked(),
                        is_mutable: is_mutable.get_untracked(),
                        freeze_authority: freeze_authority.get_untracked(),
                        rate_limit: rate_limit.get_untracked(),
                        transfer_fee: transfer_fee.get_untracked(),
                        max_transfer_amount: max_transfer_amount.get_untracked(),
                        network,
                        #[cfg(not(target_arch = "wasm32"))]
                        payer: None,
                    };

                    match create_token(params).await {
                        Ok(result) => {
                            set_status.set(format!("View on Explorer: {}", result.explorer_url));
                        }
                        Err(e) => {
                            set_status.set(format!("Token creation failed: {}", e));
                        }
                    }
                }
                Err(e) => {
                    set_status.set(format!("Metadata upload failed: {}", e));
                }
            }
        });
    };

    view! {
        <div class="container">
            <h2 class="token-management">"Create Token"</h2>
            <div class="token-forms">
                <form class="token-form" on:submit=handle_submit>
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
                        <label for="token_uri">"Token URI"</label>
                        <input
                            type="text"
                            id="token_uri"
                            placeholder="Enter token URI"
                            on:input=move |ev| {
                                set_token_uri.set(event_target_value(&ev));
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

                    <div class="form-group">
                        <label>"Network"</label>
                        <select 
                            class="select-input"
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                set_network(if value == "mainnet" {
                                    NetworkType::Mainnet
                                } else {
                                    NetworkType::Devnet
                                });
                            }
                        >
                            <option value="devnet" selected=move || network.get() == NetworkType::Devnet>
                                "Devnet"
                            </option>
                            <option value="mainnet" selected=move || network.get() == NetworkType::Mainnet>
                                "Mainnet"
                            </option>
                        </select>
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
        </div>
    }
}

#[component]
fn SendTokenPage() -> impl IntoView {
    let _wallet_ctx = use_context::<WalletContext>().expect("WalletContext not found");
    let (token_address, set_token_address) = create_signal(String::new());
    let (recipient_address, set_recipient_address) = create_signal(String::new());
    let (amount, set_amount) = create_signal(0u64);
    let (loading, _set_loading) = create_signal(false);
    let (error_msg, _set_error_msg) = create_signal(Option::<String>::None);
    let (success_msg, _set_success_msg) = create_signal(Option::<String>::None);

    view! {
        <div class="container">
            <h2>"Send Token"</h2>
            <form>
                <div class="status-message">
                    {move || loading.get().then(|| view! {
                        <div class="loading-message">"Processing..."</div>
                    })}
                    {move || error_msg.get().map(|msg| view! {
                        <div class="error-message">{msg}</div>
                    })}
                    {move || success_msg.get().map(|msg| view! {
                        <div class="success-message">{msg}</div>
                    })}
                </div>
                <div class="form-group">
                    <label for="token-address">"Token Address"</label>
                    <input
                        type="text"
                        id="token-address"
                        value=move || token_address.get()
                        on:input=move |ev| {
                            set_token_address(event_target_value(&ev));
                        }
                    />
                </div>
                <div class="form-group">
                    <label for="recipient">"Recipient Address"</label>
                    <input
                        type="text"
                        id="recipient"
                        value=move || recipient_address.get()
                        on:input=move |ev| {
                            set_recipient_address(event_target_value(&ev));
                        }
                    />
                </div>
                <div class="form-group">
                    <label for="amount">"Amount"</label>
                    <input
                        type="number"
                        id="amount"
                        value=move || amount.get().to_string()
                        on:input=move |ev| {
                            if let Ok(val) = event_target_value(&ev).parse::<u64>() {
                                set_amount(val);
                            }
                        }
                    />
                </div>
                <button
                    type="submit"
                    class="button"
                    disabled=move || loading.get()
                >
                    {move || if loading.get() { "Sending..." } else { "Send" }}
                </button>
            </form>
        </div>
    }
}

#[component]
fn WalletConnect() -> impl IntoView {
    let wallet_ctx = use_context::<WalletContext>().expect("No wallet context found");
    let (_balance_error, _set_balance_error) = create_signal(None::<String>);
    
    create_effect({
        let wallet_ctx = wallet_ctx.clone();
        move |_| {
            if wallet_ctx.state.get().connected {
                let wallet_ctx = wallet_ctx.clone();
                spawn_local(async move {
                    if let Ok(balance) = wallet_ctx.get_balance().await {
                        wallet_ctx.state.update(|state| {
                            state.sol_balance = balance;
                        });
                    }
                    if let Ok(balances) = wallet_ctx.get_token_balances().await {
                        wallet_ctx.state.update(|state| {
                            state.token_balances = balances;
                        });
                    }
                });
            }
        }
    });

    let connect_phantom = create_action({
        let wallet_ctx = wallet_ctx.clone();
        move |_: &()| {
            let wallet_ctx = wallet_ctx.clone();
            async move {
                let _ = wallet_ctx.connect(WalletType::Phantom).await;
            }
        }
    });

    let connect_metamask = create_action({
        let wallet_ctx = wallet_ctx.clone();
        move |_: &()| {
            let wallet_ctx = wallet_ctx.clone();
            async move {
                let _ = wallet_ctx.connect(WalletType::MetaMask).await;
            }
        }
    });

    let disconnect = create_action({
        let wallet_ctx = wallet_ctx.clone();
        move |_: &()| {
            let wallet_ctx = wallet_ctx.clone();
            async move {
                wallet_ctx.disconnect();
            }
        }
    });

    view! {
        <div class="wallet-connect">
            {move || {
                let wallet_ctx = wallet_ctx.clone();
                if wallet_ctx.state.get().connected {
                    view! {
                        <div class="wallet-info">
                            <div class="wallet-address">
                                {wallet_ctx.state.get().address.clone().unwrap_or_default()}
                            </div>
                            <button class="disconnect-button"
                                on:click=move |_| disconnect.dispatch(())>
                                "Disconnect"
                            </button>
                        </div>
                    }
                } else {
                    view! {
                        <div class="connect-buttons">
                            <button class="connect-button"
                                on:click=move |_| connect_phantom.dispatch(())>
                                "Connect Phantom"
                            </button>
                            <button class="connect-button"
                                on:click=move |_| connect_metamask.dispatch(())>
                                "Connect MetaMask"
                            </button>
                        </div>
                    }
                }
            }}
        </div>
    }
}

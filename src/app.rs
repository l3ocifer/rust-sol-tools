use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use web_sys::SubmitEvent;
use crate::wallet::{WalletProvider, WalletContext, WalletType};

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
                        <h1>"Token Tools"</h1>
                        <WalletConnect/>
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
    let (token_uri, set_token_uri) = create_signal(String::new());
    let (initial_supply, set_initial_supply) = create_signal(1_000_000_000u64);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        if !wallet_ctx.state.get().connected {
            logging::warn!("Please connect your wallet first");
            return;
        }
        logging::log!(
            "Creating token: {} {} {} supply: {}", 
            token_name.get(), 
            token_symbol.get(), 
            token_uri.get(),
            initial_supply.get(),
        );
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
                    <label for="token_uri">"Metadata URI"</label>
                    <input
                        type="url"
                        id="token_uri"
                        required
                        placeholder="Enter metadata URI"
                        on:input=move |ev| {
                            set_token_uri.set(event_target_value(&ev));
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

                <button type="submit" class="button">
                    {move || if wallet_ctx.state.get().connected {
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

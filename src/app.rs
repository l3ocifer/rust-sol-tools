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
        <Title text="Solana Tools - Token Creation & Management"/>

        <WalletProvider>
            <Router>
                <main>
                    <nav class="header">
                        <h1>"Solana Tools"</h1>
                        <div class="nav-links">
                            <A href="/">"Home"</A>
                            <A href="/create">"Create Token"</A>
                            <A href="/mint">"Mint Tokens"</A>
                            <WalletConnect/>
                        </div>
                    </nav>

                    <Routes>
                        <Route path="/" view=HomePage/>
                        <Route path="/create" view=CreateTokenPage/>
                        <Route path="/mint" view=MintTokenPage/>
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
            <h2>"Welcome to Solana Tools"</h2>
            <p>"A suite of tools for creating and managing Solana tokens."</p>
            
            <div class="features">
                <div class="feature-card">
                    <h3>"Create Token"</h3>
                    <p>"Create new SPL tokens with custom metadata"</p>
                    <A href="/create" class="button">"Get Started"</A>
                </div>
                
                <div class="feature-card">
                    <h3>"Mint Tokens"</h3>
                    <p>"Mint additional tokens to any wallet"</p>
                    <A href="/mint" class="button">"Start Minting"</A>
                </div>
            </div>
        </div>
    }
}

#[component]
fn CreateTokenPage() -> impl IntoView {
    let (token_name, set_token_name) = create_signal(String::new());
    let (token_symbol, set_token_symbol) = create_signal(String::new());
    let (token_uri, set_token_uri) = create_signal(String::new());
    let (_decimals, set_decimals) = create_signal(9u8);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        logging::log!("Creating token: {} {} {}", token_name.get(), token_symbol.get(), token_uri.get());
    };

    view! {
        <div class="container">
            <h2>"Create New SPL Token"</h2>
            
            <form class="token-form" on:submit=on_submit>
                <div class="form-group">
                    <label for="token_name">"Token Name"</label>
                    <input
                        type="text"
                        id="token_name"
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
                        placeholder="Enter metadata URI"
                        on:input=move |ev| {
                            set_token_uri.set(event_target_value(&ev));
                        }
                    />
                </div>

                <div class="form-group">
                    <label for="decimals">"Decimals"</label>
                    <input
                        type="number"
                        id="decimals"
                        min="0"
                        max="9"
                        value="9"
                        on:input=move |ev| {
                            if let Ok(value) = event_target_value(&ev).parse() {
                                set_decimals.set(value);
                            }
                        }
                    />
                </div>

                <button type="submit" class="button">"Create Token"</button>
            </form>
        </div>
    }
}

#[component]
fn MintTokenPage() -> impl IntoView {
    let (mint_address, set_mint_address) = create_signal(String::new());
    let (receiver_address, set_receiver_address) = create_signal(String::new());
    let (amount, set_amount) = create_signal(0u64);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        logging::log!("Minting {} tokens to {} from mint {}", 
            amount.get(), 
            receiver_address.get(), 
            mint_address.get()
        );
        // TODO: Implement token minting
    };

    view! {
        <div class="container">
            <h2>"Mint SPL Tokens"</h2>
            
            <form class="token-form" on:submit=on_submit>
                <div class="form-group">
                    <label for="mint_address">"Token Mint Address"</label>
                    <input
                        type="text"
                        id="mint_address"
                        placeholder="Enter token mint address"
                        on:input=move |ev| {
                            set_mint_address.set(event_target_value(&ev));
                        }
                    />
                </div>

                <div class="form-group">
                    <label for="receiver_address">"Receiver Address"</label>
                    <input
                        type="text"
                        id="receiver_address"
                        placeholder="Enter receiver's wallet address"
                        on:input=move |ev| {
                            set_receiver_address.set(event_target_value(&ev));
                        }
                    />
                </div>

                <div class="form-group">
                    <label for="amount">"Amount"</label>
                    <input
                        type="number"
                        id="amount"
                        min="1"
                        placeholder="Enter amount to mint"
                        on:input=move |ev| {
                            if let Ok(value) = event_target_value(&ev).parse() {
                                set_amount.set(value);
                            }
                        }
                    />
                </div>

                <button type="submit" class="button">"Mint Tokens"</button>
            </form>
        </div>
    }
}

#[component]
fn WalletConnect() -> impl IntoView {
    let wallet_ctx = use_context::<WalletContext>().expect("WalletContext not found");
    let state = wallet_ctx.state;

    let connect_phantom = move |_| wallet_ctx.connect(WalletType::Phantom);
    let connect_metamask = move |_| wallet_ctx.connect(WalletType::MetaMask);
    let disconnect = move |_| wallet_ctx.disconnect();

    view! {
        <div class="wallet-connect">
            {move || {
                if state.get().connected {
                    view! {
                        <div class="wallet-info">
                            <span class="wallet-address">{state.get().address.unwrap_or_default()}</span>
                            <button class="button" on:click=disconnect>"Disconnect"</button>
                        </div>
                    }
                } else {
                    view! {
                        <div class="wallet-buttons">
                            <button class="button" on:click=connect_phantom>"Connect Phantom"</button>
                            <button class="button" on:click=connect_metamask>"Connect MetaMask"</button>
                        </div>
                    }
                }
            }}
        </div>
    }
}

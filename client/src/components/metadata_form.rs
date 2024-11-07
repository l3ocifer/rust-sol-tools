use leptos::*;
use serde::Serialize;
use web_sys::SubmitEvent;

#[derive(Serialize, Clone)]
struct MetadataInput {
    name: String,
    symbol: String,
    description: String,
    image: String,
}

#[component]
pub fn MetadataForm() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let (symbol, set_symbol) = create_signal(String::new());
    let (description, set_description) = create_signal(String::new());
    let (image, set_image) = create_signal(String::new());
    let (metadata_uri, set_metadata_uri) = create_signal(String::new());

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let metadata = MetadataInput {
            name: name.get(),
            symbol: symbol.get(),
            description: description.get(),
            image: image.get(),
        };

        spawn_local(async move {
            let client = reqwest::Client::new();
            if let Ok(res) = client
                .post("http://localhost:3000/upload-metadata")
                .json(&metadata)
                .send()
                .await
            {
                if let Ok(json) = res.json::<serde_json::Value>().await {
                    if let Some(uri) = json["uri"].as_str() {
                        set_metadata_uri.set(uri.to_string());
                    }
                }
            }
        });
    };

    view! {
        <>
            <form on:submit=on_submit>
                <input
                    type="text"
                    placeholder="Token Name"
                    on:input=move |ev| set_name.set(event_target_value(&ev))
                />
                <input
                    type="text"
                    placeholder="Token Symbol"
                    on:input=move |ev| set_symbol.set(event_target_value(&ev))
                />
                <textarea
                    placeholder="Description"
                    on:input=move |ev| set_description.set(event_target_value(&ev))
                />
                <input
                    type="text"
                    placeholder="Image URL"
                    on:input=move |ev| set_image.set(event_target_value(&ev))
                />
                <button type="submit">"Upload Metadata"</button>
            </form>
            {move || metadata_uri.get().is_empty().then(|| view! {
                <CreateTokenButton metadata_uri=metadata_uri.get()/>
            })}
        </>
    }
} 
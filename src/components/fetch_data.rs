use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Deserialize, Debug, Clone, PartialEq)]
struct ApiResponseStoic {
    text: String,
    author: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
struct ApiResponseZenn {
    q: String,
    a: String,
}

#[function_component(FetchData)]
pub fn fetch_data() -> Html {
    let quote_stoic = use_state(|| None);
    let quote_zenn = use_state(|| None);

    {
        let quote_stoic = quote_stoic.clone();
        let quote_zenn = quote_zenn.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let url_stoic = "https://stoic-quotes.com/api/quote";
                let response_stoic = reqwest::get(url_stoic).await.unwrap();
                let data_stoic: ApiResponseStoic = response_stoic.json().await.unwrap();
                quote_stoic.set(Some(data_stoic));

                let url_zenn = "https://api.allorigins.win/get?url=https://zenquotes.io/api/random";
                let response_zenn = reqwest::get(url_zenn).await.unwrap();
                let json_zenn: serde_json::Value = response_zenn.json().await.unwrap();
                let data_str = json_zenn["contents"].as_str().unwrap();
                let data_zenn: Vec<ApiResponseZenn> = serde_json::from_str(data_str).unwrap();
                quote_zenn.set(Some(data_zenn[0].clone()));
            });
            || ()
        });
    }

    html! {
        <div class="custom-card">
            <h2 class="fw-bold">{ "今日の名言" }</h2>
            <h3 class="fw-bold">{ "その１" }</h3>
            { if let Some(data) = &*quote_stoic {
                html! { <p class="fst-italic">{ format!("「{}」 - {}", data.text, data.author) }</p> }
            } else {
                html! { <p class="text-muted">{ "読み込み中..." }</p> }
            } }
            <h3 class="fw-bold">{ "その２" }</h3>
            { if let Some(data) = &*quote_zenn {
                html! { <p class="fst-italic">{ format!("「{}」 - {}", data.q, data.a) }</p> }
            } else {
                html! { <p class="text-muted">{ "読み込み中..." }</p> }
            } }
        </div>
    }
}
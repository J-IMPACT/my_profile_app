use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class="card">
            <h2>{ "このアプリについて" }</h2>
            <p>{ "これはYewとTrunkで作成されたシンプルなWebアプリです。" }</p>
        </div>
    }
}
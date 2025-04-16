use yew::prelude::*;

use super::utils::set_meta;

#[function_component(About)]
pub fn about() -> Html {
    use_effect(|| {
        set_meta("About | My Profile App", "このアプリについて説明しています");
        || ()
    });
    
    html! {
        <div class="card">
            <h2>{ "このアプリについて" }</h2>
            <p>{ "これはYewとTrunkで作成されたシンプルなWebアプリです。" }</p>
        </div>
    }
}
use yew::prelude::*;

use super::utils::set_meta;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    use_effect(|| {
        set_meta("Page Not Found | My Profile App", "ページが存在しません");
        || ()
    });
    
    html! {
        <h2>{ "ページが存在しません" }</h2>
    }
}
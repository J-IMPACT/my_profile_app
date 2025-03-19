use yew::prelude::*;

#[derive(Properties, PartialEq)] // Props (第2章)
pub struct ProfileProps {
    pub name: String,
    pub bio: String,
}

#[function_component(Profile)]
pub fn profile(props: &ProfileProps) -> Html {
    let show_info = use_state(|| false); // 状態管理 (第2章)

    let toggle_info = {
        let show_info = show_info.clone();
        Callback::from(move |_| show_info.set(!*show_info))
    };

    let button_message = if *show_info {
        "プロフィールを隠す"
    } else {
        "プロフィールを見る"
    };

    html! {
        <div class="custom-card">
            <h2 class="fw-bold">{ "プロフィール" }</h2>
            <button class="btn btn-custom mt-2" onclick={toggle_info.clone()}>
                { button_message }
            </button>
            if *show_info {
                <div class="mt-3">
                    <h3>{ &props.name }</h3>
                    <p class="text-muted">{ &props.bio }</p>
                </div>
            }
        </div>
    }
}
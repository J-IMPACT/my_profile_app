use web_sys::HtmlInputElement;
use yew::prelude::*;

/// 入力フォーム
#[function_component(InputForm)]
pub fn input_form() -> Html {
    let input_value = use_state(|| "".to_string());

    let on_input = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement =  e.target_unchecked_into();
            input_value.set(input.value());
        })
    };

    html! {
        <div class="custom-card input-form">
            <h2 class="fw-bold">{ "名前を入力してください" }</h2>
            <input
                type="text"
                class="form-control mt-2"
                placeholder="ここに入力..."
                oninput={on_input.clone()}
            />
            <p class="mt-2 text-muted">{ format!("入力値: {}", *input_value) }</p>
        </div>
    }
}
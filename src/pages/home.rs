use yew::prelude::*;

use crate::components::fetch_data::FetchData;
use crate::components::input_form::InputForm;
use crate::components::profile::Profile;

use super::utils::set_meta;

#[function_component(Home)]
pub fn home() -> Html {
    use_effect(|| {
        set_meta("Home | My Profile App", "トップページです");
        || ()
    });

    html! {
        <div class="main-container">
            <h1 class="text-primary fw-bold">{ "Welcome to My Profile App" }</h1>
            <div class="mb-4">
                <Profile name="Taro Yamada" bio="RustとWeb開発が大好きなエンジニアです。" /> // 自己紹介ボタン
            </div>
            <div class="mb-4">
                <InputForm /> // 入力フォーム
            </div>
            <div class="mb-4">
                <FetchData /> // APIデータ取得
            </div>
        </div>
    }
}
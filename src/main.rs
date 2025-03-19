use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;

use pages::home::Home;
use pages::about::About;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> }, // ホームページ (第3章)
        Route::About => html! { <About /> }, // 別ページ (第3章)
    }
}

#[function_component(App)] // アプリのルートコンポーネント (第2章)
fn app() -> Html {
    html! {
        <BrowserRouter>
            <nav class="navbar navbar-expand-lg navbar-dark bg-primary">
                <div class="container">
                    <a class="navbar-brand fw-bold" href="/">{ "My Profile App "}</a>
                    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav">
                        <span class="navbar-toggler-icon"></span>
                    </button>
                    <div class="collapse navbar-collapse" id="navbarNav">
                        <ul class="navbar-nav ms-auto">
                            <li class="nav-item">
                                <a class="nav-link text-white fw-bold" href="/">{ "Home" }</a>
                            </li>
                            <li class="nav-item">
                                <a class="nav-link text-white fw-bold" href="/about">{ "About" }</a>
                            </li>
                        </ul>
                    </div>
                    <a href="/">{ "Home" }</a>
                    <a href="/about">{ "About" }</a>
                </div>
            </nav>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
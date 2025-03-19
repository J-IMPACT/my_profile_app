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

#[function_component(NavBar)]
fn navbar() -> Html {
    html! {
        <nav class="navbar navbar-expand-lg navbar-dark bg-primary">
            <div class="container">
                <a class="navbar-brand fw-bold" href="/my_profile_app/#/">{"My Profile App"}</a>
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarNav">
                    <ul class="navbar-nav ms-auto">
                        <li class="nav-item"><a class="nav-link" href="/my_profile_app/#/">{"Home"}</a></li>
                        <li class="nav-item"><a class="nav-link" href="/my_profile_app/#/about">{"About"}</a></li>
                    </ul>
                </div>
            </div>
        </nav>
    }
}

#[function_component(App)] // アプリのルートコンポーネント (第2章)
fn app() -> Html {
    html! {
        <HashRouter>
            <NavBar />
            <div class="container mt-4">
                <Switch<Route> render={switch} />
            </div>
        </HashRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
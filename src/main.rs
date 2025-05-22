use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;

use pages::home::Home;
use pages::about::About;

// ルーティング
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> }, // ホームページ
        Route::About => html! { <About /> }, // サブページ
    }
}

// ナビゲーションバー
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
                        <li class="nav-item"><Link<Route> classes="nav-link" to={Route::Home}>{"Home"}</Link<Route>></li>
                        <li class="nav-item"><Link<Route> classes="nav-link" to={Route::About}>{"About"}</Link<Route>></li>
                    </ul>
                </div>
            </div>
        </nav>
    }
}

// アプリのルートコンポーネント
#[function_component(App)] 
fn app() -> Html {
    html! {
        <BrowserRouter>
            <NavBar />
            <div class="container mt-4">
                <Switch<Route> render={switch} />
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
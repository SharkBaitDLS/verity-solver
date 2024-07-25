use yew::{function_component, html, Html, Renderer};
use yew_router::{BrowserRouter, Switch};

use normal_realm::NormalRealm;
use not_found::NotFound;
use route::Route;
use shadow_realm::ShadowRealm;

mod normal_realm;
mod not_found;
mod route;
mod shadow_realm;
mod shape_picker;
#[allow(dead_code)]
mod solver;

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <div class="main is-flex is-flex-direction-column">
                // TODO: header, nav?
                <main>
                    <Switch<Route> render={self::switch} />
                </main>
                <footer class="footer mt-auto">
                    <div class="content has-text-centered">
                        { "Powered by " }
                        <a href="https://yew.rs">{ "Yew" }</a>
                        { " using " }
                        <a href="https://bulma.io">{ "Bulma" }</a>
                    </div>
                </footer>
            </div>
        </BrowserRouter>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::ShadowRealm => html! { <ShadowRealm/> },
        Route::NormalRealm => html! { <NormalRealm/> },
        Route::NotFound => html! { <NotFound/> },
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    Renderer::<App>::new().render();
}

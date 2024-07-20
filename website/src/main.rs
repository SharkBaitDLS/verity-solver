use yew::{function_component, html, Html, Renderer};

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <p>{ "Hello, verity solver!" }</p>
        </div>
    }
}

fn main() {
    Renderer::<App>::new().render();
}

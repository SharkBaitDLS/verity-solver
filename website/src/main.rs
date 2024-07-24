use yew::{function_component, html, Html, Renderer};

use crate::shape_picker::ShapePicker;

mod shape_picker;
#[allow(dead_code)]
mod solver;

#[function_component]
fn App() -> Html {
    html! {
        <div class="main is-flex is-flex-direction-column">
            <main>
                <p>{ "Hello, verity solver! Have some mocked-up buttons!" }</p>
                <ShapePicker multi_shape={true}/>
                <ShapePicker multi_shape={false}/>
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
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    Renderer::<App>::new().render();
}

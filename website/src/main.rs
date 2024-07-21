use yew::{function_component, html, Html, Renderer};

#[allow(dead_code)]
mod solver;

#[function_component]
fn App() -> Html {
    html! {
        <div class="main is-flex is-flex-direction-column">
            <main>
                <p>{ "Hello, verity solver! Have some mocked-up buttons!" }</p>
                <div class="hex-container">
                    <div class="hex-border">
                        <button class="hex-segment-active hex-segment-top"/>
                    </div>
                    <div class="hex-border">
                        <button class="hex-segment-inactive hex-segment-left"/>
                    </div>
                    <div>
                        <button class="hex-segment-clicked hex-segment-right"/>
                    </div>
                </div>
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
    Renderer::<App>::new().render();
}

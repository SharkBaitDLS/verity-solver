use yew::{function_component, html, Html};

#[function_component]
pub fn NotFound() -> Html {
    html! {
        <p>{"This is the not found page"}</p>
    }
}

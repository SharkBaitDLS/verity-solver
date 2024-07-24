mod button_segment;
mod state;

use button_segment::ButtonSegment;
use state::PickerState;
use verity_solver_models::Shape;
use yew::{function_component, html, use_reducer_eq, Callback, Html, Properties};

#[derive(Clone, Copy, Debug, Properties, PartialEq)]
pub struct ShapePickerProps {
    pub multi_shape: bool,
}

#[function_component]
pub fn ShapePicker(props: &ShapePickerProps) -> Html {
    let state = use_reducer_eq(|| PickerState::new(props.multi_shape));
    let on_change = {
        let state = state.clone();
        Callback::from(move |(shape, button_state)| state.dispatch((shape, button_state)))
    };

    html! {
        <div class="hex-container">
            <ButtonSegment shape={Shape::Square} on_change={on_change.clone()} disabled={state.square.disabled}/>
            <ButtonSegment shape={Shape::Circle} on_change={on_change.clone()} disabled={state.circle.disabled}/>
            <ButtonSegment shape={Shape::Triangle} on_change={on_change.clone()} disabled={state.triangle.disabled}/>
        </div>
    }
}

use std::rc::Rc;

use verity_solver_models::Shape;
use yew::{function_component, html, use_reducer_eq, Callback, Html, Reducible};

use super::button_segment::{ButtonSegment, ButtonState};

#[derive(Copy, Clone, Debug, PartialEq)]
struct ShapeState {
    pub selected: bool,
    pub disabled: bool,
}

impl ShapeState {
    fn new() -> Self {
        Self {
            selected: false,
            disabled: false,
        }
    }
}

/// State tuples are (selected, disabled)
#[derive(Copy, Clone, Debug, PartialEq)]
struct PickerState {
    square: ShapeState,
    circle: ShapeState,
    triangle: ShapeState,
}

impl PickerState {
    fn new() -> Self {
        Self {
            square: ShapeState::new(),
            circle: ShapeState::new(),
            triangle: ShapeState::new(),
        }
    }
}

impl Reducible for PickerState {
    type Action = (Shape, ButtonState);

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let (shape, state) = action;
        let is_selected = state == ButtonState::Selected;

        let mut new = *self.as_ref();

        match shape {
            Shape::Square => new.square.selected = is_selected,
            Shape::Circle => new.circle.selected = is_selected,
            Shape::Triangle => new.triangle.selected = is_selected,
        }

        new.triangle.disabled = new.square.selected && new.circle.selected;
        new.circle.disabled = new.triangle.selected && new.square.selected;
        new.square.disabled = new.circle.selected && new.triangle.selected;

        Rc::new(new)
    }
}

#[function_component]
pub fn ShapePicker() -> Html {
    let state = use_reducer_eq(PickerState::new);
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

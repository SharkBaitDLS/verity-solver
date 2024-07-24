mod button_segment;
mod display_image;
mod state;

use button_segment::ButtonSegment;
use display_image::DisplayImage;
use state::PickerState;
use verity_solver_models::{CompositeShape, Shape};
use yew::{function_component, html, use_reducer_eq, Callback, Html, Properties};

#[derive(Clone, Copy, Debug, Properties, PartialEq)]
pub struct ShapePickerProps {
    pub multi_shape: bool,
}

#[function_component]
pub fn ShapePicker(props: &ShapePickerProps) -> Html {
    let state = use_reducer_eq(|| PickerState::new(props.multi_shape));
    let cb = {
        let state = state.clone();
        Callback::from(move |(shape, button_state)| state.dispatch((shape, button_state)))
    };
    let src = if props.multi_shape {
        match state.get_composite() {
            Some(composite) if composite == CompositeShape::CONE => Some("cone.png"),
            Some(composite) if composite == CompositeShape::CUBE => Some("cube.png"),
            Some(composite) if composite == CompositeShape::CYLINDER => Some("cylinder.png"),
            Some(composite) if composite == CompositeShape::PYRAMID => Some("pyramid.png"),
            Some(composite) if composite == CompositeShape::SPHERE => Some("sphere.png"),
            Some(composite) if composite == CompositeShape::TRIANGULAR_PRISM => Some("trigon.png"),
            _ => None,
        }
    } else {
        state.get_simple().map(|shape| match shape {
            Shape::Square => "square.png",
            Shape::Circle => "circle.png",
            Shape::Triangle => "triangle.png",
        })
    };

    html! {
        <div>
            <div class="hex-container">
                <ButtonSegment shape={Shape::Square} on_change={cb.clone()} disabled={state.square.disabled}/>
                <ButtonSegment shape={Shape::Circle} on_change={cb.clone()} disabled={state.circle.disabled}/>
                <ButtonSegment shape={Shape::Triangle} on_change={cb.clone()} disabled={state.triangle.disabled}/>
            </div>
            <div class="display-hex display-outer">
                <DisplayImage {src}/>
                <div class="display-border">
                    <div class="display-hex"/>
                </div>
            </div>
        </div>
    }
}

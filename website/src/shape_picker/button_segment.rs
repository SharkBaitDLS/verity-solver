use verity_solver_models::Shape;
use yew::{function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct SegmentProps {
    pub disabled: bool,
    pub shape: Shape,
    pub on_change: Callback<(Shape, ButtonState)>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonState {
    Selected,
    Deselected,
}

impl ButtonState {
    fn as_button_class(&self) -> &str {
        match self {
            ButtonState::Selected => "hex-segment-selected",
            ButtonState::Deselected => "hex-segment-deselected",
        }
    }
}

#[function_component]
pub fn ButtonSegment(props: &SegmentProps) -> Html {
    let state = yew::use_state(|| ButtonState::Deselected);
    let shape = props.shape;
    let disabled = props.disabled;

    let onclick = {
        let internal_state = state.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |_| {
            internal_state.set(match *internal_state {
                ButtonState::Deselected => {
                    on_change.emit((shape, ButtonState::Selected));
                    ButtonState::Selected
                }
                ButtonState::Selected => {
                    on_change.emit((shape, ButtonState::Deselected));
                    ButtonState::Deselected
                }
            });
        })
    };

    let (button_classes, text_class) = match props.shape {
        Shape::Square => ("hex-segment-top", "hex-text-square"),
        Shape::Circle => ("hex-segment-left", "hex-text-circle"),
        Shape::Triangle => ("hex-segment-right", "hex-text-triangle"),
    };
    let button_classes = format!("hex-segment {} {}", state.as_button_class(), button_classes,);

    html! {
        <div>
            <div class="hex-border">
                <button {disabled} {onclick} class={button_classes}>
                    <p class={format!("{} {}", text_class, if disabled { "hex-text-disabled" } else { "hex-text" })}>
                    {
                        match props.shape {
                            Shape::Square => "■",
                            Shape::Circle => "●",
                            Shape::Triangle => "▲",
                        }
                    }
                    </p>
                </button>
            </div>
        </div>
    }
}

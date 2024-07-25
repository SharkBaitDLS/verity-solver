use yew::{function_component, html, Html};

use crate::shape_picker::ShapePicker;

#[function_component]
pub fn NormalRealm() -> Html {
    html! {
        <div>
            <div class="level">
                <div class="columns level-item">
                    <ShapePicker multi_shape={false}/>
                    <ShapePicker multi_shape={false}/>
                    <ShapePicker multi_shape={false}/>
                </div>
            </div>
            <div class="level">
                <div class="columns level-item">
                    <ShapePicker multi_shape={true}/>
                    <ShapePicker multi_shape={true}/>
                    <ShapePicker multi_shape={true}/>
                </div>
            </div>
        </div>
    }
}

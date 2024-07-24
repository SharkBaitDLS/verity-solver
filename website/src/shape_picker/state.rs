use std::{rc::Rc, vec::IntoIter};

use itertools::Itertools;
use verity_solver_models::{CompositeShape, Shape};
use yew::Reducible;

use super::button_segment::ButtonState;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ShapeState {
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PickerState {
    multi_shape: bool,
    pub square: ShapeState,
    pub circle: ShapeState,
    pub triangle: ShapeState,
}

impl PickerState {
    pub fn new(multi_shape: bool) -> Self {
        Self {
            multi_shape,
            square: ShapeState::new(),
            circle: ShapeState::new(),
            triangle: ShapeState::new(),
        }
    }

    pub fn get_simple(&self) -> Option<Shape> {
        self.into_iter()
            .find(|(_shape, state)| state.selected)
            .map(|(shape, _state)| shape)
    }

    pub fn get_composite(&self) -> Option<CompositeShape> {
        self.into_iter()
            .filter(|(_shape, state)| state.selected)
            .map(|(shape, _state)| shape)
            .collect_tuple::<(Shape, Shape)>()
            .map(|tuple| tuple.into())
            .or(self.get_simple().map(|shape| (shape, shape).into()))
    }
}

impl IntoIterator for &PickerState {
    type Item = (Shape, ShapeState);

    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            (Shape::Square, self.square),
            (Shape::Circle, self.circle),
            (Shape::Triangle, self.triangle),
        ]
        .into_iter()
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

        if self.multi_shape {
            new.triangle.disabled = new.square.selected && new.circle.selected;
            new.circle.disabled = new.triangle.selected && new.square.selected;
            new.square.disabled = new.circle.selected && new.triangle.selected;
        } else {
            new.triangle.disabled = new.square.selected || new.circle.selected;
            new.circle.disabled = new.triangle.selected || new.square.selected;
            new.square.disabled = new.circle.selected || new.triangle.selected;
        }

        Rc::new(new)
    }
}

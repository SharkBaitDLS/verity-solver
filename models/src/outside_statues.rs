use std::mem;

use crate::{CompositeShape, Exchange, Shape, Statue};

#[derive(Debug, PartialEq)]
pub struct OutsideStatues {
    pub left: CompositeShape,
    pub middle: CompositeShape,
    pub right: CompositeShape,
}

impl OutsideStatues {
    pub fn find(&self, shape: &Shape, desired_state: &OutsideStatues) -> Result<Statue, String> {
        if self.left != desired_state.left && self.left.contains(shape) {
            Ok(Statue::Left)
        } else if self.middle != desired_state.middle && self.middle.contains(shape) {
            Ok(Statue::Middle)
        } else if self.right != desired_state.right && self.right.contains(shape) {
            Ok(Statue::Right)
        } else {
            Err("Solution is already complete, no more moves required".to_owned())
        }
    }

    pub fn exchange(&mut self, exchange: Exchange) -> Result<(), String> {
        match exchange.into_ordered_tuple() {
            ((Statue::Left, left_shape), (Statue::Right, right_shape)) => mem::swap(
                self.left.mut_ref_of(&left_shape)?,
                self.right.mut_ref_of(&right_shape)?,
            ),
            ((Statue::Left, left_shape), (Statue::Middle, middle_shape)) => mem::swap(
                self.left.mut_ref_of(&left_shape)?,
                self.middle.mut_ref_of(&middle_shape)?,
            ),
            ((Statue::Middle, middle_shape), (Statue::Right, right_shape)) => mem::swap(
                self.middle.mut_ref_of(&middle_shape)?,
                self.right.mut_ref_of(&right_shape)?,
            ),
            _ => return Err("A statue cannot be swapped with itself".to_owned()),
        }
        Ok(())
    }
}

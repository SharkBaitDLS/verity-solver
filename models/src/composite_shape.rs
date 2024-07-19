use crate::Shape;

#[derive(Debug)]
pub struct CompositeShape {
    pub(crate) a: Shape,
    pub(crate) b: Shape,
}

impl CompositeShape {
    pub fn new(a: Shape, b: Shape) -> Self {
        Self { a, b }
    }

    pub fn contains(&self, shape: &Shape) -> bool {
        &self.a == shape || &self.b == shape
    }

    pub fn mut_ref_of(&mut self, shape: &Shape) -> Result<&mut Shape, String> {
        if &self.a == shape {
            Ok(&mut self.a)
        } else if &self.b == shape {
            Ok(&mut self.b)
        } else {
            Err(format!(
                "This shape does not contain a reference to {:?}",
                shape
            ))
        }
    }

    pub fn difference(&self, other: &Self) -> Vec<(Shape, Shape)> {
        let mut result = Vec::new();

        // The first element of this is an invalid shape
        if !other.contains(&self.a) && !self.contains(&other.a) {
            result.push((other.a, self.a))
        } else if !other.contains(&self.a) && !self.contains(&other.b) {
            result.push((other.a, self.b))
        }

        // The second element of this is an invalid shape
        if !other.contains(&self.b) && !self.contains(&other.b) {
            result.push((other.b, self.b))
        } else if !other.contains(&self.b) && !self.contains(&other.a) {
            result.push((other.a, self.b))
        }

        // Both elements are the same valid shapes, so we need to get the other valid one
        if self.a == self.b && other.contains(&self.a) && !self.contains(&other.a) {
            result.push((other.a, self.a))
        } else if self.a == self.b && other.contains(&self.a) && !self.contains(&other.b) {
            result.push((other.b, self.a))
        }

        result
    }
}

impl PartialEq for CompositeShape {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b || self.a == other.b && self.b == other.a
    }
}

impl From<(Shape, Shape)> for CompositeShape {
    fn from(value: (Shape, Shape)) -> Self {
        CompositeShape {
            a: value.0,
            b: value.1,
        }
    }
}

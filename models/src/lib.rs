mod composite_shape;
mod outside_statues;

pub use composite_shape::CompositeShape;
pub use outside_statues::OutsideStatues;
pub use unordered_pair::UnorderedPair;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Shape {
    Square,
    Circle,
    Triangle,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Statue {
    Left,
    Middle,
    Right,
}

#[derive(Debug)]
pub struct InsideStatues {
    pub left: Shape,
    pub middle: Shape,
    pub right: Shape,
}

pub type Dissection = (Statue, Shape);
pub type Exchange = UnorderedPair<Dissection>;

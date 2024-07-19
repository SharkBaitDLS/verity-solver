use solver::solve;
use verity_solver_models::{CompositeShape, InsideStatues, OutsideStatues, Shape};

mod solver;

fn main() -> Result<(), String> {
    let outside_state = OutsideStatues {
        left: CompositeShape::new(Shape::Square, Shape::Triangle),
        middle: CompositeShape::new(Shape::Triangle, Shape::Circle),
        right: CompositeShape::new(Shape::Square, Shape::Circle),
    };

    let inside_state = InsideStatues {
        left: Shape::Square,
        middle: Shape::Circle,
        right: Shape::Triangle,
    };

    solve(inside_state, outside_state)?;
    Ok(())
}

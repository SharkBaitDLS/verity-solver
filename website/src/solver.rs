use verity_solver_models::{
    CompositeShape, Exchange, InsideStatues, OutsideStatues, Shape, Statue,
};

fn truth(shape: Shape) -> CompositeShape {
    match shape {
        Shape::Square => (Shape::Circle, Shape::Triangle).into(),
        Shape::Circle => (Shape::Square, Shape::Triangle).into(),
        Shape::Triangle => (Shape::Circle, Shape::Square).into(),
    }
}

fn find_truths(inside_state: InsideStatues) -> OutsideStatues {
    OutsideStatues {
        left: truth(inside_state.left),
        middle: truth(inside_state.middle),
        right: truth(inside_state.right),
    }
}

fn compute_moves(
    current_state: &OutsideStatues,
    desired_state: &OutsideStatues,
    statue: Statue,
    current_shape: &CompositeShape,
    desired_shape: &CompositeShape,
) -> Result<Vec<Exchange>, String> {
    let mut moves = Vec::new();
    let differnces = current_shape.difference(desired_shape);
    if differnces.is_empty() {
        return Err(format!(
            "No differnces computed between {:?} and {:?}, this is unexpected.",
            current_shape, desired_shape
        ));
    }
    for (expected, actual) in differnces {
        let target = current_state.find(&expected, desired_state)?;
        let pending_move = ((statue, actual), (target, expected)).into();
        moves.push(pending_move)
    }
    Ok(moves)
}

pub fn solve(
    inside_state: InsideStatues,
    outside_state: OutsideStatues,
) -> Result<Vec<Exchange>, String> {
    let desired_state = find_truths(inside_state);
    let mut moves: Vec<Exchange> = Vec::new();
    let mut mutable_state = outside_state;

    while mutable_state != desired_state {
        if mutable_state.left != desired_state.left {
            let new_moves = compute_moves(
                &mutable_state,
                &desired_state,
                Statue::Left,
                &mutable_state.left,
                &desired_state.left,
            )?;
            for pending_move in &new_moves {
                mutable_state.exchange(*pending_move)?;
            }
            moves.extend(new_moves);
        }

        if mutable_state.middle != desired_state.middle {
            let new_moves = compute_moves(
                &mutable_state,
                &desired_state,
                Statue::Middle,
                &mutable_state.middle,
                &desired_state.middle,
            )?;
            for pending_move in &new_moves {
                mutable_state.exchange(*pending_move)?;
            }
            moves.extend(new_moves);
        }

        if mutable_state.right != desired_state.right {
            let new_moves = compute_moves(
                &mutable_state,
                &desired_state,
                Statue::Right,
                &mutable_state.right,
                &desired_state.right,
            )?;
            for pending_move in &new_moves {
                mutable_state.exchange(*pending_move)?;
            }
            moves.extend(new_moves);
        }

        if moves.len() > 3 {
            return Err(format!(
                "Computation entered an invalid state, looping with moves: {:?}",
                moves
            ));
        }
    }

    Ok(moves)
}

#[cfg(test)]
mod tests {
    use super::*;
    use verity_solver_models::UnorderedPair;

    #[test]
    fn single_swap_case() -> Result<(), String> {
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

        let actual_solution = solve(inside_state, outside_state)?;
        let expected_solution: Vec<Exchange> = vec![UnorderedPair(
            (Statue::Left, Shape::Square),
            (Statue::Middle, Shape::Circle),
        )];

        assert_eq!(expected_solution, actual_solution);
        Ok(())
    }

    #[test]
    fn single_swap_case_skip_completed() -> Result<(), String> {
        let outside_state = OutsideStatues {
            left: CompositeShape::new(Shape::Square, Shape::Triangle),
            middle: CompositeShape::new(Shape::Triangle, Shape::Circle),
            right: CompositeShape::new(Shape::Square, Shape::Circle),
        };

        let inside_state = InsideStatues {
            left: Shape::Triangle,
            middle: Shape::Square,
            right: Shape::Circle,
        };

        let actual_solution = solve(inside_state, outside_state)?;
        let expected_solution: Vec<Exchange> = vec![UnorderedPair(
            (Statue::Left, Shape::Triangle),
            (Statue::Right, Shape::Circle),
        )];

        assert_eq!(expected_solution, actual_solution);
        Ok(())
    }

    #[test]
    fn two_swap_case() -> Result<(), String> {
        let outside_state = OutsideStatues {
            left: CompositeShape::new(Shape::Circle, Shape::Triangle),
            middle: CompositeShape::new(Shape::Triangle, Shape::Square),
            right: CompositeShape::new(Shape::Square, Shape::Circle),
        };

        let inside_state = InsideStatues {
            left: Shape::Triangle,
            middle: Shape::Square,
            right: Shape::Circle,
        };

        let actual_solution = solve(inside_state, outside_state)?;
        let expected_solution: Vec<Exchange> = vec![
            UnorderedPair(
                (Statue::Left, Shape::Triangle),
                (Statue::Middle, Shape::Square),
            ),
            UnorderedPair(
                (Statue::Middle, Shape::Triangle),
                (Statue::Right, Shape::Circle),
            ),
        ];

        assert_eq!(expected_solution, actual_solution);
        Ok(())
    }

    #[test]
    fn three_swap_case() -> Result<(), String> {
        let outside_state = OutsideStatues {
            left: CompositeShape::new(Shape::Triangle, Shape::Triangle),
            middle: CompositeShape::new(Shape::Square, Shape::Square),
            right: CompositeShape::new(Shape::Circle, Shape::Circle),
        };

        let inside_state = InsideStatues {
            left: Shape::Triangle,
            middle: Shape::Square,
            right: Shape::Circle,
        };

        let actual_solution = solve(inside_state, outside_state)?;
        let expected_solution: Vec<Exchange> = vec![
            UnorderedPair(
                (Statue::Left, Shape::Triangle),
                (Statue::Right, Shape::Circle),
            ),
            UnorderedPair(
                (Statue::Left, Shape::Triangle),
                (Statue::Middle, Shape::Square),
            ),
            UnorderedPair(
                (Statue::Middle, Shape::Square),
                (Statue::Right, Shape::Circle),
            ),
        ];

        assert_eq!(expected_solution, actual_solution);
        Ok(())
    }

    #[test]
    fn two_swap_case_with_all_wrong_doubles() -> Result<(), String> {
        let outside_state = OutsideStatues {
            left: CompositeShape::new(Shape::Triangle, Shape::Triangle),
            middle: CompositeShape::new(Shape::Square, Shape::Square),
            right: CompositeShape::new(Shape::Circle, Shape::Circle),
        };

        let inside_state = InsideStatues {
            left: Shape::Square,
            middle: Shape::Circle,
            right: Shape::Triangle,
        };

        let actual_solution = solve(inside_state, outside_state)?;
        let expected_solution: Vec<Exchange> = vec![
            UnorderedPair(
                (Statue::Left, Shape::Triangle),
                (Statue::Right, Shape::Circle),
            ),
            UnorderedPair(
                (Statue::Middle, Shape::Square),
                (Statue::Right, Shape::Triangle),
            ),
        ];

        assert_eq!(expected_solution, actual_solution);
        Ok(())
    }

    #[test]
    fn two_swap_case_with_two_wrong_doubles() -> Result<(), String> {
        let outside_state = OutsideStatues {
            left: CompositeShape::new(Shape::Triangle, Shape::Triangle),
            middle: CompositeShape::new(Shape::Square, Shape::Square),
            right: CompositeShape::new(Shape::Circle, Shape::Circle),
        };

        let inside_state = InsideStatues {
            left: Shape::Triangle,
            middle: Shape::Circle,
            right: Shape::Square,
        };

        let actual_solution = solve(inside_state, outside_state)?;
        let expected_solution: Vec<Exchange> = vec![
            UnorderedPair(
                (Statue::Left, Shape::Triangle),
                (Statue::Right, Shape::Circle),
            ),
            UnorderedPair(
                (Statue::Left, Shape::Triangle),
                (Statue::Middle, Shape::Square),
            ),
        ];

        assert_eq!(expected_solution, actual_solution);
        Ok(())
    }
}

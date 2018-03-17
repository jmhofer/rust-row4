use std::f64;

use row4::*;
use row4::board::Board;

type Evaluate = fn(&Board, Color, Color, u32) -> (f64, u64);

// TODO use main variant
// TODO alpha beta pruning
// TODO cache evaluations
pub fn minmax(board: &Board, own_color: Color, color_to_move: Color, depth: u8, evaluate: Evaluate) -> (Vec<Column>, f64, u64) {
    match board.winner {
        Some(color) if color == own_color => return (Vec::new(), 1.0, 0),
        Some(_) => return (Vec::new(), 0.0, 0),
        None => if board.moves.len() == 0 {
            return (Vec::new(), 0.5, 0);
        }
    };

    if depth == 0 {
        let (result, moves_played) = evaluate(board, own_color, color_to_move, 10);
        return (Vec::new(), result, moves_played);
    }

    let mut variants = Vec::new();
    let mut num_moves = 0;

    for column in monte_carlo::useful_moves(board, color_to_move) {
        let mut sim = board.clone();
        sim.play_move(color_to_move, column, true);
        let (mut variant, eval, moves) = minmax(&sim, own_color, color_to_move.switch(), depth - 1, evaluate);
        num_moves += moves;
        variant.push(column);
        variants.push((variant, eval));
    }
    let (best_variant, best_eval) = min_or_max(variants, own_color, color_to_move);

    (best_variant, best_eval, num_moves)
}

fn min_or_max(variants: Vec<(Vec<Column>, f64)>, own_color: Color, color_to_move: Color) -> (Vec<Column>, f64) {
    let mut best_variant = Vec::new();
    let mut best_eval= if own_color == color_to_move {
        -1.0
    } else {
        2.0
    };

    for (variant, eval) in variants {
        if own_color == color_to_move && eval > best_eval || own_color != color_to_move && eval < best_eval {
            best_eval = eval;
            best_variant = variant;
        }
    }

    (best_variant, best_eval)
}

#[test]
fn test_min_or_max_max() {
    assert_eq!(
        min_or_max(vec!(),Color::Red, Color::Red),
        (vec!(), -1.0));
    assert_eq!(
        min_or_max(vec!((vec!(1), 0.3), (vec!(2, 1), 0.5), (vec!(3), 0.0)), Color::Red, Color::Red),
        (vec!(2, 1), 0.5));
}

#[test]
fn test_min_or_max_min() {
    assert_eq!(
        min_or_max(vec!(),Color::Blue, Color::Red),
        (vec!(), 2.0));
    assert_eq!(
        min_or_max(vec!((vec!(1), 1.0), (vec!(2, 1), 0.5), (vec!(3), 0.05)), Color::Blue, Color::Red),
        (vec!(3), 0.05));
}

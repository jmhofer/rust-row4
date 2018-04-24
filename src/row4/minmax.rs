use std::f64;

use row4::*;
use row4::board::Board;
use row4::cache::BoardCache;
use row4::time::Timer;

type Evaluate = fn(&Board, Color, u32) -> (f64, u64);

const GAMES_PER_EVALUATION: u32 = 80;

pub fn iterative_minmax(board: &Board, own_color: Color, millis: u64, cache: &mut BoardCache, evaluate: Evaluate) -> (Vec<Column>, f64, u64, u64) {
    let mut depth = 0;
    let mut main_variant = vec!();
    let mut elapsed = 0;
    let mut current_eval = 0.0;
    let mut current_moves_played = 0;
    let mut current_positions = 0;

    let timer = Timer::new();

    while elapsed < millis * 1_000 && main_variant.len() >= depth as usize {
        depth += 1;
        let (updated_main_variant, eval, moves_played, positions) =
            minmax(board, own_color, depth, &main_variant, -1.0, 2.0, cache, evaluate);
        elapsed = timer.elapsed_micros();
        main_variant = updated_main_variant;
        current_eval = eval;
        current_moves_played += moves_played;
        current_positions += positions;

        let (moves_per_second, positions_per_second) = if elapsed != 0 {
            ((current_moves_played * 1_000_000) / elapsed, (current_positions * 1_000_000) / elapsed)
        } else {
            (0, 0)
        };
        println!("depth: {}, elapsed: {} ms, moves: {} ({} moves/s), positions: {} ({} positions/s), eval: {}, variant: {:?}",
                 depth, elapsed, current_moves_played, moves_per_second, current_positions, positions_per_second, eval, main_variant);
    }

    (main_variant, current_eval, current_moves_played, current_positions)
}

pub fn minmax(board: &Board, own_color: Color, depth: u8, main_variant: &[Column], mut alpha: f64, mut beta: f64, cache: &mut BoardCache, evaluate: Evaluate) -> (Vec<Column>, f64, u64, u64) {
    match board.winner {
        Some(color) if color == own_color => return (Vec::new(), 1.0, 0, 1),
        Some(_) => return (Vec::new(), 0.0, 0, 1),
        None => if board.moves.len() == 0 {
            return (Vec::new(), 0.5, 0, 1);
        }
    };

    if depth == 0 {
        let (result, moves_played) = match cache.get(board) {
            Some(&eval) => (eval, 0),
            None => {
                let (eval, moves_played) = evaluate(board, own_color, GAMES_PER_EVALUATION);
                cache.store(board, eval, true);
                (eval, moves_played)
            }
        };
        return (Vec::new(), result, moves_played, 1);
    }

    let maximize = own_color == board.color_to_move;
    let mut best_variant = Vec::new();
    let mut best_eval = if maximize { -1.0 } else { 2.0 };
    let mut num_moves = 0;
    let mut num_positions = 0;

    let mut moves = monte_carlo::useful_moves(board);
    let mut updated_main_variant = put_main_variant_first(&mut moves, main_variant);

    for column in moves {
        let mut sim = board.clone();
        sim.play_move(column, true);
        let (mut variant, eval, moves, positions) = minmax(&sim, own_color, depth - 1, &updated_main_variant,alpha, beta, cache, evaluate);
        updated_main_variant = vec!();
        variant.push(column);

        num_moves += moves;
        num_positions += positions;

        if maximize {
            if eval > best_eval {
                best_eval = eval;
                best_variant = variant;
            }
            alpha = f64::max(alpha, eval);
            if beta <= alpha {
                break; // beta cut-off
            }
        } else {
            if eval < best_eval {
                best_eval = eval;
                best_variant = variant;
            }
            beta = f64::min(beta, eval);
            if beta <= alpha {
                break; // alpha cut-off
            }
        }
    }

    (best_variant, best_eval, num_moves, num_positions)
}

fn put_main_variant_first(moves: &mut Vec<Column>, main_variant: &[Column]) -> Vec<Column> {
    let mut updated_main_variant = main_variant.to_vec();
    match updated_main_variant.pop() {
        None => (),
        Some(column) => {
            moves.retain(|&c| c != column);
            moves.insert(0, column);
        }
    }

    updated_main_variant
}

#[test]
fn test_put_main_variant_first_for_empty_main_variant() {
    let mut moves = vec!(3, 4, 2, 1);
    assert_eq!(put_main_variant_first(&mut moves, &[]), vec!());
    assert_eq!(moves, vec!(3, 4, 2, 1));
}

#[test]
fn test_put_main_variant_first_for_single_element_main_variant() {
    let mut moves = vec!(3, 4, 2, 1);
    assert_eq!(put_main_variant_first(&mut moves, &[2]), vec!());
    assert_eq!(moves, vec!(2, 3, 4, 1));
}

#[test]
fn test_put_main_variant_first_for_overlapping_main_variant() {
    let mut moves = vec!(3, 4, 2, 1);
    assert_eq!(put_main_variant_first(&mut moves, &[1, 3, 3, 2, 1]), vec!(1, 3, 3, 2));
    assert_eq!(moves, vec!(1, 3, 4, 2));
}

#[test]
fn test_put_main_variant_first_for_disjunct_main_variant() { // should never really happen
    let mut moves = vec!(3, 4, 2, 1);
    assert_eq!(put_main_variant_first(&mut moves, &[1, 3, 3, 2, 1, 5]), vec!(1, 3, 3, 2, 1));
    assert_eq!(moves, vec!(5, 3, 4, 2, 1));
}

use std::f64;

use row4::*;
use row4::board::Board;
use row4::time::Timer;

type Evaluate = fn(&Board, Color, Color, u32) -> (f64, u64);

pub fn iterative_minmax(board: &Board, own_color: Color, color_to_move: Color, millis: u64, evaluate: Evaluate) -> (Vec<Column>, f64, u64) {
    let mut depth = 0;
    let mut main_variant = vec!();
    let mut elapsed = 0;
    let mut current_eval = 0.0;
    let mut current_moves_played = 0;
    let timer = Timer::new();

    while elapsed < millis && main_variant.len() >= depth as usize {
        depth += 1;
        let (updated_main_variant, eval, moves_played) =
            minmax(board, own_color, color_to_move, depth, &main_variant, -1.0, 2.0, evaluate);
        elapsed = timer.elapsed_millis();
        main_variant = updated_main_variant;
        current_eval = eval;
        current_moves_played += moves_played;
        println!("depth: {}, elapsed: {} ms, moves: {} ({} moves/s), eval: {}, variant: {:?}",
                 depth, elapsed, moves_played, (moves_played * 1_000) / elapsed, eval, main_variant);
    }

    (main_variant, current_eval, current_moves_played)
}

// TODO cache evaluations
pub fn minmax(board: &Board, own_color: Color, color_to_move: Color, depth: u8, main_variant: &[Column], mut alpha: f64, mut beta: f64, evaluate: Evaluate) -> (Vec<Column>, f64, u64) {
    match board.winner {
        Some(color) if color == own_color => return (Vec::new(), 1.0, 0),
        Some(_) => return (Vec::new(), 0.0, 0),
        None => if board.moves.len() == 0 {
            return (Vec::new(), 0.5, 0);
        }
    };

    if depth == 0 {
        let (result, moves_played) = evaluate(board, own_color, color_to_move, 100);
        return (Vec::new(), result, moves_played);
    }

    let maximize = own_color == color_to_move;
    let mut best_variant = Vec::new();
    let mut best_eval = if maximize { -1.0 } else { 2.0 };
    let mut num_moves = 0;

    let mut moves = monte_carlo::useful_moves(board, color_to_move);
    let mut updated_main_variant = main_variant.to_vec();
    match updated_main_variant.pop() {
        None => (),
        Some(column) => {
            moves.retain(|&c| c != column);
            moves.insert(0, column);
        }
    }
    for column in moves {
        let mut sim = board.clone();
        sim.play_move(color_to_move, column, true);
        let (mut variant, eval, moves) = minmax(&sim, own_color, color_to_move.switch(), depth - 1, &updated_main_variant,alpha, beta, evaluate);
        num_moves += moves;
        variant.push(column);

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

    (best_variant, best_eval, num_moves)
}

use row4::*;
use row4::board::Board;

use rand::{thread_rng, Rng};
use std::time::SystemTime;

/// choose the best next move, evaluating all next moves using monte carlo simulations
/// this is just a temporary variant of the simplest possible minmax search
pub fn choose_next_move(board: &Board, own_color: Color, color_to_move: Color, millis: u32) -> (Column, f64, u64) {
    let mut max: (Option<Column>, f64) = (None, -1.0);
    let mut total = 0u64;

    let useful_moves = useful_moves(board, color_to_move);
    let num_moves = useful_moves.len();

    for column in useful_moves {
        let mut sim = board.clone();
        sim.play_move(color_to_move, column, false);
        let (wins, computed) = evaluate(&sim, own_color, color_to_move.switch(), millis / num_moves as u32);
        total += computed;
        if wins > max.1 {
            max = (Some(column), wins)
        }
    }
    (max.0.unwrap(), max.1, total)
}

/// evaluate the current position, using monte carlo simulation
pub fn evaluate(board: &Board, own_color: Color, color_to_move: Color, millis: u32) -> (f64, u64) {
    fn millis_since(start: SystemTime) -> u32 {
        let elapsed = SystemTime::now().duration_since(start).unwrap();
        elapsed.as_secs() as u32 * 1_000 + elapsed.subsec_nanos() / 1_000_000
    }

    let start = SystemTime::now();

    let mut wins = 0u32;
    let mut losses = 0u32;
    let mut draws = 0u32;
    let mut moves = 0u64;

    while millis_since(start) < millis {
        let mut sim = board.clone();
        let (variant, result) = play_random_game(&mut sim, color_to_move.clone());
        match result {
            Some(color) if color == own_color => wins += 1,
            Some(_) => losses += 1,
            None => draws += 1
        }
        moves += variant.len() as u64;
    }

    let total = wins + losses + draws;
    (wins as f64 / total as f64, moves)
}

// play a random game
fn play_random_game(board: &mut Board, mut color_to_move: Color) -> (Vec<Column>, Option<Color>) {
    let mut protocol = Vec::new();
    loop {
        let moves = useful_moves(&board, color_to_move);
        match thread_rng().choose(&moves) {
            None => return (protocol, None),
            Some(&column) => {
                protocol.push(column);
                board.play_move(color_to_move, column, true);
                if board.winner.is_some() {
                    return (protocol, board.winner);
                }
            }
        }
        color_to_move = color_to_move.switch();
    }
}

// restrict full list of moves to only moves that win, or do not lose immediately
pub fn useful_moves(board: &Board, color_to_move: Color) -> Vec<Column> {
    // check if we have won with any of the moves
    for column in board.moves.moves() {
        let mut sim = board.clone();
        sim.play_move(color_to_move, column, false);
        if sim.winner.is_some() {
            return vec!(column);
        }
    }

    // check if the opponent would win with any of the moves
    for column in board.moves.moves() {
        let mut sim = board.clone();
        sim.play_move(color_to_move.switch(), column, false);
        if sim.winner.is_some() {
            return vec!(column);
        }
    }

    board.moves.moves()
}

#[test]
fn test_useful_moves_win() {
    let mut board = Board::new();
    board.play_moves(Color::Red, &vec!(4, 3, 4, 3, 4, 3));
    assert_eq!(useful_moves(&board, Color::Red), vec!(4));
}

#[test]
fn test_useful_moves_avoid_losing() {
    let mut board = Board::new();
    board.play_moves(Color::Red, &vec!(4, 3, 4, 3, 4));
    assert_eq!(useful_moves(&board,Color::Blue), vec!(4));
}

#[test]
fn test_useful_moves_normal() {
    let mut board = Board::new();
    board.play_moves(Color::Red, &vec!(4, 3, 4, 3, 5, 2));
    assert_eq!(useful_moves(&board, Color::Red), COLUMNS.to_vec());
}

use row4::*;
use row4::board::Board;

use rand::{thread_rng, Rng};
use std::thread;
use std::sync::mpsc;

const NUM_THREADS: usize = 4;

pub fn evaluate_in_parallel(board: &Board, own_color: Color, num_games: u32) -> (f64, u64) {
    let (sender, receiver) = mpsc::channel();

    for _core in 0..NUM_THREADS {
        let core_sender = sender.clone();
        let core_board = board.clone();
        thread::spawn(move || {
            let result = evaluate(&core_board, own_color, num_games / NUM_THREADS as u32);
            core_sender.send(result).unwrap();
        });
    }

    let mut sum_eval = 0.0;
    let mut sum_games = 0;
    for _core in 0..NUM_THREADS {
        let (eval, games) = receiver.recv().unwrap();
        sum_eval += eval;
        sum_games += games;
    }
    (sum_eval / NUM_THREADS as f64, sum_games)
}

/// evaluate the current position, using monte carlo simulation
pub fn evaluate(board: &Board, own_color: Color, num_games: u32) -> (f64, u64) {
    let mut wins = 0u32;
    let mut moves = 0u64;
    let mut total = 0u32;

    while total < num_games {
        let mut sim = board.clone();
        let (variant, result) = play_random_game(&mut sim);
        match result {
            Some(color) if color == own_color => wins += 1,
            _ => ()
        }
        total += 1;
        moves += variant.len() as u64;
    }

    (wins as f64 / total as f64, moves)
}

// play a random game
fn play_random_game(board: &mut Board) -> (Vec<Column>, Option<Color>) {
    let mut protocol = Vec::new();
    loop {
        let moves = useful_moves(&board);
        match thread_rng().choose(&moves) {
            None => return (protocol, None),
            Some(&column) => {
                protocol.push(column);
                board.play_move(column, true);
                if board.winner.is_some() {
                    return (protocol, board.winner);
                }
            }
        }
    }
}

// restrict full list of moves to only moves that win, or do not lose immediately
pub fn useful_moves(board: &Board) -> Vec<Column> {
    // check if we have won with any of the moves
    for column in board.moves.moves() {
        let mut sim = board.clone();
        sim.play_move(column, false);
        if sim.winner.is_some() {
            return vec!(column);
        }
    }

    // check if the opponent would win with any of the moves
    for column in board.moves.moves() {
        let mut sim = board.clone();
        sim.color_to_move = sim.color_to_move.switch();
        sim.play_move(column, false);
        if sim.winner.is_some() {
            return vec!(column);
        }
    }

    board.moves.moves()
}

#[test]
fn test_useful_moves_win() {
    let mut board = Board::new();
    board.play_moves(&vec!(4, 3, 4, 3, 4, 3));
    assert_eq!(useful_moves(&board), vec!(4));
}

#[test]
fn test_useful_moves_avoid_losing() {
    let mut board = Board::new();
    board.play_moves(&vec!(4, 3, 4, 3, 4));
    assert_eq!(useful_moves(&board), vec!(4));
}

#[test]
fn test_useful_moves_normal() {
    let mut board = Board::new();
    board.play_moves(&vec!(4, 3, 4, 3, 5, 2));
    assert_eq!(useful_moves(&board), COLUMNS.to_vec());
}

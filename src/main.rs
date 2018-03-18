pub mod row4;

#[macro_use]
extern crate lazy_static;
extern crate rand;

use std::io;

use row4::*;
use row4::board::Board;
use row4::cache::BoardCache;

fn main() {
    play_game();
}

fn play_game() {
    let mut board = Board::new();
    let mut cache = BoardCache::new();
    let ai_color = Color::Red;

    while board.winner.is_none() {
        // AI move
        {
            let (variant, eval, num_moves, num_positions) =
                row4::minmax::iterative_minmax(&board, ai_color, 5_000, &mut cache, monte_carlo::evaluate_in_parallel);

            board.play_move(*variant.last().unwrap(), true);
            let mut print_variant = variant.clone();
            print_variant.reverse();
            print_variant = print_variant.iter().map(|&c| c + 1 ).collect();

            println!("ai moves: {:?}, win rate: {} (moves: {}, positions: {})\n{}\n", print_variant, eval, num_moves, num_positions, board);
        }

        if board.winner.is_some() {
            break;
        }

        // player move
        println!("Your move: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let column = input.trim().parse::<Column>().unwrap() - 1;

        board.play_move(column, true);
        println!("player move: {}\n{}\n", column, board);
    }
}

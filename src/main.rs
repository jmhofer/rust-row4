pub mod row4;

#[macro_use]
extern crate lazy_static;
extern crate rand;

use std::io;

use row4::*;
use row4::board::Board;

fn main() {
    play_game();
}

fn play_game() {
    let mut board = Board::new();
    let ai_color = Color::Red;
    let player_color = Color::Blue;
    let allowance = 1000;

    while board.winner.is_none() {
        // AI move
        {
            let (variant, eval, num_moves) =
                row4::minmax::minmax(&board, ai_color, ai_color, 3, monte_carlo::evaluate);

            board.play_move(ai_color, *variant.last().unwrap(), true);
            let mut print_variant = variant.clone();
            print_variant.reverse();
            print_variant = print_variant.iter().map(|&c| c + 1 ).collect();

            println!("ai moves: {:?}, win rate: {} ({})\n{}\n", print_variant, eval, num_moves, board);
        }

        if board.winner.is_some() {
            break;
        }

        // player move
        println!("Your move: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let column = input.trim().parse::<Column>().unwrap() - 1;

        board.play_move(player_color, column, true);
        println!("player move: {}\n{}\n", column, board);
    }
}

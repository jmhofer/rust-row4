pub mod row4;

#[macro_use]
extern crate lazy_static;
extern crate rand;

use std::io;

use row4::Column;
use row4::board::Board;
use row4::board::Color;

fn main() {
    let mut board = Board::new();
    let ai_color = Color::Red;
    let player_color = Color::Blue;
    let allowance = 1000;

    while board.winner.is_none() {
        // AI move
        {
            let (column, win_rate, games_played) =
                board.choose_monte_carlo(ai_color, ai_color, allowance);
            board.play_move(ai_color, column, true);
            println!("ai move: {}, win rate: {} ({})\n{}\n", column + 1, win_rate, games_played, board);
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

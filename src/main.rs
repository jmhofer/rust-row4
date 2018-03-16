pub mod row4;

use row4::Board;
use row4::Color;

fn main() {
    let mut board = Board::new();
    println!("{:?}", board.moves());
    println!("{}{:?}", board, board);

    board = board.play_move(Color::Red, 3);
    println!("{}{:?}", board, board);

    board = board.play_move(Color::Blue, 3);
    println!("{}{:?}", board, board);

    board = board.play_move(Color::Red, 4);
    println!("{}{:?}", board, board);
}

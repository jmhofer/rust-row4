pub mod row4;

use row4::Board;
use row4::Color;

fn main() {
    let mut board = Board::new();
    println!("{:?}", board.moves());
    println!("{}\n{:?}", board, board);

    board.play_move(Color::Red, 3);
    println!("{}\n{:?}", board, board);

    board.play_move(Color::Blue, 3);
    println!("{}\n{:?}", board, board);

    board.play_move(Color::Red, 4);
    println!("{}\n{:?}", board, board);

    board.play_moves(Color::Blue, vec!(5, 3, 3));
    println!("{}\n{:?}", board, board);
}

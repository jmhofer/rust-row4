pub mod row4;

use row4::Board;

fn main() {
    println!("Hello, world!");

    let board = Board::new();
    println!("{:?}", board.moves());
    println!("{:?}", board);
}

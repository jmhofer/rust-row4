use std::fmt;

use row4::*;
use row4::move_list::MoveList;

lazy_static! {
    static ref WIN_MASKS: Vec<u64> = Board::compute_win_masks();
}

/// A simple row4 board. Each color is represented as a bit array.
///
/// Bits 0..6 of the first byte are used as the lowest row of the board,
/// bits 0..6 of the second byte as the second-lowest row, and so on.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Board {
    pub red: u64,
    pub blue: u64,
    pub column_heights: [Column; 7],
    pub color_to_move: Color,
    pub moves: MoveList,
    pub winner: Option<Color>,
}

impl Board {
    /// creates an empty board
    pub fn new() -> Board {
        Board {
            red: 0,
            blue: 0,
            column_heights: [0; 7],
            color_to_move: Color::Red,
            moves: MoveList::full(),
            winner: None
        }
    }

    /// utility to complete a bit mask from a given position
    fn position_mask(column: Column, height: u8) -> u64 {
        1u64 << (height * 8 + column)
    }

    fn compute_win_masks() -> Vec<u64> {
        let mut masks = Vec::new();
        // vertical row4s
        for col in 0..7u8 {
            for row in 0..3u8 {
                let mut mask = 0u64;
                for idx in 0..4u8 {
                    mask |= Board::position_mask(col, row + idx);
                }
                masks.push(mask);
            }
        }
        // horizontal row4s
        for row in 0..6u8 {
            for col in 0..4u8 {
                let mut mask = 0u64;
                for idx in 0..4u8 {
                    mask |= Board::position_mask(col + idx, row);
                }
                masks.push(mask);
            }
        }
        // diagonal row4s
        for row in 0..3u8 {
            for col in 0..4u8 {
                let mut mask_left = 0u64;
                let mut mask_right = 0u64;
                for idx in 0..4u8 {
                    mask_left |= Board::position_mask(col + idx, row + idx);
                    mask_right |= Board::position_mask(col + 3 - idx, row + idx);
                }
                masks.push(mask_left);
                masks.push(mask_right);
            }
        }
        masks
    }

    /// play a series of moves
    pub fn play_moves(&mut self, moves: &[Column]) {
        for &column in moves {
            self.play_move(column, true);
        }
    }

    /// play a move in the specified column
    /// does not check if the column is legal - this has to be done beforehand!
    pub fn play_move(&mut self, column: Column, gen_next_moves: bool) {
        let height = self.height(column);
        let mask = Board::position_mask(column, height);

        self.column_heights[column as usize] = height + 1;

        match self.color_to_move {
            Color::Red => self.red = self.red | mask,
            Color::Blue => self.blue = self.blue | mask
        };

        if gen_next_moves {
            self.moves = MoveList::from(&self.compute_moves());
        }
        self.color_to_move = self.color_to_move.switch();
        self.winner = self.compute_winner();
    }

    /// computes all currently available moves for this board
    fn compute_moves(&self) -> Vec<Column> {
        let mut moves = Vec::new();
        for i in 0..7usize {
            let column = COLUMNS[i];
            if self.height(column) < 6 {
                moves.push(column);
            }
        }
        moves
    }

    /// returns the height of the specified column
    pub fn height(&self, column: Column) -> u8 {
        self.column_heights[column as usize]
    }

    /// checks if one of the players has won
    fn compute_winner(&self) -> Option<Color> { // TODO this has lots of optimization potential
        for &mask in WIN_MASKS.iter() {
            if self.red & mask == mask {
                return Some(Color::Red);
            }
            if self.blue & mask == mask {
                return Some(Color::Blue);
            }
        }
        None
    }

    /// reset the board to an empty one
    pub fn reset(&mut self) {
        self.red = 0;
        self.blue = 0;
        self.column_heights = [0; 7];
        self.color_to_move = Color::Red;
        self.moves = MoveList::full();
        self.winner = None;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        let mut board = Vec::<String>::new();
        for row in 0..6u8 {
            let mut row_string = String::new();
            for column in 0..7u8 {
                let mask = Board::position_mask(column, 5 - row);
                if self.red & mask != 0 {
                    row_string.push('x');
                } else if self.blue & mask != 0 {
                    row_string.push('o');
                } else {
                    row_string.push('.');
                }
                row_string.push(' ');
            }
            board.push(row_string);
        }
        write!(dest, "{}", board.join("\n").as_str())
    }
}

#[test]
fn test_position_mask() {
    assert_eq!(Board::position_mask(3, 1), 2048);
    assert_eq!(Board::position_mask(6, 5), 1 << (8 * 5 + 6));
}

#[test]
fn test_height() {
    let board = Board { red: 0, blue: 0, column_heights: [0, 6, 5, 3, 1, 6, 0], color_to_move: Color::Red, moves: MoveList::full(), winner: None };
    assert_eq!(board.height(0), 0);
    assert_eq!(board.height(1), 6);
    assert_eq!(board.height(3), 3);
}

#[test]
fn test_moves() {
    let board = Board { red: 0, blue: 0, column_heights: [0, 6, 5, 3, 1, 6, 0], color_to_move: Color::Red, moves: MoveList::full(), winner: None };
    assert_eq!(board.compute_moves(), vec!(3, 2, 4, 0, 6));
}

#[test]
fn test_play_move() {
    let mut board = Board::new();
    board.red = 8;
    board.column_heights = [0, 0, 0, 1, 0, 0, 0];

    board.play_move(3, false);
    assert_eq!(board.color_to_move, Color::Blue);

    board.play_move(1, false);
    assert_eq!(board.color_to_move, Color::Red);

    assert_eq!(board.blue, 2);
    assert_eq!(board.red, 2056);
    assert_eq!(board.column_heights, [0, 1, 0, 2, 0, 0, 0]);
}

#[test]
fn test_play_moves() {
    let mut board = Board::new();
    board.red = 8;
    board.column_heights = [0, 0, 0, 1, 0, 0, 0];
    board.color_to_move = Color::Blue;

    board.play_moves(&vec!(3, 1));

    assert_eq!(board.color_to_move, Color::Blue);
    assert_eq!(board.blue, 2048);
    assert_eq!(board.red, 10);
    assert_eq!(board.column_heights, [0, 1, 0, 2, 0, 0, 0]);
    assert_eq!(board.winner, None);
}

#[test]
fn test_winner() {
    let mut board = Board::new();
    board.play_moves(&vec!(4, 3, 4, 3, 4, 3, 4));
    assert_eq!(board.winner, Some(Color::Red));

    board.reset();
    board.color_to_move = Color::Blue;
    board.play_moves(&vec!(3, 4, 4, 3, 3, 4, 4, 3, 1, 2, 2));
    assert_eq!(board.winner, Some(Color::Blue));

    board.reset();
    board.color_to_move = Color::Blue;
    board.play_moves(&vec!(3, 4, 4, 3, 3, 4, 4, 3, 1, 1, 2, 2));
    assert_eq!(board.winner, None);
}

use std::fmt;

pub type Column = u8;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Color {
    Red,
    Blue
}

const COLUMNS: [Column; 7] = [3, 2, 4, 1, 5, 0, 6];

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
    pub column_heights: [Column; 7]
}

impl Board {
    /// creates an empty board
    pub fn new() -> Board {
        Board { red: 0, blue: 0, column_heights: [0; 7] }
    }

    /// utility to complete a bit mask from a given position
    pub fn position_mask(column: Column, height: u8) -> u64 {
        1u64 << (height * 8 + column)
    }

    pub fn switch_color(color: Color) -> Color {
        if color == Color::Red { Color::Blue } else { Color::Red }
    }

    pub fn compute_win_masks() -> Vec<u64> {
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

    pub fn play_moves(&mut self, mut color: Color, moves: Vec<Column>) {
        for column in moves {
            self.play_move(color, column);
            color = Board::switch_color(color);
        }
    }

    /// play a move in the specified column
    /// does not check if the column is legal - this has to be done beforehand!
    pub fn play_move(&mut self, color: Color, column: Column) {
        let height = self.height(column);
        let mask = Board::position_mask(column, height);

        self.column_heights[column as usize] = height + 1;

        match color {
            Color::Red => self.red = self.red | mask,
            Color::Blue => self.blue = self.blue | mask
        };
    }

    /// computes all currently available moves for this board
    pub fn moves(&self) -> Vec<Column> {
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
    pub fn has_won(&self) -> Option<Color> {
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

    pub fn reset(&mut self) {
        self.red = 0;
        self.blue = 0;
        self.column_heights = [0; 7];
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
}

#[test]
fn test_height() {
    let board = Board { red: 0, blue: 0, column_heights: [0, 6, 5, 3, 1, 6, 0] };
    assert_eq!(board.height(0), 0);
    assert_eq!(board.height(1), 6);
    assert_eq!(board.height(3), 3);
}

#[test]
fn test_moves() {
    let board = Board { red: 0, blue: 0, column_heights: [0, 6, 5, 3, 1, 6, 0] };
    assert_eq!(board.moves(), vec!(3, 2, 4, 0, 6));
}

#[test]
fn test_play_move() {
    let mut board = Board { red: 8, blue: 0, column_heights: [0, 0, 0, 1, 0, 0, 0] };
    board.play_move(Color::Red, 3);
    board.play_move(Color::Blue, 1);

    assert_eq!(board.blue, 2);
    assert_eq!(board.red, 2056);
    assert_eq!(board.column_heights, [0, 1, 0, 2, 0, 0, 0]);
}

#[test]
fn test_play_moves() {
    let mut board = Board { red: 8, blue: 0, column_heights: [0, 0, 0, 1, 0, 0, 0] };
    board.play_moves(Color::Blue, vec!(3, 1));

    assert_eq!(board.blue, 2048);
    assert_eq!(board.red, 10);
    assert_eq!(board.column_heights, [0, 1, 0, 2, 0, 0, 0]);
}

#[test]
fn test_has_won() {
    let mut board = Board::new();
    board.play_moves(Color::Red, vec!(4, 3, 4, 3, 4, 3, 4));
    assert_eq!(board.has_won(), Some(Color::Red));

    board.reset();
    board.play_moves(Color::Blue, vec!(3, 4, 4, 3, 3, 4, 4, 3, 1, 2, 2));
    assert_eq!(board.has_won(), Some(Color::Blue));

    board.reset();
    board.play_moves(Color::Blue, vec!(3, 4, 4, 3, 3, 4, 4, 3, 1, 1, 2, 2));
    assert_eq!(board.has_won(), None);
}

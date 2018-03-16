use std::fmt;

pub type Column = u8;

pub enum Color {
    Red,
    Blue
}

pub const COLUMNS: [Column; 7] = [3, 2, 4, 1, 5, 0, 6];

/// A simple row4 board. Each color is represented as a bit array.
///
/// Bits 0..6 of the first byte are used as the lowest row of the board,
/// bits 0..6 of the second byte as the second-lowest row, and so on.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Board {
    red: u64,
    blue: u64,
    column_heights: [Column; 7]
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

    /// play a move in the specified column
    /// does not check if the column is legal - this has to be done beforehand!
    pub fn play_move(&self, color: Color, column: Column) -> Board {
        let height = self.height(column);
        let mask = Board::position_mask(column, height);

        let mut updated = self.clone();
        updated.column_heights[column as usize] = height + 1;

        match color {
            Color::Red => updated.red = updated.red | mask,
            Color::Blue => updated.blue = updated.blue | mask
        };

        updated
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
}

impl fmt::Display for Board {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..6u8 {
            for column in 0..7u8 {
                let mask = Board::position_mask(column, 5 - row);
                if self.red & mask != 0 {
                    write!(dest, "x ")?;
                } else if self.blue & mask != 0 {
                    write!(dest, "o ")?;
                } else {
                    write!(dest, ". ")?;
                }
            }
            write!(dest, "\n")?
        }
        Ok(())
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
    let board = Board { red: 8, blue: 0, column_heights: [0, 0, 0, 1, 0, 0, 0] };
    let updated = board
        .play_move(Color::Red, 3)
        .play_move(Color::Blue, 1);

    assert_eq!(updated.blue, 2);
    assert_eq!(updated.red, 2056);
    assert_eq!(updated.column_heights, [0, 1, 0, 2, 0, 0, 0]);
}

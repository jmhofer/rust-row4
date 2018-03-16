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
#[derive(Copy, Clone, Debug)]
pub struct Board {
    red: u64,
    blue: u64,
    column_heights: [Column; 7]
}

impl Board {
    pub fn new() -> Board {
        Board { red: 0, blue: 0, column_heights: [0; 7] }
    }

    pub fn position_mask(column: Column, height: u8) -> u64 {
        1u64 << (height * 8 + column)
    }

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
        for i in 0usize .. 7 {
            let column = COLUMNS[i];
            if self.height(column) < 6 {
                moves.push(column);
            }
        }
        moves
    }

    pub fn height(&self, column: Column) -> u8 {
        self.column_heights[column as usize]
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

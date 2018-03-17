use rand::{thread_rng, Rng};
use std::fmt;
use std::time::SystemTime;

use row4::Column;
use row4::COLUMNS;

use row4::move_list::MoveList;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Color {
    Red,
    Blue
}

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
    pub moves: MoveList,
    pub winner: Option<Color>
}

impl Board {
    /// creates an empty board
    pub fn new() -> Board {
        Board {
            red: 0,
            blue: 0,
            column_heights: [0; 7],
            moves: MoveList::full(),
            winner: None
        }
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

    pub fn choose_monte_carlo(&self, own_color: Color, color_to_move: Color, millis: u32) -> (Column, f64, u32) {
        let mut max: (Option<Column>, f64) = (None, -1.0);
        let mut total = 0u32;

        let useful_moves = self.useful_moves(color_to_move);
        let num_moves = useful_moves.len();

        for column in useful_moves {
            let mut sim = self.clone();
            sim.play_move(color_to_move, column, false);
            let (wins, computed) = sim.evaluate_monte_carlo(own_color, Board::switch_color(color_to_move), millis / num_moves as u32);
            total += computed;
            if wins > max.1 {
                max = (Some(column), wins)
            }
        }
        (max.0.unwrap(), max.1, total)
    }

    pub fn evaluate_monte_carlo(&self, own_color: Color, color_to_move: Color, millis: u32) -> (f64, u32) {
        fn millis_since(start: SystemTime) -> u32 {
            let elapsed = SystemTime::now().duration_since(start).unwrap();
            elapsed.as_secs() as u32 * 1_000 + elapsed.subsec_nanos() / 1_000_000
        }

        let start = SystemTime::now();

        let mut wins = 0u32;
        let mut losses = 0u32;
        let mut draws = 0u32;

        while millis_since(start) < millis {
            let mut sim = self.clone();
            match sim.play_random_game(color_to_move.clone()) {
                (_, Some(color)) if color == own_color => wins += 1,
                (_, Some(_)) => losses += 1,
                (_, None) => draws += 1
            }
        }

        let total = wins + losses + draws;
        (wins as f64 / total as f64, total)
    }

    pub fn play_random_game(&mut self, mut color_to_move: Color) -> (Vec<Column>, Option<Color>) {
        let mut protocol = Vec::new();
        loop {
            let moves = self.useful_moves(color_to_move);
            match thread_rng().choose(&moves) {
                None => return (protocol, None),
                Some(&column) => {
                    protocol.push(column);
                    self.play_move(color_to_move, column, true);
                    if self.winner.is_some() {
                        return (protocol, self.winner);
                    }
                }
            }
            color_to_move = Board::switch_color(color_to_move);
        }
    }

    /// restrict full movelist to only moves that win, or do not lose immediately
    pub fn useful_moves(&self, color_to_move: Color) -> Vec<Column> {
        // check if we have won with any of the moves
        for column in self.moves.moves() {
            let mut sim = self.clone();
            sim.play_move(color_to_move, column, false);
            if sim.winner.is_some() {
                return vec!(column);
            }
        }

        // check if the opponent would win with any of the moves
        for column in self.moves.moves() {
            let mut sim = self.clone();
            sim.play_move(Board::switch_color(color_to_move), column, false);
            if sim.winner.is_some() {
                return vec!(column);
            }
        }

        self.moves.moves()
    }

    pub fn play_moves(&mut self, mut color: Color, moves: &[Column]) {
        for &column in moves {
            self.play_move(color, column, true);
            color = Board::switch_color(color);
        }
    }

    /// play a move in the specified column
    /// does not check if the column is legal - this has to be done beforehand!
    pub fn play_move(&mut self, color: Color, column: Column, gen_next_moves: bool) {
        let height = self.height(column);
        let mask = Board::position_mask(column, height);

        self.column_heights[column as usize] = height + 1;

        match color {
            Color::Red => self.red = self.red | mask,
            Color::Blue => self.blue = self.blue | mask
        };

        if gen_next_moves {
            self.moves = MoveList::from(&self.moves());
        }
        self.winner = self.has_won();
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
}

#[test]
fn test_height() {
    let board = Board { red: 0, blue: 0, column_heights: [0, 6, 5, 3, 1, 6, 0], moves: MoveList::full(), winner: None };
    assert_eq!(board.height(0), 0);
    assert_eq!(board.height(1), 6);
    assert_eq!(board.height(3), 3);
}

#[test]
fn test_moves() {
    let board = Board { red: 0, blue: 0, column_heights: [0, 6, 5, 3, 1, 6, 0], moves: MoveList::full(), winner: None };
    assert_eq!(board.moves(), vec!(3, 2, 4, 0, 6));
}

#[test]
fn test_play_move() {
    let mut board = Board::new();
    board.red = 8;
    board.column_heights = [0, 0, 0, 1, 0, 0, 0];

    board.play_move(Color::Red, 3, false);
    board.play_move(Color::Blue, 1, false);

    assert_eq!(board.blue, 2);
    assert_eq!(board.red, 2056);
    assert_eq!(board.column_heights, [0, 1, 0, 2, 0, 0, 0]);
}

#[test]
fn test_play_moves() {
    let mut board = Board::new();
    board.red = 8;
    board.column_heights = [0, 0, 0, 1, 0, 0, 0];

    board.play_moves(Color::Blue, &vec!(3, 1));

    assert_eq!(board.blue, 2048);
    assert_eq!(board.red, 10);
    assert_eq!(board.column_heights, [0, 1, 0, 2, 0, 0, 0]);
    assert_eq!(board.winner, None);
}

#[test]
fn test_winner() {
    let mut board = Board::new();
    board.play_moves(Color::Red, &vec!(4, 3, 4, 3, 4, 3, 4));
    assert_eq!(board.winner, Some(Color::Red));

    board.reset();
    board.play_moves(Color::Blue, &vec!(3, 4, 4, 3, 3, 4, 4, 3, 1, 2, 2));
    assert_eq!(board.winner, Some(Color::Blue));

    board.reset();
    board.play_moves(Color::Blue, &vec!(3, 4, 4, 3, 3, 4, 4, 3, 1, 1, 2, 2));
    assert_eq!(board.winner, None);
}

#[test]
fn test_useful_moves_win() {
    let mut board = Board::new();
    board.play_moves(Color::Red, &vec!(4, 3, 4, 3, 4, 3));
    assert_eq!(board.useful_moves(Color::Red), vec!(4));
}

#[test]
fn test_useful_moves_avoid_losing() {
    let mut board = Board::new();
    board.play_moves(Color::Red, &vec!(4, 3, 4, 3, 4));
    assert_eq!(board.useful_moves(Color::Blue), vec!(4));
}

#[test]
fn test_useful_moves_normal() {
    let mut board = Board::new();
    board.play_moves(Color::Red, &vec!(4, 3, 4, 3, 5, 2));
    assert_eq!(board.useful_moves(Color::Red), COLUMNS.to_vec());
}


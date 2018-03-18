pub mod board;
pub mod move_list;
pub mod monte_carlo;
pub mod minmax;
pub mod time;

pub type Column = u8;

const COLUMNS: [Column; 7] = [3, 2, 4, 1, 5, 0, 6];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Color {
    Red,
    Blue
}

impl Color {
    pub fn switch(self) -> Color {
        match self {
            Color::Red => Color::Blue,
            Color::Blue => Color::Red
        }
    }
}

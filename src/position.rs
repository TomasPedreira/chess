use crate::aux_func::{int_to_letter, letter_to_int};

#[derive(Clone)]
pub struct Position {
    pub column: char,
    pub row: i32,
}
impl Position {
    pub fn equals(&self, other: &Position) -> bool {
        self.column == other.column && self.row == other.row
    }
    pub fn is_within_bounds(&self) -> bool {
        let col: i32 = letter_to_int(self.column);
        let row: i32 = self.row;

        col < 9 && col > 0 && row < 9 && row > 0
    }
    pub fn next_move(&self, mov: &(i32, i32, bool)) -> Position {
        let next_col = letter_to_int(self.column) + mov.0;
        let next_row = self.row + mov.1;

        Position {
            column: int_to_letter(next_col),
            row: next_row,
        }
    }
}

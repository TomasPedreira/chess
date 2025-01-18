use std::collections::HashMap;

#[derive(Clone)]
pub struct Piece {
    pub name: String,
    pub white: bool,
    pub position: Position,
    pub ways_to_move: Vec<(i32, i32, bool)>, //  columns, rows, allows multiple moves
}

#[derive(Clone)]
pub struct Game {
    pub pieces: HashMap<(char, i32), Piece>,
    pub white_to_move: bool,
    pub kings: (Position, Position),
    pub has_king_moved: (bool, bool),
    pub has_rook_moved: ((bool, bool), (bool, bool)),
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub column: char,
    pub row: i32,
}
impl Position {
    pub fn equals(&self, other: &Position) -> bool {
        self.column == other.column && self.row == other.row
    }
}

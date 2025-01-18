use crate::position::Position;

#[derive(Clone)]
pub struct Piece {
    pub name: String,
    pub white: bool,
    pub position: Position,
    pub ways_to_move: Vec<(i32, i32, bool)>, //  columns, rows, allows multiple moves
}
impl Piece {
}

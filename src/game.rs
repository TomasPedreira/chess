use std::collections::HashMap;

use crate::aux_func::{int_to_letter, letter_to_int};
use crate::piece::Piece;
use crate::position::Position;

#[derive(Clone)]
pub struct Game {
    pub pieces: HashMap<(char, i32), Piece>,
    pub white_to_move: bool,
    pub kings: (Position, Position),
    pub has_king_moved: (bool, bool),
    pub has_rook_moved: ((bool, bool), (bool, bool)),
}
impl Game {
    pub fn update_piece(&mut self, piece: &Piece, pos: Position) {
        let init_pos: Position = piece.position.clone();
        if piece.name.clone() == *"king" {
            if piece.white {
                self.kings.0 = pos.clone();
                self.has_king_moved.0 = true;
            } else {
                self.kings.1 = pos.clone();
                self.has_king_moved.1 = true;
            }
        }
        if piece.name.clone() == *"rook" {
            if piece.white
                && init_pos.equals(&Position {
                    column: 'A',
                    row: 1,
                })
            {
                self.has_rook_moved.0 .0 = true;
            } else if piece.white
                && init_pos.equals(&Position {
                    column: 'H',
                    row: 1,
                })
            {
                self.has_rook_moved.0 .1 = true;
            } else if !piece.white
                && init_pos.equals(&Position {
                    column: 'A',
                    row: 8,
                })
            {
                self.has_rook_moved.1 .0 = true;
            } else if !piece.white
                && init_pos.equals(&Position {
                    column: 'H',
                    row: 8,
                })
            {
                self.has_rook_moved.1 .1 = true;
            }
        }
        self.pieces
            .remove(&(piece.position.column, piece.position.row));
        self.pieces.insert(
            (pos.column, pos.row),
            Piece {
                name: piece.name.clone(),
                white: piece.white,
                position: pos,
                ways_to_move: piece.ways_to_move.clone(),
            },
        );
        self.pieces.remove(&(init_pos.column, init_pos.row));
    }

    pub fn can_make_single_move(
        &self,
        mov: &(i32, i32, bool),
        current_pos: &Position,
        end_pos: &Position,
        piece: &Piece,
        is_last: bool,
    ) -> bool {
        let is_white = &piece.white;
        let is_pawn = piece.name == *"pawn";
        if !end_pos.is_within_bounds() {
            return false;
        }
        if is_pawn && mov.1.abs() == 2 {
            if *is_white && current_pos.row != 2 {
                return false;
            }
            if !*is_white && current_pos.row != 7 {
                return false;
            }
        }
        let end_row: i32 = end_pos.row;
        let end_column: i32 = letter_to_int(end_pos.column);
        let start_column: i32 = letter_to_int(current_pos.column);
        let start_row: i32 = current_pos.row;
        let value: Option<&Piece> = self.pieces.get(&(end_pos.column, end_pos.row));

        if let Some(piece) = value {
            if &piece.white == is_white {
                return false;
            } else if !is_last {
                return false;
            }
            if is_pawn && (mov.0.abs() != mov.1.abs()) {
                return false;
            }
        } else if is_pawn && (mov.0.abs() == mov.1.abs()) {
            return false;
        }
        if end_column - start_column != mov.0 || end_row - start_row != mov.1 {
            return false;
        }
        true
    }

    pub fn can_make_multiple_move(
        &self,
        mov: &(i32, i32, bool),
        end_pos: &Position,
        piece: &Piece,
    ) -> bool {
        let mut cur_pos: Position = piece.position.clone();

        if piece.name == *"king" {
            if piece.white
                && ((self.has_king_moved.0
                    && (letter_to_int(end_pos.column) - letter_to_int(cur_pos.column)).abs() > 1)
                    || (mov.0 < 0 && self.has_rook_moved.0 .0)
                    || (mov.0 > 0 && self.has_rook_moved.0 .1))
            {
                println!("King has moved");
                return false;
            }
            if !piece.white
                && ((self.has_king_moved.0
                    && (letter_to_int(end_pos.column) - letter_to_int(cur_pos.column)).abs() > 1)
                    || (mov.0 < 0 && self.has_rook_moved.1 .0)
                    || (mov.0 > 0 && self.has_rook_moved.1 .1))
            {
                println!("King has moved");
                return false;
            }
        }
        let mut count: i32 = 0;

        while is_within_bounds(&cur_pos) {
            let next_pos: Position = cur_pos.next_move(mov);
            let is_last = next_pos.equals(end_pos);
            if !self.can_make_single_move(mov, &cur_pos, &next_pos, piece, is_last) {
                break;
            }
            if next_pos.equals(end_pos) {
                return true;
            }
            if piece.name == *"king" {
                count += 1;
                if count == 2 {
                    return next_pos.equals(end_pos);
                }
            }
            cur_pos = next_pos;
        }
        false
    }
    pub fn check_can_castle(
        &self,
        piece: &Piece,
        mov: &(i32, i32, bool),
    ) -> (bool, Position, Position) {
        if piece.white {
            if mov.0 > 0 {
                (
                    self.can_make_multiple_move(
                        &(1, 0, true),
                        &Position {
                            column: 'F',
                            row: 1,
                        },
                        self.pieces.get(&('H', 1)).unwrap(),
                    ),
                    Position {
                        column: 'F',
                        row: 1,
                    },
                    Position {
                        column: 'H',
                        row: 1,
                    },
                )
            } else {
                (
                    self.can_make_multiple_move(
                        &(-1, 0, true),
                        &Position {
                            column: 'D',
                            row: 1,
                        },
                        self.pieces.get(&('A', 1)).unwrap(),
                    ),
                    Position {
                        column: 'D',
                        row: 1,
                    },
                    Position {
                        column: 'A',
                        row: 1,
                    },
                )
            }
        } else if mov.0 > 0 {
            (
                self.can_make_multiple_move(
                    &(1, 0, true),
                    &Position {
                        column: 'F',
                        row: 8,
                    },
                    self.pieces.get(&('H', 8)).unwrap(),
                ),
                Position {
                    column: 'F',
                    row: 8,
                },
                Position {
                    column: 'H',
                    row: 8,
                },
            )
        } else {
            (
                self.can_make_multiple_move(
                    &(-1, 0, true),
                    &Position {
                        column: 'D',
                        row: 8,
                    },
                    self.pieces.get(&('A', 8)).unwrap(),
                ),
                Position {
                    column: 'D',
                    row: 8,
                },
                Position {
                    column: 'A',
                    row: 8,
                },
            )
        }
    }
    pub fn playable_pos(&self, piece: &Piece) -> Vec<Position> {
        let mut pos: Vec<Position> = Vec::<Position>::new();
        let mut cur_pos: Position = piece.position.clone();

        for mov in &piece.ways_to_move {
            if mov.2 {
                while cur_pos.is_within_bounds() {
                    let next_pos: Position = cur_pos.next_move(mov);
                    // let is_last = &next_pos == &cur_pos;
                    if self.can_make_single_move(mov, &cur_pos, &next_pos, piece, true) {
                        pos.push(next_pos.clone());
                    } else {
                        break;
                    }
                    cur_pos = next_pos;
                }
            } else {
                let next_pos = cur_pos.next_move(mov);
                if self.can_make_single_move(mov, &cur_pos, &next_pos, piece, true) {
                    pos.push(next_pos);
                }
            }
        }

        pos
    }

    pub fn is_draw(&self) -> bool {
        let mut white_count = [0, 0, 0, 0, 0, 0]; // pawn, rook, knight, bishop, queen, king
        let mut black_count = [0, 0, 0, 0, 0, 0]; // pawn, rook, knight, bishop, queen, king

        for val in self.pieces.clone() {
            let piece = val.1;
            if piece.white {
                match piece.name.as_str() {
                    "pawn" => white_count[0] += 1,
                    "rook" => white_count[1] += 1,
                    "knight" => white_count[2] += 1,
                    "bishop" => white_count[3] += 1,
                    "queen" => white_count[4] += 1,
                    "king" => white_count[5] += 1,
                    _ => {}
                }
            } else {
                match piece.name.as_str() {
                    "pawn" => black_count[0] += 1,
                    "rook" => black_count[1] += 1,
                    "knight" => black_count[2] += 1,
                    "bishop" => black_count[3] += 1,
                    "queen" => black_count[4] += 1,
                    "king" => black_count[5] += 1,
                    _ => {}
                }
            }
        }
        let total_white = white_count.iter().sum::<i32>();
        let total_black = black_count.iter().sum::<i32>();
        if total_white == 1 && total_black == 1 {
            return true;
        }
        if total_white == 2 && total_black == 1 && (white_count[2] == 1 || white_count[3] == 1) {
            return true;
        }
        if total_black == 2 && total_white == 1 && (black_count[2] == 1 || black_count[3] == 1) {
            return true;
        }
        if total_white == 2
            && total_black == 2
            && (white_count[2] == 1 || white_count[3] == 1)
            && (black_count[2] == 1 || black_count[3] == 1)
        {
            return true;
        }
        return false;
    }



    /*
    Checks if King_color is in check
    */
    pub fn is_in_check(&self, king_color: bool) -> bool {
        let king: &Position;
        if king_color {
            king = &self.kings.0;
        } else {
            king = &self.kings.1;
        }

        for val in &self.pieces {
            let piece = val.1;
            if piece.white == king_color {
                continue;
            }
            for mov in &piece.ways_to_move {
                if mov.2 {
                    if self.can_make_multiple_move(mov, king, piece) {
                        return true;
                    }
                } else if self.can_make_single_move(mov, &piece.position, king, piece, true) {
                    return true;
                }
            }
        }
        false
    }
    pub fn is_move_legal(&self, piece: &Piece, end_pos: Position) -> (bool, Position, Position) {
        let current_pos: Position = piece.position.clone();
        let move_coll: &Vec<(i32, i32, bool)> = &piece.ways_to_move;
        let mut can_make: bool = false;
        let mut can_castle: (bool, Position, Position) = (
            false,
            Position {
                column: 'Z',
                row: -1,
            },
            Position {
                column: 'Z',
                row: -1,
            },
        );
        for mov in move_coll {
            if mov.2 {
                if self.can_make_multiple_move( mov, &end_pos, piece) {
                    can_make = true;
                    if piece.name == *"king"
                        && (letter_to_int(piece.position.column) - letter_to_int(end_pos.column)).abs()
                            >= 2
                    {
                        can_castle = self.check_can_castle( piece, mov);
                    }
                    break;
                }
            } else if self.can_make_single_move( mov, &current_pos, &end_pos, piece, true) {
                can_make = true;
                break;
            }
        }
        if !can_make {
            return (
                false,
                Position {
                    column: can_castle.1.column,
                    row: can_castle.1.row,
                },
                Position {
                    column: can_castle.2.column,
                    row: can_castle.2.row,
                },
            );
        }
        let mut copy_game: Game = self.clone();
        let copy_piece: Piece = piece.clone();
        let color: bool = copy_piece.white;
    
        if can_castle.0 {
            copy_game.update_piece(
                self.pieces
                    .get(&(can_castle.1.column, can_castle.1.row))
                    .unwrap(),
                end_pos.clone(),
            );
        }
        copy_game = copy_game.clone();
        copy_game.update_piece(&copy_piece, end_pos);
    
        if copy_game.is_in_check( color) {
            // println!("Can't move into check");
            return (false, can_castle.1, can_castle.2);
        }
        //println!("moved piece from {}{} to {}{}, color of king = {}", copy_piece.position.0, copy_piece.position.1, end_pos.0, end_pos.1, color);
        (true, can_castle.1, can_castle.2)
    }

    pub fn is_mate(&self) -> i32 {
        for val in &self.pieces {
            if val.1.white == self.white_to_move {
                for pos in self.playable_pos(&val.1) {
                    let res = self.is_move_legal(&val.1, pos);
                    if res.0 {
                        return 0;
                    }
                }
            }
        }
        if self.is_in_check(self.white_to_move) {
            return 1;
        }
        2
    }
}

pub fn init_pieces() -> Game {
    // Create an empty board with default pieces
    let mut piece_map: HashMap<(char, i32), Piece> = HashMap::<(char, i32), Piece>::new();

    for i in 1..=8 {
        // White pawns init: from 'A' to 'H' in row 2 (index 1)
        let white_pawn: Piece = Piece {
            name: "pawn".to_string(),
            white: true,
            position: Position {
                column: int_to_letter(i),
                row: 2,
            },
            ways_to_move: vec![(0, 1, false), (1, 1, false), (-1, 1, false), (0, 2, false)],
        };
        piece_map.insert((int_to_letter(i), 2), white_pawn);

        // Black pawns init: from 'A' to 'H' in row 7 (index 6)
        let black_pawn: Piece = Piece {
            name: "pawn".to_string(),
            white: false,
            position: Position {
                column: int_to_letter(i),
                row: 7,
            },
            ways_to_move: vec![
                (0, -1, false),
                (-1, -1, false),
                (1, -1, false),
                (0, -1, true),
            ],
        };
        piece_map.insert((int_to_letter(i), 7), black_pawn);

        if i == 1 || i == 8 {
            // White rooks init: from 'A' to 'H' in row 1 (index 0)
            let white_rook: Piece = Piece {
                name: "rook".to_string(),
                white: true,
                position: Position {
                    column: int_to_letter(i),
                    row: 1,
                },
                ways_to_move: vec![(-1, 0, true), (0, -1, true), (1, 0, true), (0, 1, true)],
            };
            piece_map.insert((int_to_letter(i), 1), white_rook);

            // Black rooks init: from 'A' to 'H' in row 8 (index 7)
            let black_rook: Piece = Piece {
                name: "rook".to_string(),
                white: false,
                position: Position {
                    column: int_to_letter(i),
                    row: 8,
                },
                ways_to_move: vec![(-1, 0, true), (0, -1, true), (1, 0, true), (0, 1, true)],
            };
            piece_map.insert((int_to_letter(i), 8), black_rook);
        } else if i == 2 || i == 7 {
            // White knights init: from 'A' to 'H' in row 1 (index 0)
            let white_knight: Piece = Piece {
                name: "knight".to_string(),
                white: true,
                position: Position {
                    column: int_to_letter(i),
                    row: 1,
                },
                ways_to_move: vec![
                    (2, 1, false),
                    (2, -1, false),
                    (-2, 1, false),
                    (-2, -1, false),
                    (1, 2, false),
                    (-1, 2, false),
                    (1, -2, false),
                    (-1, -2, false),
                ],
            };
            piece_map.insert((int_to_letter(i), 1), white_knight);

            // Black knights init: from 'A' to 'H' in row 8 (index 7)
            let black_knight: Piece = Piece {
                name: "knight".to_string(),
                white: false,
                position: Position {
                    column: int_to_letter(i),
                    row: 8,
                },
                ways_to_move: vec![
                    (2, 1, false),
                    (2, -1, false),
                    (-2, 1, false),
                    (-2, -1, false),
                    (1, 2, false),
                    (-1, 2, false),
                    (1, -2, false),
                    (-1, -2, false),
                ],
            };
            piece_map.insert((int_to_letter(i), 8), black_knight);
        } else if i == 3 || i == 6 {
            // White bishops init: from 'A' to 'H' in row 1 (index 0)
            let white_bishop: Piece = Piece {
                name: "bishop".to_string(),
                white: true,
                position: Position {
                    column: int_to_letter(i),
                    row: 1,
                },
                ways_to_move: vec![(1, 1, true), (1, -1, true), (-1, 1, true), (-1, -1, true)],
            };
            piece_map.insert((int_to_letter(i), 1), white_bishop);

            // Black bishops init: from 'A' to 'H' in row 8 (index 7)
            let black_bishop: Piece = Piece {
                name: "bishop".to_string(),
                white: false,
                position: Position {
                    column: int_to_letter(i),
                    row: 8,
                },
                ways_to_move: vec![(1, 1, true), (1, -1, true), (-1, 1, true), (-1, -1, true)],
            };
            piece_map.insert((int_to_letter(i), 8), black_bishop);
        } else if i == 5 {
            // White king init: from 'A' to 'H' in row 1 (index 0)
            let white_king: Piece = Piece {
                name: "king".to_string(),
                white: true,
                position: Position {
                    column: int_to_letter(i),
                    row: 1,
                },
                ways_to_move: vec![
                    (1, 1, false),
                    (1, -1, false),
                    (-1, 1, false),
                    (-1, -1, false),
                    (1, 0, true),
                    (-1, 0, true),
                    (0, 1, false),
                    (0, -1, false),
                ],
            };
            piece_map.insert((int_to_letter(i), 1), white_king);
            // Black king init: from 'A' to 'H' in row 8 (index 7)
            let black_king: Piece = Piece {
                name: "king".to_string(),
                white: false,
                position: Position {
                    column: int_to_letter(i),
                    row: 8,
                },
                ways_to_move: vec![
                    (1, 1, false),
                    (1, -1, false),
                    (-1, 1, false),
                    (-1, -1, false),
                    (1, 0, true),
                    (-1, 0, true),
                    (0, 1, false),
                    (0, -1, false),
                ],
            };
            piece_map.insert((int_to_letter(i), 8), black_king);
        } else {
            // White queen init: from 'A' to 'H' in row 1 (index 0)
            let white_queen: Piece = Piece {
                name: "queen".to_string(),
                white: true,
                position: Position {
                    column: int_to_letter(i),
                    row: 1,
                },
                ways_to_move: vec![
                    (1, 1, true),
                    (1, -1, true),
                    (-1, 1, true),
                    (-1, -1, true),
                    (1, 0, true),
                    (-1, 0, true),
                    (0, 1, true),
                    (0, -1, true),
                ],
            };
            piece_map.insert((int_to_letter(i), 1), white_queen);

            // Black queen init: from 'A' to 'H' in row 8 (index 7)
            let black_queen: Piece = Piece {
                name: "queen".to_string(),
                white: false,
                position: Position {
                    column: int_to_letter(i),
                    row: 8,
                },
                ways_to_move: vec![
                    (1, 1, true),
                    (1, -1, true),
                    (-1, 1, true),
                    (-1, -1, true),
                    (1, 0, true),
                    (-1, 0, true),
                    (0, 1, true),
                    (0, -1, true),
                ],
            };
            piece_map.insert((int_to_letter(i), 8), black_queen);
        }
    }
    Game {
        pieces: piece_map,
        white_to_move: true,
        kings: (
            Position {
                column: 'E',
                row: 1,
            },
            Position {
                column: 'E',
                row: 8,
            },
        ),
        has_king_moved: (false, false),
        has_rook_moved: ((false, false), (false, false)),
    }
}
fn is_within_bounds(pos: &Position) -> bool {
    let col: i32 = letter_to_int(pos.column);
    let row: i32 = pos.row;

    col < 9 && col > 0 && row < 9 && row > 0
}

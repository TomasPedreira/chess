use std::collections::HashMap;
use std::io;
use std::process::Command;

mod classes;
use classes::{Game, Piece, Position};

fn letter_to_int(ch: char) -> i32 {
    ch as i32 - 64
}

fn int_to_letter(num: i32) -> char {
    ((num + 64) as u8) as char
}

fn init_pieces() -> Game {
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

fn can_make_single_move(
    game: &Game,
    mov: &(i32, i32, bool),
    current_pos: &Position,
    end_pos: &Position,
    piece: &Piece,
    is_last: bool,
) -> bool {
    let is_white = &piece.white;
    let is_pawn = piece.name == "pawn".to_string();
    if !is_within_bounds(&end_pos) {
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
    let value: Option<&Piece> = game.pieces.get(&(end_pos.column, end_pos.row));

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

fn next_move(mov: &(i32, i32, bool), current_pos: &Position) -> Position {
    let next_col = letter_to_int(current_pos.column) + mov.0;
    let next_row = current_pos.row + mov.1;

    Position {
        column: int_to_letter(next_col),
        row: next_row,
    }
}

fn can_make_multiple_move(
    game: &Game,
    mov: &(i32, i32, bool),
    end_pos: &Position,
    piece: &Piece,
) -> bool {
    let mut cur_pos: Position = piece.position.clone();

    if piece.name == "king".to_string() {
        if piece.white
            && ((game.has_king_moved.0
                && (letter_to_int(end_pos.column) - letter_to_int(cur_pos.column)).abs() > 1)
                || (mov.0 < 0 && game.has_rook_moved.0 .0)
                || (mov.0 > 0 && game.has_rook_moved.0 .1))
        {
            println!("King has moved");
            return false;
        }
        if !piece.white
            && ((game.has_king_moved.0
                && (letter_to_int(end_pos.column) - letter_to_int(cur_pos.column)).abs() > 1)
                || (mov.0 < 0 && game.has_rook_moved.1 .0)
                || (mov.0 > 0 && game.has_rook_moved.1 .1))
        {
            println!("King has moved");
            return false;
        }
    }
    let mut count: i32 = 0;

    while is_within_bounds(&cur_pos) {
        let next_pos: Position = next_move(mov, &cur_pos);
        let is_last = next_pos.equals(end_pos);
        if !can_make_single_move(game, mov, &cur_pos, &next_pos, piece, is_last) {
            break;
        }
        if &next_pos == end_pos {
            return true;
        }
        if piece.name == "king".to_string() {
            count += 1;
            if count == 2 {
                return &next_pos == end_pos;
            }
        }
        cur_pos = next_pos;
    }
    false
}

fn check_can_castle(
    game: &Game,
    piece: &Piece,
    mov: &(i32, i32, bool),
) -> (bool, Position, Position) {
    if piece.white {
        if mov.0 > 0 {
            (
                can_make_multiple_move(
                    game,
                    &(1, 0, true),
                    &Position {
                        column: 'F',
                        row: 1,
                    },
                    game.pieces.get(&('H', 1)).unwrap(),
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
                can_make_multiple_move(
                    game,
                    &(-1, 0, true),
                    &Position {
                        column: 'D',
                        row: 1,
                    },
                    game.pieces.get(&('A', 1)).unwrap(),
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
            can_make_multiple_move(
                game,
                &(1, 0, true),
                &Position {
                    column: 'F',
                    row: 8,
                },
                game.pieces.get(&('H', 8)).unwrap(),
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
            can_make_multiple_move(
                game,
                &(-1, 0, true),
                &Position {
                    column: 'D',
                    row: 8,
                },
                game.pieces.get(&('A', 8)).unwrap(),
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

// TODO implement casteling
fn is_move_legal(game: &Game, piece: &Piece, end_pos: Position) -> (bool, Position, Position) {
    let current_pos: Position = piece.position.clone();
    let move_coll: &Vec<(i32, i32, bool)> = &piece.ways_to_move;
    let mut can_make = false;
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
            if can_make_multiple_move(game, mov, &end_pos, piece) {
                can_make = true;
                if piece.name == *"king"
                    && (letter_to_int(piece.position.column) - letter_to_int(end_pos.column)).abs()
                        >= 2
                {
                    can_castle = check_can_castle(game, piece, mov);
                }
                break;
            }
        } else if can_make_single_move(game, mov, &current_pos, &end_pos, piece, true) {
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
    let mut copy_game = game.clone();
    let copy_piece = piece.clone();
    let color = copy_piece.white;

    if can_castle.0 {
        update_piece(
            &mut copy_game,
            game.pieces
                .get(&(can_castle.1.column, can_castle.1.row))
                .unwrap(),
            end_pos.clone(),
        );
    }
    update_piece(&mut copy_game, &copy_piece, end_pos);

    if is_in_check(&copy_game, color) {
        // println!("Can't move into check");
        return (false, can_castle.1, can_castle.2);
    }
    //println!("moved piece from {}{} to {}{}, color of king = {}", copy_piece.position.0, copy_piece.position.1, end_pos.0, end_pos.1, color);
    (true, can_castle.1, can_castle.2)
}

fn make_move(game: &mut Game, start_pos: Position, end_pos: Position) -> bool {
    let cloned_game: Game = game.clone();
    let moving_piece: Option<&Piece> = cloned_game.pieces.get(&(start_pos.column, start_pos.row));
    if let Some(piece) = moving_piece {
        if piece.white != game.white_to_move {
            return false;
        }
        let res: (bool, Position, Position) = is_move_legal(game, piece, end_pos.clone());
        if res.0 {
            update_piece(game, piece, end_pos);
            if res.1.column != 'Z' {
                println!("castling {}{}", res.1.column, res.1.row);
                let castle_piece: Piece =
                    game.pieces.get(&(res.2.column, res.2.row)).unwrap().clone();
                update_piece(game, &castle_piece, res.1);
            }
            game.white_to_move = !game.white_to_move;
            return true;
        } else {
            return false;
        }
    }
    println!("no piece found");
    false
}
fn update_piece(game: &mut Game, piece: &Piece, pos: Position) {
    let init_pos: Position = piece.position.clone();
    if piece.name.clone() == *"king" {
        if piece.white {
            game.kings.0 = pos.clone();
            game.has_king_moved.0 = true;
        } else {
            game.kings.1 = pos.clone();
            game.has_king_moved.1 = true;
        }
    }
    if piece.name.clone() == *"rook" {
        if piece.white
            && init_pos.equals(&Position {
                column: 'A',
                row: 1,
            })
        {
            game.has_rook_moved.0 .0 = true;
        } else if piece.white
            && init_pos.equals(&Position {
                column: 'H',
                row: 1,
            })
        {
            game.has_rook_moved.0 .1 = true;
        } else if !piece.white
            && init_pos.equals(&Position {
                column: 'A',
                row: 8,
            })
        {
            game.has_rook_moved.1 .0 = true;
        } else if !piece.white
            && init_pos.equals(&Position {
                column: 'H',
                row: 8,
            })
        {
            game.has_rook_moved.1 .1 = true;
        }
    }
    game.pieces
        .remove(&(piece.position.column, piece.position.row));
    game.pieces.insert(
        (pos.column, pos.row),
        Piece {
            name: piece.name.clone(),
            white: piece.white,
            position: pos,
            ways_to_move: piece.ways_to_move.clone(),
        },
    );
    game.pieces.remove(&(init_pos.column, init_pos.row));
}

fn print_board(game: &Game) {
    for i in (1..=8).rev() {
        for j in 1..=8 {
            let key: i32 = i;
            let value: Option<&Piece> = game.pieces.get(&(int_to_letter(j), key));
            if let Some(piece) = value {
                let padding: f32 = 10.0 - piece.name.len() as f32;
                let left_pd: i32 = (padding / 2.0).floor() as i32;
                let right_pd: i32 = (padding / 2.0).ceil() as i32;
                //print!("{} + {} = {}", left_pd, right_pd, piece.name.len() as i32 );
                let mut color: String = "b".to_string();
                if piece.white {
                    color = "w".to_string();
                }
                print!(
                    "{}{}{}{}",
                    " ".repeat(left_pd as usize),
                    color,
                    piece.name,
                    " ".repeat(right_pd as usize)
                );
            } else {
                let st: String = ".".to_string();
                print!("     {}     ", st);
            }
        }
        println!("{}\n", i);
    }
    for i in 1..=8 {
        let st = int_to_letter(i);
        print!("     {}     ", st);
    }
    println!();
}

fn get_user_pos() -> Option<(char, i32)> {
    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).expect("lolada");
    match input_str.to_uppercase().trim().chars().collect::<Vec<_>>()[..] {
        [letter, num] => {
            if num.is_numeric() {
                Some((letter, (num) as i32 - '0' as i32))
            } else {
                None
            }
        }
        _ => None,
    }
}

/*
Checks if King_color is in check
*/
fn is_in_check(game: &Game, king_color: bool) -> bool {
    let king: &Position;
    if king_color {
        king = &game.kings.0;
    } else {
        king = &game.kings.1;
    }

    for val in &game.pieces {
        let piece = val.1;
        if piece.white == king_color {
            continue;
        }
        for mov in &piece.ways_to_move {
            if mov.2 {
                if can_make_multiple_move(game, mov, king, piece) {
                    return true;
                }
            } else if can_make_single_move(game, mov, &piece.position, king, piece, true) {
                return true;
            }
        }
    }
    false
}

fn playable_pos(game: &Game, piece: &Piece) -> Vec<Position> {
    let mut pos: Vec<Position> = Vec::<Position>::new();
    let mut cur_pos: Position = piece.position.clone();

    for mov in &piece.ways_to_move {
        if mov.2 {
            while is_within_bounds(&cur_pos) {
                let next_pos: Position = next_move(mov, &cur_pos);
                // let is_last = &next_pos == &cur_pos;
                if can_make_single_move(game, mov, &cur_pos, &next_pos, piece, true) {
                    pos.push(next_pos.clone());
                } else {
                    break;
                }
                cur_pos = next_pos;
            }
        } else {
            let next_pos = next_move(mov, &cur_pos);
            if can_make_single_move(game, mov, &cur_pos, &next_pos, piece, true) {
                pos.push(next_pos);
            }
        }
    }

    pos
}
/*
Checks if King_color is in mate
*/
fn is_mate(game: &Game, king_color: bool) -> i32 {
    for val in &game.pieces {
        if val.1.white == king_color {
            for pos in playable_pos(game, val.1) {
                let res = is_move_legal(game, val.1, pos);
                if res.0 {
                    return 0;
                }
            }
        }
    }
    if is_in_check(game, king_color) {
        return 1;
    }
    2
}

fn check_draw(game: &Game) -> bool {
    let mut white_count = [0, 0, 0, 0, 0, 0]; // pawn, rook, knight, bishop, queen, king
    let mut black_count = [0, 0, 0, 0, 0, 0]; // pawn, rook, knight, bishop, queen, king

    for val in &game.pieces {
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

fn gaming() {
    println!("Game initialized!");
    let mut game = init_pieces();
    let mut res = Command::new("clear");
    res.status().expect("failed to clear");
    print_board(&game);

    loop {
        let player = if game.white_to_move {
            "White".to_string()
        } else {
            "Black".to_string()
        };
        println!("{} to play", player);
        let init_pos: Position;
        let end_pos: Position;
        match get_user_pos() {
            Some(pos) => {
                init_pos = Position {
                    column: pos.0,
                    row: pos.1,
                };
            }
            None => {
                return;
                // continue;
            }
        }
        match get_user_pos() {
            Some(pos) => {
                end_pos = Position {
                    column: pos.0,
                    row: pos.1,
                };
            }
            None => {
                println!("lolada2");
                continue;
            }
        }

        res.status().expect("failed to clear");
        if !make_move(&mut game, init_pos, end_pos) {
            print_board(&game);
            println!("Invalid move lol");
            continue;
        }

        print_board(&game);
        if is_mate(&game, game.white_to_move) == 1 {
            println!("{} is in check", game.white_to_move);
        }

        if is_mate(&game, game.white_to_move) == 1 {
            println!("Game over");
            println!("{} wins", player);

            break;
        }
        if is_mate(&game, game.white_to_move) == 2 {
            println!("Game over");
            println!("Stalemate");
            break;
        }
        if check_draw(&game) {
            println!("Game over");
            println!("Draw");
            break;
        }
    }
}

fn main() {
    gaming();
}

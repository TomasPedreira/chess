use std::collections::HashMap;
use std::io;
struct Piece {
    name: String,
    white: bool,
    position: (char, i32),
    ways_to_move: Vec<(i32, i32, bool)>, //  columns, rows, allows multiple moves
}

struct Game {
    pieces: HashMap<(char, i32), Piece>,
    white_to_move: bool,
    kings: ((char, i32), (char, i32)),
}

fn letter_to_int(ch: char) -> i32 {
    return ch as i32 - 64;
}

fn int_to_letter(num: i32) -> char {
    return ((num + 64) as u8) as char;
}

fn init_pieces() -> Game {
    // Create an empty board with default pieces
    let mut piece_map = HashMap::<(char, i32), Piece>::new();

    for i in 1..=8 {
        // White pawns init: from 'A' to 'H' in row 2 (index 1)
        let white_pawn = Piece {
            name: "pawn".to_string(),
            white: true,
            position: (int_to_letter(i), 2),
            ways_to_move: vec![(0, 1, false), (1, 1, false), (-1, 1, false), (0, 2, false)],
        };
        piece_map.insert((int_to_letter(i), 2), white_pawn);

        // Black pawns init: from 'A' to 'H' in row 7 (index 6)
        let black_pawn = Piece {
            name: "pawn".to_string(),
            white: false,
            position: (int_to_letter(i), 7),
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
            let white_rook = Piece {
                name: "rook".to_string(),
                white: true,
                position: (int_to_letter(i), 1),
                ways_to_move: vec![(-1, 0, true), (0, -1, true), (1, 0, true), (0, 1, true)],
            };
            piece_map.insert((int_to_letter(i), 1), white_rook);

            // Black rooks init: from 'A' to 'H' in row 8 (index 7)
            let black_rook = Piece {
                name: "rook".to_string(),
                white: false,
                position: (int_to_letter(i), 8),
                ways_to_move: vec![(-1, 0, true), (0, -1, true), (1, 0, true), (0, 1, true)],
            };
            piece_map.insert((int_to_letter(i), 8), black_rook);
        } else if i == 2 || i == 7 {
            // White knights init: from 'A' to 'H' in row 1 (index 0)
            let white_knight = Piece {
                name: "knight".to_string(),
                white: true,
                position: (int_to_letter(i), 1),
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
            let black_knight = Piece {
                name: "knight".to_string(),
                white: false,
                position: (int_to_letter(i), 8),
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
            let white_bishop = Piece {
                name: "bishop".to_string(),
                white: true,
                position: (int_to_letter(i), 1),
                ways_to_move: vec![(1, 1, true), (1, -1, true), (-1, 1, true), (-1, -1, true)],
            };
            piece_map.insert((int_to_letter(i), 1), white_bishop);

            // Black bishops init: from 'A' to 'H' in row 8 (index 7)
            let black_bishop = Piece {
                name: "bishop".to_string(),
                white: false,
                position: (int_to_letter(i), 8),
                ways_to_move: vec![(1, 1, true), (1, -1, true), (-1, 1, true), (-1, -1, true)],
            };
            piece_map.insert((int_to_letter(i), 8), black_bishop);
        } else if i == 5 {
            // White king init: from 'A' to 'H' in row 1 (index 0)
            let white_king = Piece {
                name: "king".to_string(),
                white: true,
                position: (int_to_letter(i), 1),
                ways_to_move: vec![
                    (1, 1, false),
                    (1, -1, false),
                    (-1, 1, false),
                    (-1, -1, false),
                    (1, 0, false),
                    (-1, 0, false),
                    (0, 1, false),
                    (0, -1, false),
                ],
            };
            piece_map.insert((int_to_letter(i), 1), white_king);
            // Black king init: from 'A' to 'H' in row 8 (index 7)
            let black_king = Piece {
                name: "king".to_string(),
                white: false,
                position: (int_to_letter(i), 8),
                ways_to_move: vec![
                    (1, 1, false),
                    (1, -1, false),
                    (-1, 1, false),
                    (-1, -1, false),
                    (1, 0, false),
                    (-1, 0, false),
                    (0, 1, false),
                    (0, -1, false),
                ],
            };
            piece_map.insert((int_to_letter(i), 8), black_king);
        } else {
            // White queen init: from 'A' to 'H' in row 1 (index 0)
            let white_queen = Piece {
                name: "queen".to_string(),
                white: true,
                position: (int_to_letter(i), 1),
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
            let black_queen = Piece {
                name: "queen".to_string(),
                white: false,
                position: (int_to_letter(i), 8),
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
    return Game {
        pieces: piece_map,
        white_to_move: true,
        kings: (('E', 1), ('E', 8)),
    };
}

fn is_within_bounds(pos: &(char, i32)) -> bool {
    let col: i32 = letter_to_int(pos.0);
    let row: i32 = pos.1;

    return col < 9 && col > 0 && row < 9 && row > 0;
}

// TODO pawn movement fml
fn can_make_single_move(
    game: &Game,
    mov: &(i32, i32, bool),
    current_pos: &(char, i32),
    end_pos: &(char, i32),
    is_white: &bool,
    is_pawn: bool,
) -> bool {
    if !is_within_bounds(end_pos) {
        return false;
    }
    if is_pawn && mov.1.clone().abs() == 2 {
        if *is_white && current_pos.1.clone() != 2 {
            return false;
        }
        if !*is_white && current_pos.1.clone() != 7 {
            return false;
        }
    }
    let end_row: i32 = end_pos.1;
    let end_column: i32 = letter_to_int(end_pos.0.clone());
    let start_column: i32 = letter_to_int(current_pos.0);
    let start_row = current_pos.1;
    let value = game.pieces.get(end_pos);

    if let Some(piece) = value {
        if &piece.white == is_white {
            return false;
        }
        if is_pawn && (mov.0.clone().abs() != mov.1.clone().abs()) {
            return false;
        }
    } else if is_pawn && (mov.0.clone().abs() == mov.1.clone().abs()) {
        return false;
    }
    if end_column - start_column != mov.0 || end_row - start_row != mov.1 {
        return false;
    }
    return true;
}

fn next_move(mov: &(i32, i32, bool), current_pos: &(char, i32)) -> (char, i32) {
    let next_col = letter_to_int(current_pos.0) + mov.0;
    let next_row = current_pos.1 + mov.1;

    return (int_to_letter(next_col), next_row);
}

fn can_make_multiple_move(
    game: &Game,
    mov: &(i32, i32, bool),
    current_pos: &(char, i32),
    end_pos: &(char, i32),
    is_white: &bool,
    is_pawn: bool,
) -> bool {
    let mut cur_pos: (char, i32) = current_pos.clone();

    while is_within_bounds(&cur_pos) {
        let next_pos: (char, i32) = next_move(mov, &cur_pos);
        if !can_make_single_move(game, mov, &cur_pos, &next_pos, is_white, is_pawn) {
            break;
        }
        if &next_pos == end_pos {
            return true;
        }
        cur_pos = next_pos.clone();
    }
    return false;
}

fn is_move_legal(game: &Game, piece: &Piece, end_pos: (char, i32)) -> bool {
    let current_pos: (char, i32) = piece.position.clone();
    let move_coll: &Vec<(i32, i32, bool)> = &piece.ways_to_move;
    let is_pawn = &piece.name == "pawn";

    for mov in move_coll {
        if mov.2 {
            if can_make_multiple_move(game, mov, &current_pos, &end_pos, &piece.white, is_pawn) {
                return true;
            }
        } else {
            if can_make_single_move(game, mov, &current_pos, &end_pos, &piece.white, is_pawn) {
                return true;
            }
        }
    }

    return false;
}

fn make_move(game: &mut Game, start_pos: (char, i32), end_pos: (char, i32)) -> bool {
    let moving_piece: Option<&Piece> = game.pieces.get(&start_pos);
    if let Some(piece) = moving_piece {
        if piece.white != game.white_to_move {
            println!("Not your piece dumbass!");
            return false;
        }
        if is_move_legal(game, piece, end_pos) {
            println!("Moving: {} lolada", piece.name);
            let new_piece = Piece {
                name: piece.name.clone(),
                white: piece.white,
                position: end_pos,
                ways_to_move: piece.ways_to_move.clone(),
            };
            if new_piece.name.clone() == "king" {
                if new_piece.white {
                    game.kings.0 = end_pos;
                } else {
                    game.kings.1 = end_pos;
                }
            }
            game.pieces.insert(end_pos, new_piece);
            game.pieces.remove(&start_pos);
            
            if game.white_to_move {
                game.white_to_move = false;
            } else {
                game.white_to_move = true;
            }

            return true;
        } else {
            println!("ilegal move");
            return false;
        }
    } else {
        println!("Moving: Nothing idiot!");
    }
    return false;
}

fn print_board(game: &Game) {
    for i in (1..=8).rev() {
        for j in 1..=8 {
            let key = i as i32;
            let value = game.pieces.get(&(int_to_letter(j), key));
            if let Some(piece) = value {
                let padding = 10.0 - piece.name.len() as f32;
                let left_pd = (padding / 2.0).floor() as i32;
                let right_pd = (padding / 2.0).ceil() as i32;
                //print!("{} + {} = {}", left_pd, right_pd, piece.name.len() as i32 );
                let mut color = "b".to_string();
                if piece.white {
                    color = "w".to_string();
                }
                print!(
                    "{}{}{}{}",
                    " ".repeat(left_pd as usize).to_string(),
                    color,
                    piece.name,
                    " ".repeat(right_pd as usize).to_string()
                );
            } else {
                let st: String = ".".to_string();
                print!("     {}     ", st);
            }
        }
        println!("{}\n", i);
    }
    for i in 1..=8 {
        let st = int_to_letter(i as i32);
        print!("     {}     ", st);
    }
    println!("");
}

fn get_user_pos() -> Option<(char, i32)> {
    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).expect("lolada");
    match input_str.to_uppercase().trim().chars().collect::<Vec<_>>()[..] {
        [letter, num] => {
            if num.is_numeric() {
                return Some((letter, (num) as i32 - '0' as i32));
            } else {
                return None;
            }
        }
        _ => {
            return None;
        }
    }
}
fn is_in_check(game: &Game, king_color: bool) -> bool {
    let king: &(char, i32);
    if king_color {
        king = &game.kings.0;
    } else {
        king = &game.kings.1;
    }

    for val in &game.pieces {
        let piece = val.1;
        if piece.name == "king" {
            continue;
        }
        if piece.white == king_color {
            continue;
        }
        let is_white = !king_color;
        let is_pawn = piece.name == "pawn";
        for mov in &piece.ways_to_move {
            if mov.2 {
                if can_make_multiple_move(game, mov, &piece.position, king, &is_white, is_pawn) {
                    println!(
                        "{},{}{},{}{}",
                        piece.name, piece.position.0, piece.position.1, king.0, king.1
                    );
                    return true;
                }
            } else {
                if can_make_single_move(game, mov, &piece.position, king, &is_white, is_pawn) {
                    println!(
                        "{},{}{},{}{}",
                        piece.name, piece.position.0, piece.position.1, king.0, king.1
                    );
                    return true;
                }
            }
        }
    }
    return false;
}

fn main() {
    println!("Game initialized!");
    let mut game = init_pieces();

    print_board(&game);
    
    loop {
        let mut init_pos: (char, i32) = ('z', -1);
        let mut end_pos: (char, i32) = ('z', -1);
        match get_user_pos() {
            Some(pos) => {
                init_pos = pos;
            }
            None => {
                println!("lolada");
                continue;
            }
        }
        match get_user_pos() {
            Some(pos) => {
                end_pos = pos;
            }
            None => {
                println!("lolada");
                continue;
            }
        }
        if !make_move(&mut game, init_pos, end_pos) {
            return;
        }

        print_board(&game);
        let is_check = is_in_check(&game, game.white_to_move);
        println!("{}", is_check);
        
    }
}

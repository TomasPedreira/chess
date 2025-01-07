use std::collections::HashMap;

struct Piece {
    name: String,
    white: bool,
    position: (char, i32),
    ways_to_move: Vec<(i32, i32, bool)>, // how many squares vertically, horizontally, allows multiple moves
}

struct Game {
    pieces: HashMap<(char, i32), Piece>,
    white_to_move: bool,
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
            ways_to_move: vec![(0, 1, false), (1, 1, false), (-1, 1, false)],
        };
        piece_map.insert((int_to_letter(i), 2), white_pawn);

        // Black pawns init: from 'A' to 'H' in row 7 (index 6)
        let black_pawn = Piece {
            name: "pawn".to_string(),
            white: false,
            position: (int_to_letter(i), 7),
            ways_to_move: vec![(0, -1, false), (-1, -1, false), (1, -1, false)],
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
    };
}

fn is_within_bounds(pos: &(char, i32)) -> bool {
    let col: i32 = letter_to_int(pos.0);
    let row: i32 = pos.1;

    return col < 9 && col > 0 && row < 9 && row > 0;
}

fn can_make_single_move(
    game: &Game,
    mov: &(i32, i32, bool),
    current_pos: &(char, i32),
    end_pos: &(char, i32),
    is_white: &bool,
) -> bool {
    let end_row: i32 = end_pos.1;
    let end_column: i32 = letter_to_int(end_pos.0.clone());
    let start_column: i32 = letter_to_int(current_pos.0);
    let start_row = current_pos.1;
    let value = game.pieces.get(end_pos);
    if !is_within_bounds(end_pos){
        return false;
    }
    if let Some(piece) = value {
        if &piece.white == is_white {
            return false;
        }
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
) -> bool {
    let mut cur_pos: (char, i32) = current_pos.clone();

    while is_within_bounds(&cur_pos) {
        let next_pos: (char, i32) = next_move(mov, &cur_pos);
        if !can_make_single_move(game, mov, &cur_pos, &next_pos, is_white) {
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
    // (column, row)
    for mov in move_coll {
        if mov.2 {
            if can_make_multiple_move(game, mov, &current_pos, &end_pos, &piece.white) {
                println!("Correct {} move", piece.name);
                return true;
            }
        } else {
            if can_make_single_move(game, mov, &current_pos, &end_pos, &piece.white) {
                println!("Correct {} move", piece.name);
                return true;
            }
        }
    }
    println!("Incorrect {} move", piece.name);
    return false;
}

fn make_move(game: &mut Game, start_pos: (char, i32), end_pos: (char, i32)) -> bool {
    let moving_piece: Option<&Piece> = game.pieces.get(&start_pos);
    if let Some(piece) = moving_piece {
        if is_move_legal(game, piece, end_pos) {
            println!("Moving: {}", piece.name);
            let new_piece = Piece {
                name: piece.name.clone(),
                white: piece.white,
                position: end_pos,
                ways_to_move: piece.ways_to_move.clone(),
            };
            game.pieces.insert(end_pos, new_piece);
            game.pieces.remove(&start_pos);
            return true;
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
                if piece.white {
                    print!("w{:<10}", piece.name);
                } else {
                    print!("b{:<10}", piece.name);
                }
            } else {
                let st: String = ".".to_string();
                print!("{:<11}", st);
            }
        }
        println!("{}", i);
    }
    for i in 1..=8 {
        let st = int_to_letter(i as i32);
        print!("{:<11}", st);
    }
    println!("");
}

fn main() {
    //let game = init_pieces();
    println!("Game initialized!");
    let mut game = init_pieces();
    print_board(&game);
    make_move(&mut game, ('E', 2 as i32), ('E', 3 as i32));
    make_move(&mut game, ('D', 1 as i32), ('H', 5 as i32));
    print_board(&game);
}

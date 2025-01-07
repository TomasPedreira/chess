use std::collections::HashMap;

struct Piece {
    name: String,
    white: bool,
    position: (char, i32),
    ways_to_move: Vec<(i32, i32, bool)>, // how many squares vertically, horizontally, allows multiple moves
}

struct Game {
    pieces: HashMap<(char, i32), Piece>,
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
        } else if i == 4 {
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
    Game { pieces: piece_map }
}

fn print_board(game: &Game) {
    for i in (1..=8).rev() {
        for j in 1..=8 {
            let key = i as i32;
            let value = game.pieces.get(&(int_to_letter(j), key));
            if let Some(piece) = value {
                if piece.white{
                    print!("w{:<10}", piece.name);
                }else{
                    print!("b{:<10}", piece.name);
                }
                
            } else {
                let st: String = ".".to_string();
                print!("{:<11}", st); // Empty square
            }
        }
        println!("");
    }
}

fn main() {
    //let game = init_pieces();
    println!("Game initialized!");
    let mut game = init_pieces();
    print_board(&game);
    println!("Game Finished!");
}

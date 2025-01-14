use std::collections::HashMap;
use std::io;

#[derive(Clone)]
struct Piece {
    name: String,
    white: bool,
    position: (char, i32),
    ways_to_move: Vec<(i32, i32, bool)>, //  columns, rows, allows multiple moves
}

#[derive(Clone)]
struct Game {
    pieces: HashMap<(char, i32), Piece>,
    white_to_move: bool,
    kings: ((char, i32), (char, i32)),
    has_king_moved: (bool, bool),
    has_rook_moved: ((bool, bool), (bool, bool)),
}

fn letter_to_int(ch: char) -> i32 {
    ch as i32 - 64
}

fn int_to_letter(num: i32) -> char {
    ((num + 64) as u8) as char
}

fn give_ways_to_move_by_name(name: String) -> Vec<(i32, i32, bool)> {
    match name.as_str() {
        "pawn" => vec![(0, 1, false), (1, 1, false), (-1, 1, false), (0, 2, false)],
        "rook" => vec![(-1, 0, true), (0, -1, true), (1, 0, true), (0, 1, true)],
        "knight" => {
            vec![
                (2, 1, false),
                (2, -1, false),
                (-2, 1, false),
                (-2, -1, false),
                (1, 2, false),
                (-1, 2, false),
                (1, -2, false),
                (-1, -2, false),
            ]
        }
        "bishop" => vec![(1, 1, true), (1, -1, true), (-1, 1, true), (-1, -1, true)],
        "queen" => {
            vec![
                (1, 1, true),
                (1, -1, true),
                (-1, 1, true),
                (-1, -1, true),
                (1, 0, true),
                (-1, 0, true),
                (0, 1, true),
                (0, -1, true),
            ]
        }
        "king" => {
            vec![
                (1, 1, false),
                (1, -1, false),
                (-1, 1, false),
                (-1, -1, false),
                (1, 0, false),
                (-1, 0, false),
                (0, 1, false),
                (0, -1, false),
            ]
        }
        _ => vec![],
    }
}

fn create_piece_config(file_name: String) -> Vec<(String, bool, (char, i32))> {
    let mut pieces = Vec::<(String, bool, (char, i32))>::new();
    let file = std::fs::read_to_string(file_name).expect("lolada");
    for line in file.lines() {
        let mut split = line.split_whitespace();
        let name = split.next().unwrap().to_string();
        let color = split.next().unwrap().to_string();
        let pos = split.next().unwrap().to_string();
        let is_white = color == "white";
        let pos_char = pos.chars().next().unwrap();
        let pos_int = pos
            .chars()
            .nth(1)
            .unwrap()
            .to_string()
            .parse::<i32>()
            .unwrap();
        pieces.push((name.clone(), is_white, (pos_char, pos_int)));
    }
    pieces
}

fn init_config_pieces(pieces: Vec<(String, bool, (char, i32))>) -> Game {
    let mut piece_map = HashMap::<(char, i32), Piece>::new();
    let mut white_king: (char, i32) = ('E', 1);
    let mut black_king: (char, i32) = ('E', 8);
    for piece in pieces {
        let letter = piece.2 .0.to_uppercase().next().unwrap();
        let num = piece.2 .1;
        let new_piece = Piece {
            name: piece.0.clone(),
            white: piece.1,
            position: (letter, num),
            ways_to_move: give_ways_to_move_by_name(piece.0.clone()),
        };
        println!(
            "{} {} {}",
            new_piece.name.clone(),
            new_piece.position.0,
            new_piece.position.1
        );
        if new_piece.name.clone() == "king".to_string() {
            if new_piece.white {
                white_king = (letter, num);
            } else {
                black_king = (letter, num);
            }
        }
        piece_map.insert((letter, num), new_piece);
    }
    Game {
        pieces: piece_map,
        white_to_move: true,
        kings: (white_king, black_king),
        has_king_moved: (false, false),
        has_rook_moved: ((false, false), (false, false)),
    }
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
                    (1, 0, true),
                    (-1, 0, true),
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
                    (1, 0, true),
                    (-1, 0, true),
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
    Game {
        pieces: piece_map,
        white_to_move: true,
        kings: (('E', 1), ('E', 8)),
        has_king_moved: (false, false),
        has_rook_moved: ((false, false), (false, false)),
    }
}

fn is_within_bounds(pos: &(char, i32)) -> bool {
    let col: i32 = letter_to_int(pos.0);
    let row: i32 = pos.1;

    col < 9 && col > 0 && row < 9 && row > 0
}

fn can_make_single_move(
    game: &Game,
    mov: &(i32, i32, bool),
    current_pos: &(char, i32),
    end_pos: &(char, i32),
    piece: &Piece,
    is_last: bool,
) -> bool {
    let is_white = &piece.white;
    let is_pawn = &piece.name == "pawn";
    if !is_within_bounds(end_pos) {
        return false;
    }
    if is_pawn && mov.1.abs() == 2 {
        if *is_white && current_pos.1 != 2 {
            return false;
        }
        if !*is_white && current_pos.1 != 7 {
            return false;
        }
    }
    let end_row: i32 = end_pos.1;
    let end_column: i32 = letter_to_int(end_pos.0);
    let start_column: i32 = letter_to_int(current_pos.0);
    let start_row = current_pos.1;
    let value = game.pieces.get(end_pos);

    if let Some(piece) = value {
        if &piece.white == is_white {
            return false;
        }else if !is_last {
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

fn next_move(mov: &(i32, i32, bool), current_pos: &(char, i32)) -> (char, i32) {
    let next_col = letter_to_int(current_pos.0) + mov.0;
    let next_row = current_pos.1 + mov.1;

    (int_to_letter(next_col), next_row)
}

fn can_make_multiple_move(
    game: &Game,
    mov: &(i32, i32, bool),
    end_pos: &(char, i32),
    piece: &Piece,
) -> bool {
    let mut cur_pos: (char, i32) = piece.position;
    let mut count: i32 = 0;
    if piece.name == "king" {
        if piece.white
            && (game.has_king_moved.0
                || (mov.0 < 0 && game.has_rook_moved.0 .0)
                || (mov.0 > 0 && game.has_rook_moved.0 .1))
        {
            return false;
        }
        if !piece.white
            && (game.has_king_moved.1
                || (mov.0 < 0 && game.has_rook_moved.1 .0)
                || (mov.0 > 0 && game.has_rook_moved.1 .1))
        {
            return false;
        }
    }
    while is_within_bounds(&cur_pos) {
        if piece.name == "king" {
            count += 1;
            if count > 2 {
                return true;
            }
        }
        let next_pos: (char, i32) = next_move(mov, &cur_pos);
        let is_last = &next_pos == end_pos;
        if !can_make_single_move(game, mov, &cur_pos, &next_pos, piece, is_last) {
            break;
        }
        if &next_pos == end_pos {
            return true;
        }
        cur_pos = next_pos;
    }
    false
}

// TODO implement casteling
fn is_move_legal(game: &Game, piece: &Piece, end_pos: (char, i32)) -> bool {
    let current_pos: (char, i32) = piece.position;
    let move_coll: &Vec<(i32, i32, bool)> = &piece.ways_to_move;
    let mut can_make = false;
    for mov in move_coll {
        if mov.2 {
            if can_make_multiple_move(game, mov, &end_pos, piece) {
                can_make = true;
                break;
            }
        } else if can_make_single_move(game, mov, &current_pos, &end_pos, piece, true) {
            can_make = true;
            break;
        }
    }
    if !can_make {
        return false;
    }
    let mut copy_game = game.clone();
    let copy_piece = piece.clone();
    let color = copy_piece.white;

    copy_game.pieces.remove(&copy_piece.position);
    copy_game.pieces.insert(
        end_pos,
        Piece {
            name: copy_piece.name.clone(),
            white: copy_piece.white,
            position: end_pos,
            ways_to_move: copy_piece.ways_to_move.clone(),
        },
    );
    if copy_piece.name.clone() == "king" {
        if piece.white {
            copy_game.has_king_moved.0 = true;
            copy_game.kings.0 = end_pos;
        } else {
            copy_game.has_king_moved.1 = true;
            copy_game.kings.1 = end_pos;
        }
    }
    if copy_piece.name.clone() == "rook" {
        if piece.white && current_pos == ('A', 1) {
            copy_game.has_rook_moved.0 .0 = true;
        } else if piece.white && current_pos == ('H', 1) {
            copy_game.has_rook_moved.0 .1 = true;
        } else if !piece.white && current_pos == ('A', 8) {
            copy_game.has_rook_moved.1 .0 = true;
        } else if !piece.white && current_pos == ('H', 8) {
            copy_game.has_rook_moved.1 .1 = true;
        }
    }
    if is_in_check(&copy_game, color) {
        return false;
    }
    //println!("moved piece from {}{} to {}{}, color of king = {}", copy_piece.position.0, copy_piece.position.1, end_pos.0, end_pos.1, color);
    true
}

fn make_move(game: &mut Game, start_pos: (char, i32), end_pos: (char, i32)) -> bool {
    let cloned_game: Game = game.clone();
    let moving_piece: Option<&Piece> = cloned_game.pieces.get(&start_pos);
    if let Some(piece) = moving_piece {
        if piece.white != game.white_to_move {
            println!("Not your xolor");
            return false;
        }
        if is_move_legal(&game.clone(), piece, end_pos) {
            update_piece(game, piece, end_pos);
            game.white_to_move = !game.white_to_move;
            return true;
        } else {
            println!("Move ilegal");
            return false;
        }
    }
    println!("no piece found");
    false
}
fn update_piece(game: &mut Game, piece: &Piece, pos: (char, i32)) {
    let init_pos = piece.position;
    if piece.name.clone() == "king" {
        if piece.white {
            game.kings.0 = pos;
            game.has_king_moved.0 = true;
        } else {
            game.kings.1 = pos;
            game.has_king_moved.1 = true;
        }
    }
    if piece.name.clone() == "rook" {
        if piece.white && init_pos == ('A', 1) {
            game.has_rook_moved.0 .0 = true;
        } else if piece.white && init_pos == ('H', 1) {
            game.has_rook_moved.0 .1 = true;
        } else if !piece.white && init_pos == ('A', 8) {
            game.has_rook_moved.1 .0 = true;
        } else if !piece.white && init_pos == ('H', 8) {
            game.has_rook_moved.1 .1 = true;
        }
    }
    game.pieces.remove(&piece.position);
    game.pieces.insert(
        pos,
        Piece {
            name: piece.name.clone(),
            white: piece.white,
            position: pos,
            ways_to_move: piece.ways_to_move.clone(),
        },
    );
    game.pieces.remove(&init_pos);

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

/*
Checks if King_color is in check
*/
fn is_in_check(game: &Game, king_color: bool) -> bool {
    let king: &(char, i32);
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

fn playable_pos(game: &Game, piece: &Piece) -> Vec<(char, i32)> {
    let mut pos = Vec::<(char, i32)>::new();
    let mut cur_pos = piece.position;

    for mov in &piece.ways_to_move {
        if mov.2 {
            while is_within_bounds(&cur_pos) {
                let next_pos = next_move(mov, &cur_pos);
                // let is_last = &next_pos == &cur_pos;
                if can_make_single_move(game, mov, &cur_pos, &next_pos, piece, true) {
                    pos.push(next_pos);
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
                if is_move_legal(game, val.1, pos) {
                    println!(
                        "{} can move from {}{}, to {}{}",
                        val.1.name, val.1.position.0, val.1.position.1, pos.0, pos.1
                    );
                    return 0;
                }
            }
        }
    }
    if is_in_check(game, king_color) {
        return 1;
    }
    return 2;
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
    if total_white == 2 && total_black == 1 {
        if white_count[2] == 1 || white_count[3] == 1 {
            return true;
        }
    }
    if total_black == 2 && total_white == 1 {
        if black_count[2] == 1 || black_count[3] == 1 {
            return true;
        }
    }
    if total_white == 2 && total_black == 2 {
        if (white_count[2] == 1 || white_count[3] == 1)
            && (black_count[2] == 1 || black_count[3] == 1)
        {
            return true;
        }
    }
    return false;
}

fn gaming() {
    println!("Game initialized!");
    let mut game = init_pieces();

    print_board(&game);

    loop {
        let player = if game.white_to_move {
            "White".to_string()
        } else {
            "Black".to_string()
        };
        println!("{} to play", player);
        let init_pos: (char, i32);
        let end_pos: (char, i32);
        match get_user_pos() {
            Some(pos) => {
                init_pos = pos;
            }
            None => {
                println!("lolada1");
                return;
                // continue;
            }
        }
        match get_user_pos() {
            Some(pos) => {
                end_pos = pos;
            }
            None => {
                println!("lolada2");
                continue;
            }
        }
        if !make_move(&mut game, init_pos, end_pos) {
            print_board(&game);
            println!("Invalid move lol");
            continue;
        }

        print_board(&game);
        if is_mate(&game, false) == 1 {
            println!("{} is in check", false);
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

fn unit_test(file: String) {
    let piece_config = create_piece_config(file);
    println!("{}", piece_config.len());
    let game = init_config_pieces(piece_config);

    print_board(&game);

    // if is_mate(&game, false) == 1 {
    //     println!("Game over");
    //     println!("{} wins", true);
    // }
    // if is_mate(&game, false) == 2 {
    //     println!("Game over");
    //     println!("Stalemate");
    // }
    // if check_draw(&game) {
    //     println!("Game over");
    //     println!("Draw");
    // }
    if is_in_check(&game, false) {
        println!("Check");
    }else{
        println!("No check");
    }
}
fn main() {
    gaming();
    //unit_test("piece_config1.txt".to_string());
}

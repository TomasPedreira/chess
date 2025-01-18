// use std::collections::HashMap;
use std::io;
use std::process::Command;

mod aux_func;
mod game;
mod piece;
mod position;
use aux_func::{int_to_letter, letter_to_int};
use game::{init_pieces, Game};
use piece::Piece;
use position::Position;



// TODO implement casteling
fn is_move_legal(game: &Game, piece: &Piece, end_pos: Position) -> (bool, Position, Position) {
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
            if game.can_make_multiple_move( mov, &end_pos, piece) {
                can_make = true;
                if piece.name == *"king"
                    && (letter_to_int(piece.position.column) - letter_to_int(end_pos.column)).abs()
                        >= 2
                {
                    can_castle = game.check_can_castle( piece, mov);
                }
                break;
            }
        } else if game.can_make_single_move( mov, &current_pos, &end_pos, piece, true) {
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
    let mut copy_game: Game = game.clone();
    let copy_piece: Piece = piece.clone();
    let color: bool = copy_piece.white;

    if can_castle.0 {
        copy_game.update_piece(
            game.pieces
                .get(&(can_castle.1.column, can_castle.1.row))
                .unwrap(),
            end_pos.clone(),
        );
    }
    copy_game = copy_game.clone();
    copy_game.update_piece(&copy_piece, end_pos);

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
            game.update_piece(piece, end_pos);
            if res.1.column != 'Z' {
                println!("castling {}{}", res.1.column, res.1.row);
                let castle_piece: Piece =
                    game.pieces.get(&(res.2.column, res.2.row)).unwrap().clone();
                game.update_piece(&castle_piece, res.1);
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
                if game.can_make_multiple_move(mov, king, piece) {
                    return true;
                }
            } else if game.can_make_single_move(mov, &piece.position, king, piece, true) {
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
            while cur_pos.is_within_bounds() {
                let next_pos: Position = cur_pos.next_move(mov);
                // let is_last = &next_pos == &cur_pos;
                if game.can_make_single_move(mov, &cur_pos, &next_pos, piece, true) {
                    pos.push(next_pos.clone());
                } else {
                    break;
                }
                cur_pos = next_pos;
            }
        } else {
            let next_pos = cur_pos.next_move(mov);
            if game.can_make_single_move(mov, &cur_pos, &next_pos, piece, true) {
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

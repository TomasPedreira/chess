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



fn make_move(game: &mut Game, start_pos: Position, end_pos: Position) -> bool {
    let cloned_game: Game = game.clone();
    let moving_piece: Option<&Piece> = cloned_game.pieces.get(&(start_pos.column, start_pos.row));
    if let Some(piece) = moving_piece {
        if piece.white != game.white_to_move {
            return false;
        }
        let res: (bool, Position, Position) = game.is_move_legal( piece, end_pos.clone());
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
        if game.is_in_check( game.white_to_move) {
            println!("{} is in check", game.white_to_move);
        }

        if game.is_mate() == 1 {
            println!("Game over");
            println!("{} wins", player);

            break;
        }
        if game.is_mate() == 2 {
            println!("Game over");
            println!("Stalemate");
            break;
        }
        if game.is_draw() {
            println!("Game over");
            println!("Draw");
            break;
        }
    }
}

fn main() {
    gaming();
}

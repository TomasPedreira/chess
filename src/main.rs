// use std::collections::HashMap;
use std::io;
use std::process::Command;

mod aux_func;
mod game;
mod piece;
mod position;
use game::init_pieces;
use position::Position;







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
    game.print_board();

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
        if !game.make_move(init_pos, end_pos) {
            game.print_board();
            println!("Invalid move lol");
            continue;
        }

        game.print_board();
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

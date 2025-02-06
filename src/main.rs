use std::clone;
use std::sync::{Arc, Mutex};
use std::time::Duration;
// use std::collections::HashMap;
use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::{collections::HashMap, io};
mod aux_func;
mod game;
mod piece;
mod position;
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use game::{init_pieces, Game};
use position::Position;

#[derive(Serialize)] // This macro will allow us to convert this struct to JSON
struct Message {
    content: String,
}
#[derive(Deserialize, Debug)]
struct MoveRequest {
    start_pos: String,
    end_pos: String,
}
#[derive(Deserialize, Debug)]
struct ColorRequest {
    name: String,
    color: String,
}

#[derive(Clone)]
struct Players {
    white: String,
    black: String,
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
        if game.is_in_check(game.white_to_move) {
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

// fn main() {
//     gaming();
// }
async fn reset(
    game: web::Data<Arc<Mutex<Game>>>,
    players: web::Data<Arc<Mutex<Players>>>,
) -> impl Responder {
    println!("Received request in /reset");
    let mut game: std::sync::MutexGuard<'_, Game> = game.lock().unwrap();
    let mut players: std::sync::MutexGuard<'_, Players> = players.lock().unwrap();
    players.white = "none".to_string();
    players.black = "none".to_string();
    game.reset();
    let mut board: HashMap<String, String> = HashMap::new();

    for (pos, piece) in game.pieces.iter() {
        let color = if piece.white {
            "w".to_string()
        } else {
            "b".to_string()
        };
        board.insert(
            format!("{}{}", pos.0, pos.1),
            format!("{}{}", color, piece.name),
        );
    }
    board.insert(
        "to_move".to_string(),
        if game.white_to_move {
            "w".to_string()
        } else {
            "b".to_string()
        },
    );
    board.insert("bchosen".to_string(), players.black.clone());
    board.insert("wchosen".to_string(), players.white.clone());
    HttpResponse::Ok().json(board)
}

async fn move_piece(
    game: web::Data<Arc<Mutex<Game>>>,
    request: web::Json<MoveRequest>,
    players: web::Data<Arc<Mutex<Players>>>,
) -> impl Responder {
    println!("Received request: {:?}", request);
    let mut game: std::sync::MutexGuard<'_, Game> = game.lock().unwrap();
    let players: std::sync::MutexGuard<'_, Players> = players.lock().unwrap();
    let start_pos = Position {
        column: request.start_pos.chars().next().unwrap(),
        row: request.start_pos[1..].parse::<i32>().unwrap(),
    };
    let end_pos = Position {
        column: request.end_pos.chars().next().unwrap(),
        row: request.end_pos[1..].parse::<i32>().unwrap(),
    };
    let err = game.make_move(start_pos, end_pos);

    let mut response: HashMap<String, String> = HashMap::new();
    // fields: is_valid, is_mate, draw, check
    response.insert("is_valid".to_string(), err.to_string());
    response.insert("mate".to_string(), game.is_mate().to_string());
    response.insert("draw".to_string(), game.is_draw().to_string());
    response.insert(
        "check".to_string(),
        game.is_in_check(game.white_to_move).to_string(),
    );
    for (pos, piece) in game.pieces.iter() {
        let color = if piece.white {
            "w".to_string()
        } else {
            "b".to_string()
        };
        response.insert(
            format!("{}{}", pos.0, pos.1),
            format!("{}{}", color, piece.name),
        );
    }
    response.insert(
        "to_move".to_string(),
        if game.white_to_move {
            "w".to_string()
        } else {
            "b".to_string()
        },
    );
    response.insert("bchosen".to_string(), players.black.clone());
    response.insert("wchosen".to_string(), players.white.clone());
    HttpResponse::Ok().json(response)
}

async fn game_to_json(
    game: web::Data<Arc<Mutex<Game>>>,
    players: web::Data<Arc<Mutex<Players>>>,
) -> impl Responder {
    println!("Received request in /boardstate");
    let game: std::sync::MutexGuard<'_, Game> = game.lock().unwrap();
    let mut board: HashMap<String, String> = HashMap::new();
    let players: std::sync::MutexGuard<'_, Players> = players.lock().unwrap();

    for (pos, piece) in game.pieces.iter() {
        let color = if piece.white {
            "w".to_string()
        } else {
            "b".to_string()
        };
        board.insert(
            format!("{}{}", pos.0, pos.1),
            format!("{}{}", color, piece.name),
        );
    }

    board.insert(
        "to_move".to_string(),
        if game.white_to_move {
            "w".to_string()
        } else {
            "b".to_string()
        },
    );
    board.insert("bchosen".to_string(), players.black.clone());
    board.insert("wchosen".to_string(), players.white.clone());
    HttpResponse::Ok().json(board)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let game: Arc<Mutex<Game>> = Arc::new(Mutex::new(init_pieces()));
    let players: Arc<Mutex<Players>> = Arc::new(Mutex::new(Players {
        white: "none".to_string(),
        black: "none".to_string(),
    }));

    let ip = "127.0.0.1";

    let addr = format!("http://{}:8080", ip);
    println!("Server running at {}", addr);
    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .app_data(web::Data::new(game.clone()))
            .app_data(web::Data::new(players.clone()))
            //
            .route("/boardstate", web::get().to(game_to_json))
            .route("/movepiece", web::post().to(move_piece))
            .route("/reset", web::get().to(reset))
            .route("/black", web::post().to(choose_black))
            .route("/white", web::post().to(choose_white))
            .service(Files::new("/", "./ui").index_file("index.html"))
    })
    .bind((ip, 8080))?
    .run()
    .await
}

async fn choose_black(
    players: web::Data<Arc<Mutex<Players>>>,
    request: web::Json<ColorRequest>,
) -> impl Responder {
    println!("Received request in /black");
    let mut pla: std::sync::MutexGuard<'_, Players> = players.lock().unwrap();
    let mut map: HashMap<String, String> = HashMap::new();

    if pla.black != "none" {
        map.insert("status".to_string(), "taken".to_string());
        HttpResponse::Ok().json(map)
    } else {
        pla.black = request.name.clone();
        map.insert("status".to_string(), "chosen".to_string());
        HttpResponse::Ok().json(map)
    }
}
async fn choose_white(
    players: web::Data<Arc<Mutex<Players>>>,
    request: web::Json<ColorRequest>,
) -> impl Responder {
    println!("Received request in /white");
    let mut pla: std::sync::MutexGuard<'_, Players> = players.lock().unwrap();
    let mut map: HashMap<String, String> = HashMap::new();

    if pla.white != "none" {
        map.insert("status".to_string(), "taken".to_string());
        HttpResponse::Ok().json(map)
    } else {
        pla.white = request.name.clone();
        map.insert("status".to_string(), "chosen".to_string());
        HttpResponse::Ok().json(map)
    }
}

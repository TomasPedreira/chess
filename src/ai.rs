mod classes;
use classes::{Game, Piece, Position};

pub fn evaluate (game: &Game) -> i32 {
    let mut score = 0;
    for piece in game.pieces.values() {
        let mut piece_score = 0;
        if piece.white {
            piece_score += 1;
        } else {
            piece_score -= 1;
        }
        piece_score += match &piece.name[..] {
            "pawn" => 1,
            "knight" => 3,
            "bishop" => 3,
            "rook" => 5,
            "queen" => 9,
            "king" => 1000,
            _ => 0,
        };
        score += piece_score;
    }
    score
}
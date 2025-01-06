struct Piece {
    name: String,
    white: bool,
    position: (char, i32),
    ways_to_move: Vec<(i32, i32)>,
}

struct Game {
    pieces: [[Piece;8];8],
}

fn letter_to_int(ch: char)->i32{
    return ch as i32 - 64;
}
fn int_to_letter(num: i32)->char{
    return ((num + 64) as u8) as char;
}

fn init_pieces(game: &mut Game) {
    for i in 1..9 {
        // white pawns init: from 'A' to 'H' in row 2 (index 1)
        let pawn: Piece = Piece {
            name: "pawn".to_string(),
            white: true,
            position: (int_to_letter(i), 2),
            ways_to_move: [(1,0),(1,-1),(1,1)].to_vec(),
        };
        game.pieces[1][(i as usize)-1] = pawn;

        // Black pawns init: from 'A' to 'H' in row 7 (index 6)
        let pawn: Piece = Piece {
            name: "pawn".to_string(),
            white: false,
            position: (int_to_letter(i), 2),
            ways_to_move: [(-1,0),(-1,-1),(-1,1)].to_vec(),
        };
        game.pieces[6][(i as usize)-1] = pawn;
    }
}

fn main() {
    let conversion = letter_to_int('C');
    println!("{conversion}");
    let conversion = int_to_letter(2);
    println!("{conversion}");
}

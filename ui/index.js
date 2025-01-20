const chessboard = document.querySelector('.chessboard');
const reset_button = document.querySelector('.reset');
const white_button = document.querySelector('.choose_white');
const black_button = document.querySelector('.choose_black');
const rows = 8;
const cols = 8;

let click_count = 0;
let current_chosen = 'z1';
let start_pos = 'z1';
let end_pos = 'z1';
let board_state = {};
let chosen_color = "none";
let chosen_players = ["none","none"];
let player_to_move = "white";

// Define the initial positions of the pieces
const initialPositions = {
    'a1': '♜', 'b1': '♞', 'c1': '♝', 'd1': '♛', 'e1': '♚', 'f1': '♝', 'g1': '♞', 'h1': '♜', // White major pieces
    'a2': '♟', 'b2': '♟', 'c2': '♟', 'd2': '♟', 'e2': '♟', 'f2': '♟', 'g2': '♟', 'h2': '♟', // White pawns
    'a8': '♜', 'b8': '♞', 'c8': '♝', 'd8': '♛', 'e8': '♚', 'f8': '♝', 'g8': '♞', 'h8': '♜', // Black major pieces
    'a7': '♟', 'b7': '♟', 'c7': '♟', 'd7': '♟', 'e7': '♟', 'f7': '♟', 'g7': '♟', 'h7': '♟', // Black pawns
};
// function that takes in wpawn and returns the symbol for the piece 
function get_piece_symbol(piece){
    switch(piece){
        case "wpawn":
            piece_symbol = "♟";
            break;
        case "wrook":
            piece_symbol = "♜";
            break;
        case "wknight":
            piece_symbol = "♞";
            break;
        case "wbishop":
            piece_symbol = "♝";
            break;
        case "wqueen":
            piece_symbol = "♛";
            break;
        case "wking":
            piece_symbol = "♚";
            break;
        case "bpawn":
            piece_symbol = "♟";
            break;
        case "brook":
            piece_symbol = "♜";
            break;
        case "bknight":
            piece_symbol = "♞";
            break;
        case "bbishop":
            piece_symbol = "♝";
            break;
        case "bqueen":
            piece_symbol = "♛";
            break;
        case "bking":
            piece_symbol = "♚";
            break;
    }
    return piece_symbol;
}
//make function parsing through json looking for pices 'A1':'wpawn' and return a configuration of the board
function parse_board_state(data){
    let board_state = {};
    for (let key in data){
        if (key.toString().length == 2){
            board_state[key.toUpperCase()] = data[key];
        }
    }
    return board_state;
}

async function get_board_state() {
    const url = 'http://127.0.0.1:8080/boardstate';
    const response = await fetch(url);
    console.log("Fetching from:", url);
    const data = await response.json();
    console.log(data);
    return data;
}
async function reset_board(){
    const response = await fetch('http://127.0.0.1:8080/reset');
    return await response.json();
} 

async function move_piece(start_pos, end_pos){
    const response = await fetch('http://127.0.0.1:8080/movepiece', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({start_pos: start_pos.toUpperCase(), end_pos: end_pos.toUpperCase()})
    });
    const data = await response.json();
    //console.log(data);
    return data;
}

function draw_chessboard( board_state ){
    // Create the chessboard dynamically
    for (let row = rows; row >= 1; row--) {
        for (let col = 0; col < cols; col++) {
            const square = document.createElement('div');
            const isDark = (row + col) % 2 === 1;
            const position = `${String.fromCharCode(65 + col)}${row}`; 
            
            // Set the class for square color
            square.className = `square ${isDark ? 'dark' : 'light'}`;
            square.dataset.position = position;

            // Add the piece name if it's in the initialPositions map
            //console.log(position.toUpperCase());
            if (board_state[position]) {    
                const piece = document.createElement('span');
                piece.textContent = get_piece_symbol(board_state[position]); // Convert to uppercase
                if (board_state[position].at(0) == 'w'){
                    piece.className = 'piece white';
                }else{
                    piece.className = 'piece black';
                }
                square.appendChild(piece);
            }
            chessboard.appendChild(square);
        }
    }
}

// Add the click event listener to the square
chessboard.addEventListener('click', async (event) => {

    const selectedSquare = event.target;
    const selectedPiece = selectedSquare.querySelector('.piece');
    if (click_count == 0) {
        if (selectedPiece) {
            selectedSquare.classList.add('selected');
            console.log(`Selected piece: ${selectedPiece.textContent}, in ${selectedSquare.dataset.position}`);
            start_pos = selectedSquare.dataset.position;
            click_count++;
        } else {
            click_count = 0;
        }
    } else if (click_count == 1) {
        end_pos = selectedSquare.dataset.position;
        console.log(`Move from ${start_pos} to ${end_pos}`);
        const data = await move_piece(start_pos, end_pos);
        if (data.is_valid == "true") {
            console.log("Valid move");
            const board_state = parse_board_state(data);
            update_board_state(board_state);
            click_count++;
        }
        const prev_square = document.querySelector(`.square[data-position=${start_pos}]`);
        prev_square.classList.remove('selected');
        click_count = 0;
    }
    
});

async function update_board_state(board_state) {
    // Remove all existing pieces
    const pieces = document.querySelectorAll('.piece');
    pieces.forEach(piece => piece.remove());
    
    // Add updated pieces
    for (let position in board_state) {
        const square = document.querySelector(`.square[data-position=${position}]`);
        const newPiece = document.createElement('span');
        newPiece.textContent = get_piece_symbol(board_state[position]);
        newPiece.className = `piece ${board_state[position].charAt(0) === 'w' ? 'white' : 'black'}`;
        square.appendChild(newPiece);
    }
}


async function initializeBoard() {
    try {
        const board_state_data = await get_board_state();
        const board_state = parse_board_state(board_state_data);
        draw_chessboard(board_state);
    } catch (e) {
        console.log(e);
        draw_chessboard(initialPositions);
    }
}

reset_button.addEventListener('click', handle_reset);
white_button.addEventListener('click', handle_white);
black_button.addEventListener('click', handle_black);
async function handle_reset() {
    const data = await reset_board();
    const board_state = parse_board_state(data);
    update_board_state(board_state);
    player_to_move = "white";
}
async function handle_white() {
    const response = await fetch('http://127.0.0.1:8080/white');
    const data = await response.json();
    if (data.status == "taken"){
        console.log("White is taken");
    }else{
        console.log("White is available");
        chosen_color = "white";
    }
}
async function handle_black() {
    const response = await fetch('http://127.0.0.1:8080/black');
    const data = await response.json();
    if (data.status == "taken"){
        console.log("Black is taken");
    }else{
        console.log("Black is available");
        chosen_color = "black";
    }
}

console.log("Initializing board");
initializeBoard();


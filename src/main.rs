use server::server_start;

mod bitboard;
mod server;

fn main() {
    server_start(35326, 1);
}

/*
let w_bb = bitboard::get_initial_white_bitboard();
let b_bb = bitboard::get_initial_black_bitboard();

// Print inital board configuration
let legal_moves = bitboard::generate_legal_moves(b_bb, w_bb);
bitboard::render(w_bb, b_bb, legal_moves);

// select a move
let mov: u64 = 0x000002000000000;
bitboard::render(w_bb, b_bb, mov);

let new_board = bitboard::resolve_move(mov, b_bb, w_bb);
let new_legal_moves = bitboard::generate_legal_moves(new_board[1], new_board[0]);
bitboard::render(new_board[1], new_board[0], new_legal_moves);
*/

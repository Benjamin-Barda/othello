mod bitboard;
mod mcts;

use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let stream = TcpStream::connect("0.0.0.0:65432").unwrap();
    let mut mc = mcts::MCTS { stream };
    let white_bb = bitboard::get_initial_white_bitboard();
    let black_bb = bitboard::get_initial_black_bitboard();

    let out = mc.search([black_bb, white_bb], 1);
    println!("{:?}", out);
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

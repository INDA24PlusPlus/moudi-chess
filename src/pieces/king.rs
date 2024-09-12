use std::cmp::min;

use crate::{bitboard, BitBoard};
use super::{Board, Piece};

pub fn is_allowed_move(piece: &Piece, board: &Board, x: u8, y: u8) -> bool {
    get_all_moves(piece, board).get(y * 8 + x)
}

pub fn get_all_moves(piece: &Piece, board: &Board) -> BitBoard {
    let mut bitboard = bitboard::EMPTY;
    let opponent = board.get_opponent_board(piece);
    let [x, y] = piece.pos;

    // NORTH
    if !bitboard.is_empty_on_board_and_set(board, x, y + 1) {
        bitboard.predicate_and_set(x, y + 1, |x, y| opponent.get((y * 8) + x));
    }

    // NORTH-WEST
    if !bitboard.is_empty_on_board_and_set(board, x - 1, y + 1) {
        bitboard.predicate_and_set(x - 1, y + 1, |x, y| opponent.get((y * 8) + x));
    }

    // WEST
    if !bitboard.is_empty_on_board_and_set(board, x - 1, y) {
        bitboard.predicate_and_set(x - 1, y, |x, y| opponent.get((y * 8) + x));
    }

    // SOUTH-WEST
    if !bitboard.is_empty_on_board_and_set(board, x - 1, y - 1) {
        bitboard.predicate_and_set(x - 1, y - 1, |x, y| opponent.get((y * 8) + x));
    }

    // SOUTH
    if !bitboard.is_empty_on_board_and_set(board, x, y - 1) {
        bitboard.predicate_and_set(x, y - 1, |x, y| opponent.get((y * 8) + x));
    }

    // SOUTH-EAST
    if !bitboard.is_empty_on_board_and_set(board, x + 1, y - 1) {
        bitboard.predicate_and_set(x + 1, y - 1, |x, y| opponent.get((y * 8) + x));
    }

    // EAST
    if !bitboard.is_empty_on_board_and_set(board, x + 1, y) {
        bitboard.predicate_and_set(x + 1, y, |x, y| opponent.get((y * 8) + x));
    }

    // NORTH-EAST
    if !bitboard.is_empty_on_board_and_set(board, x + 1, y + 1) {
        bitboard.predicate_and_set(x + 1, y + 1, |x, y| opponent.get((y * 8) + x));
    }

    bitboard
}

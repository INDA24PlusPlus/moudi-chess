use crate::{bitboard, BitBoard};
use super::{Board, Piece};

pub fn is_allowed_move(piece: &Piece, board: &Board, index: usize) -> bool {
    get_all_moves(piece, board).get(index)
}

pub fn get_all_moves(piece: &Piece, board: &Board) -> BitBoard {
    let mut bitboard = bitboard::EMPTY;
    let opponent = board.get_opponent_board(piece);
    let (x, y) = piece.get_pos_as_usize();

    // NORTH
    if !bitboard.is_empty_on_board_and_set(board, x, y + 1) {
        bitboard.compare_and_set(opponent, true, x, y + 1);
    }

    // NORTH-WEST
    if x >= 1 && !bitboard.is_empty_on_board_and_set(board, x - 1, y + 1) {
        bitboard.compare_and_set(opponent, true, x - 1, y + 1);
    }

    // WEST
    if x >= 1 && !bitboard.is_empty_on_board_and_set(board, x - 1, y) {
        bitboard.compare_and_set(opponent, true, x - 1, y);
    }

    // SOUTH-WEST
    if x >= 1 && y >= 1 && !bitboard.is_empty_on_board_and_set(board, x - 1, y - 1) {
        bitboard.compare_and_set(opponent, true, x - 1, y - 1);
    }

    // SOUTH
    if y >= 1 && !bitboard.is_empty_on_board_and_set(board, x, y - 1) {
        bitboard.compare_and_set(opponent, true, x, y - 1);
    }

    // SOUTH-EAST
    if y >= 1 && !bitboard.is_empty_on_board_and_set(board, x + 1, y - 1) {
        bitboard.compare_and_set(opponent, true, x + 1, y - 1);
    }

    // EAST
    if !bitboard.is_empty_on_board_and_set(board, x + 1, y) {
        bitboard.compare_and_set(opponent, true, x + 1, y);
    }

    // NORTH-EAST
    if !bitboard.is_empty_on_board_and_set(board, x + 1, y + 1) {
        bitboard.compare_and_set(opponent, true, x + 1, y + 1);
    }

    bitboard
}

use std::cmp::min;

use crate::{bitboard, BitBoard};
use crate::CoordinateIterator::*;
use super::{Board, Piece};

pub fn is_allowed_move(piece: &Piece, board: &Board, index: usize) -> bool {
    get_all_moves(piece, board).get(index)
}

pub fn get_all_moves(piece: &Piece, board: &Board) -> BitBoard {
    let mut bitboard = bitboard::EMPTY;
    let opponent = board.get_opponent_board(piece);
    let pos = piece.get_pos_as_usize();

    // NORTH-WEST
    for (x, y) in CoordinateIterator::new(pos, (0, 8)) {
        if !bitboard.is_empty_on_board_and_set(board, x, y) {
            bitboard.compare_and_set(opponent, true, x, y);
            break;
        }
    }

    // NORTH-EAST
    for (x, y) in CoordinateIterator::new(pos, (8, 8)) {
        if !bitboard.is_empty_on_board_and_set(board, x, y) {
            bitboard.compare_and_set(opponent, true, x, y);
            break;
        }
    }

    // SOUTH-WEST
    for (x, y) in CoordinateIterator::new(pos, (0, 0)) {
        if !bitboard.is_empty_on_board_and_set(board, x, y) {
            bitboard.compare_and_set(opponent, true, x, y);
            break;
        }
    }

    // SOUTH-EAST
    for (x, y) in CoordinateIterator::new(pos, (8, 0)) {
        if !bitboard.is_empty_on_board_and_set(board, x, y) {
            bitboard.compare_and_set(opponent, true, x, y);
            break;
        }
    }

    bitboard
}

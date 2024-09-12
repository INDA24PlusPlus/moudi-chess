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
    for offset in 1..(7 - y) {
        if !bitboard.is_empty_on_board_and_set(board, x, y + offset) {
            bitboard.predicate_and_set(x, y + offset, |x, y| opponent.get((y * 8) + x));
            break;
        }
    }

    // WEST
    for offset in 1..(x) {
        if !bitboard.is_empty_on_board_and_set(board, x - offset, y) {
            bitboard.predicate_and_set(x - offset, y, |x, y| opponent.get((y * 8) + x));
            break;
        }
    }

    // SOUTH
    for offset in 1..(y) {
        if !bitboard.is_empty_on_board_and_set(board, x, y - offset) {
            bitboard.predicate_and_set(x, y - offset, |x, y| opponent.get((y * 8) + x));
            break;
        }
    }

    // EAST
    for offset in 1..(7 - x) {
        if !bitboard.is_empty_on_board_and_set(board, x + offset, y) {
            bitboard.predicate_and_set(x + offset, y, |x, y| opponent.get((y * 8) + x));
            break;
        }
    }

    bitboard
}

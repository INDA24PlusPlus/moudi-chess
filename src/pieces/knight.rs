use crate::{bitboard, BitBoard};
use super::{Board, Piece};

pub fn is_allowed_move(piece: &Piece, board: &Board, index: usize) -> bool {
    get_all_moves(piece, board).get(index)
}

pub fn get_all_moves(piece: &Piece, board: &Board) -> BitBoard {
    let mut bitboard = bitboard::EMPTY;
    let side = board.get_sides_board(piece.get_color());
    let (x, y) = piece.get_pos_as_usize();

    // NORTH-LEFT
    if x >= 1 {
        bitboard.predicate_and_set(x - 1, y + 2, |x, y| Board::is_inbounds(x, y) && !side.get(y * 8 + x));
    }
    // NORTH-RIGHT
    bitboard.predicate_and_set(x + 1, y + 2, |x, y| Board::is_inbounds(x, y) && !side.get(y * 8 + x));

    // EAST-UP
    bitboard.predicate_and_set(x + 2, y + 1, |x, y| Board::is_inbounds(x, y) && !side.get(y * 8 + x));
    // EAST-DOWN
    if y >= 1 {
        bitboard.predicate_and_set(x + 2, y - 1, |x, y| Board::is_inbounds(x, y) && !side.get(y * 8 + x));
    }

    // SOUTH-RIGHT
    if y >= 2 {
        bitboard.predicate_and_set(x + 1, y - 2, |x, y| Board::is_inbounds(x, y) && !side.get(y * 8 + x));
    }
    // SOUTH-LEFT
    if x >= 1 && y >= 2 {
        bitboard.predicate_and_set(x - 1, y - 2, |x, y| Board::is_inbounds(x, y) && !side.get(y * 8 + x));
    }
    
    // WEST-DOWN
    if x >= 2 && y >= 1 {
        bitboard.predicate_and_set(x - 2, y - 1, |x, y| Board::is_inbounds(x, y) && !side.get(y * 8 + x));
    }
    // WEST-UP
    if x >= 2 {
        bitboard.predicate_and_set(x - 2, y + 1, |x, y| Board::is_inbounds(x, y) && !side.get(y * 8 + x));
    }

    bitboard
}

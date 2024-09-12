use crate::{bitboard, BitBoard};
use super::{Board, Piece};

pub fn is_allowed_move(piece: &Piece, x: u8, y: u8) -> bool {
    get_all_moves(piece).get(y * 8 + x)
}

pub fn get_all_moves(piece: &Piece) -> BitBoard {
    let mut bitboard = bitboard::EMPTY;
    let [x, y] = piece.pos;

    // NORTH-LEFT
    bitboard.predicate_and_set(x - 1, y + 2, Board::is_inbounds);
    // NORTH-RIGHT
    bitboard.predicate_and_set(x + 1, y + 2, Board::is_inbounds);

    // EAST-UP
    bitboard.predicate_and_set(x + 2, y + 1, Board::is_inbounds);
    // EAST-DOWN
    bitboard.predicate_and_set(x + 2, y - 1, Board::is_inbounds);

    // SOUTH-RIGHT
    bitboard.predicate_and_set(x + 1, y - 2, Board::is_inbounds);
    // SOUTH-LEFT
    bitboard.predicate_and_set(x - 1, y - 2, Board::is_inbounds);
    
    // WEST-DOWN
    bitboard.predicate_and_set(x - 2, y - 1, Board::is_inbounds);
    // WEST-UP
    bitboard.predicate_and_set(x - 2, y + 1, Board::is_inbounds);

    bitboard
}

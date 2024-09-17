use crate::{bitboard, BitBoard};
use super::{Board, Piece};

pub fn is_allowed_move(piece: &Piece, board: &Board, index: i8) -> bool {
    get_all_moves(piece, board).get(index)
}

pub fn get_all_moves(piece: &Piece, board: &Board) -> BitBoard {
    let mut bitboard = bitboard::EMPTY;
    let side = board.get_piece_sides_board(piece);
    let [x, y] = piece.pos;

    // NORTH-LEFT
    bitboard.predicate_and_set(x - 1, y + 2, |x, y| Board::is_inbounds(x, y) && !side.get(y * 8 + x));
    // NORTH-RIGHT
    bitboard.predicate_and_set(x + 1, y + 2, |x, y| Board::is_inbounds(x, y) && !side.get(y * 8 + x));

    // EAST-UP
    bitboard.predicate_and_set(x + 2, y + 1, |x, y| Board::is_inbounds(x, y) && !side.get(y * 8 + x));
    // EAST-DOWN
    bitboard.predicate_and_set(x + 2, y - 1, |x, y| Board::is_inbounds(x, y) && !side.get(y * 8 + x));

    // SOUTH-RIGHT
    bitboard.predicate_and_set(x + 1, y - 2, |x, y| Board::is_inbounds(x, y) && !side.get(y * 8 + x));
    // SOUTH-LEFT
    bitboard.predicate_and_set(x - 1, y - 2, |x, y| Board::is_inbounds(x, y) && !side.get(y * 8 + x));
    
    // WEST-DOWN
    bitboard.predicate_and_set(x - 2, y - 1, |x, y| Board::is_inbounds(x, y) && !side.get(y * 8 + x));
    // WEST-UP
    bitboard.predicate_and_set(x - 2, y + 1, |x, y| Board::is_inbounds(x, y) && !side.get(y * 8 + x));

    bitboard
}

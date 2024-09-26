use crate::{BitBoard, CoordinateIterator};
use super::{Board, Piece};

pub(crate) fn is_allowed_move(piece: &Piece, board: &Board, index: usize) -> bool {
    get_allowed_moves(piece, board).get(index)
}

pub(crate) fn get_allowed_moves(piece: &Piece, board: &Board) -> BitBoard {
    let opponent = board.get_opponent_board(piece.get_color());
    board.filter_king_safety(get_all_moves(piece, board).filter_on(|index| board.is_empty(index) || opponent.get(index)), piece)
}

pub(crate) fn get_attacked_squares(piece: &Piece, board: &Board) -> BitBoard {
    get_all_moves(piece, board)
}

fn get_all_moves(piece: &Piece, board: &Board) -> BitBoard {
    let pos = piece.get_pos_as_usize();

    let action = |bitboard: &mut BitBoard, x, y| {
        if !board.is_empty(y * 8 + x) {
            bitboard.set(y * 8 + x, true);
            return true;
        }
        bitboard.set(y * 8 + x, true);
        false
    };

    board.check_and_set_piece_iter(CoordinateIterator::from_delta(pos, (0, 1)), action) // NORTH
        | board.check_and_set_piece_iter(CoordinateIterator::from_delta(pos, (1, 0)), action) // EAST
        | board.check_and_set_piece_iter(CoordinateIterator::from_delta(pos, (0, -1)), action) // SOUTH
        | board.check_and_set_piece_iter(CoordinateIterator::from_delta(pos, (-1, 0)), action) // WEST
}

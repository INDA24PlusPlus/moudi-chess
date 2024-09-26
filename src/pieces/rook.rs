use crate::{BitBoard, CoordinateIterator};
use super::{Board, Piece};

pub(crate) fn is_allowed_move(piece: &Piece, board: &Board, index: usize) -> bool {
    get_all_moves(piece, board).get(index)
}

pub(crate) fn get_all_moves(piece: &Piece, board: &Board) -> BitBoard {
    let opponent_and_empty = board.get_opponent_and_empty_squares_board(piece.get_color());
    let pos = piece.get_pos_as_usize();
    
    let action = |bitboard: &mut BitBoard, x, y, set| {
        if opponent_and_empty.get(y * 8 + x) {
            bitboard.set(y * 8 + x, set);
            return false;
        }
        false
    };

    board.check_and_set_piece_iter(piece, CoordinateIterator::from_delta(pos, (0, 1)), action) // NORTH
        | board.check_and_set_piece_iter(piece, CoordinateIterator::from_delta(pos, (1, 0)), action) // EAST
        | board.check_and_set_piece_iter(piece, CoordinateIterator::from_delta(pos, (0, -1)), action) // SOUTH
        | board.check_and_set_piece_iter(piece, CoordinateIterator::from_delta(pos, (-1, 0)), action) // WEST
}

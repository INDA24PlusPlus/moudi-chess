use crate::{BitBoard, CoordinateIterator};
use super::{Board, Piece};

pub(crate) fn is_allowed_move(piece: &Piece, board: &Board, index: usize) -> bool {
    get_all_moves(piece, board).get(index)
}

pub(crate) fn get_all_moves(piece: &Piece, board: &Board) -> BitBoard {
    let opponent = board.get_opponent_board(piece);
    let pos = piece.get_pos_as_usize();
    
    let action = |bitboard: &mut BitBoard, x, y| {
        if !bitboard.is_empty_on_board_and_set(board, x, y) {
            bitboard.compare_and_set(opponent, true, x, y);
            return true;
        }
        false
    };

    board.check_and_set_piece_iter(piece, CoordinateIterator::from_delta(pos, (0, 1)), action) // NORTH
        | board.check_and_set_piece_iter(piece, CoordinateIterator::from_delta(pos, (1, 0)), action) // EAST
        | board.check_and_set_piece_iter(piece, CoordinateIterator::from_delta(pos, (0, -1)), action) // SOUTH
        | board.check_and_set_piece_iter(piece, CoordinateIterator::from_delta(pos, (-1, 0)), action) // WEST
}

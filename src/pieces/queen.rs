use crate::{bitboard, BitBoard, CoordinateIterator};
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

    board.check_and_set_piece_iter(piece, CoordinateIterator::new(pos, (pos.0, 7)), action) // NORTH
        | board.check_and_set_piece_iter(piece, CoordinateIterator::new(pos, (7, pos.1)), action) // EAST
        | board.check_and_set_piece_iter(piece, CoordinateIterator::new(pos, (pos.0, 0)), action) // SOUTH
        | board.check_and_set_piece_iter(piece, CoordinateIterator::new(pos, (0, pos.1)), action) // WEST
        | board.check_and_set_piece_iter(piece, CoordinateIterator::new(pos, (0, 7)), action) // NORTH-WEST
        | board.check_and_set_piece_iter(piece, CoordinateIterator::new(pos, (7, 7)), action) // NORTH-EAST
        | board.check_and_set_piece_iter(piece, CoordinateIterator::new(pos, (7, 0)), action) // SOUTH-EAST
        | board.check_and_set_piece_iter(piece, CoordinateIterator::new(pos, (0, 0)), action) // SOUTH-WEST
}

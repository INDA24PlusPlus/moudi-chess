use crate::{bitboard, BitBoard, CoordinateIterator};
use super::{Board, Piece, Side};

pub(crate) fn is_allowed_move(piece: &Piece, board: &Board, index: usize) -> bool {
    get_allowed_moves(piece, board).get(index)
}

pub(crate) fn get_allowed_moves(piece: &Piece, board: &Board) -> BitBoard {
    let opponent = board.get_opponent_board(piece.get_color());

    let attacked_bitboard = if let Some(ep_index) = board.get_ep_target() {
        get_attacked_squares(piece, board).filter_on(|index| index.abs_diff(ep_index as usize) == 8 || opponent.get(index))
    } else {
        get_attacked_squares(piece, board).filter_on(|index| opponent.get(index))
    };

    board.filter_king_safety(get_move_bitboard(piece, board) | attacked_bitboard, piece)
}

pub(crate) fn get_attacked_squares(piece: &Piece, board: &Board) -> BitBoard {
    let (x, y) = piece.get_pos_as_usize();
    let attack_level = match piece.color {
        Side::White => y as isize + 1,
        Side::Black => y as isize - 1
    } as usize;

    // bounds check: not within 0..7
    if !Board::is_inbounds(x, attack_level) {
        return bitboard::EMPTY;
    }

    let mut list = vec![];
    if x > 0 {
        list.push((x - 1, attack_level));
    }
    if x < 7 {
        list.push((x + 1, attack_level));
    }
    let it = list.iter().map(|(x, y)| (*x, *y));

    board.check_and_set_piece_iter(it, |bitboard, x, y| {
        bitboard.set(y * 8 + x, true);
        false
    })
}

fn get_move_bitboard(piece: &Piece, board: &Board) -> BitBoard {
    let (x, y) = piece.get_pos_as_usize();
    let end = match piece.color {
        Side::White => (x, if y == 1 {3} else {y + 1}),
        Side::Black => (x, if y == 6 {4} else {y - 1}),
    };

    board.check_and_set_piece_iter(CoordinateIterator::from_to(piece.get_pos_as_usize(), end), 
        |bitboard: &mut BitBoard, x, y| !bitboard.is_empty_on_board_and_set(board, x, y))
}

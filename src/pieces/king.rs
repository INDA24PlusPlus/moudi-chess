use crate::{bitboard, BitBoard, CoordinateIterator,};
use super::{Board, CastlingAbility, Piece};

pub(crate) fn is_allowed_move(piece: &Piece, board: &Board, index: usize) -> bool {
    get_all_moves(piece, board).get(index)
}

pub(crate) fn get_all_moves(piece: &Piece, board: &Board) -> BitBoard {
    let opponent_and_empty = board.get_opponent_and_empty_squares_board(piece.get_color());
    let (x, y) = piece.get_pos_as_usize();
    let mut list = vec![];

    // NORTH
    list.push((x, y + 1));

    if x >= 1 {
        // NORTH-WEST
        list.push((x - 1, y + 1));
        // WEST
        list.push((x - 1, y));
    }


    // SOUTH-WEST
    if x >= 1 && y >= 1 {
        list.push((x - 1, y - 1));
    }

    if y >= 1 {
        // SOUTH
        list.push((x, y - 1));
        // SOUTH-EAST
        list.push((x + 1, y - 1));
    }

    // EAST
    list.push((x + 1, y));

    // NORTH-EAST
    list.push((x + 1, y + 1));

    let castling = board.get_castling(piece.get_color());
    let attacked = board.get_side_computed_boards(piece.get_color()).2;
    let combined_board = board.get_combined_piece_board();
    
    let is_not_empty_or_attacked = |(x, y)| attacked.get(y * 8 + x) || combined_board.get(y * 8 + x);

    let mut castling_bitboard = bitboard::EMPTY;
    // has king side castlingability that those slots are open and not attacked
    if castling.has(CastlingAbility::King) && !CoordinateIterator::from_inclusive_to(piece.get_pos_as_usize(), (6, y)).any(is_not_empty_or_attacked) {
        castling_bitboard.set(y * 8 + 6, true);
    }

    // has queen side castlingability that those slots are open and not attacked
    if castling.has(CastlingAbility::Queen) && !CoordinateIterator::from_inclusive_to(piece.get_pos_as_usize(), (2, y)).any(is_not_empty_or_attacked) {
        castling_bitboard.set(y * 8 + 2, true);
    }

    board.check_and_set_piece_iter(piece, list.iter().map(|(x, y)| (*x, *y)), |bitboard, x, y, set| {
        if opponent_and_empty.get(y * 8 + x) {
            bitboard.set(y * 8 + x, set);
        }
        false
    }) | castling_bitboard
}

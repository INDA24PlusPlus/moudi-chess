use crate::{bitboard, BitBoard};
use super::{Board, Piece};

pub fn is_allowed_move(piece: &Piece, board: &Board, index: usize) -> bool {
    get_all_moves(piece, board).get(index)
}

pub fn get_all_moves(piece: &Piece, board: &Board) -> BitBoard {
    let opponent = board.get_opponent_board(piece);
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

    board.check_and_set_piece_iter(piece, list.iter().map(|(x, y)| (*x, *y)), |bitboard, x, y| {
        if !bitboard.is_empty_on_board_and_set(board, x, y) {
            bitboard.compare_and_set(opponent, true, x, y);
        }
        false
    })
}

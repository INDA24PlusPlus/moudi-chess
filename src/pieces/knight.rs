use crate::{bitboard, BitBoard};
use super::{Board, Piece};

pub(crate) fn is_allowed_move(piece: &Piece, board: &Board, index: usize) -> bool {
    get_all_moves(piece, board).get(index)
}

pub(crate) fn get_all_moves(piece: &Piece, board: &Board) -> BitBoard {
    let side = board.get_sides_board(piece.get_color());
    let (x, y) = piece.get_pos_as_usize();
    let mut list = vec![];

    // NORTH-LEFT
    if x >= 1 {
        list.push((x - 1, y + 2));
    }
    // NORTH-RIGHT
    list.push((x + 1, y + 2));

    // EAST-UP
    list.push((x + 2, y + 1));
    
    // EAST-DOWN
    if y >= 1 {
        list.push((x + 2, y - 1));
    }

    // SOUTH-RIGHT
    if y >= 2 {
        list.push((x + 1, y - 2));
    }
    
    // SOUTH-LEFT
    if x >= 1 && y >= 2 {
        list.push((x - 1, y - 2));
    }
    
    // WEST-DOWN
    if x >= 2 && y >= 1 {
        list.push((x - 2, y - 1));
    }
    // WEST-UP
    if x >= 2 {
        list.push((x - 2, y + 1));
    }

    board.check_and_set_piece_iter(piece, list.iter().map(|(x, y)| (*x, *y)), |bitboard, x, y| {
        let _ = bitboard.compare_and_set(side, false, x, y);
        false
    })
}

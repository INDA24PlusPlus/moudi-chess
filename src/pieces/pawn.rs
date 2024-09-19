use crate::{bitboard, BitBoard};
use super::{Board, Piece, Side};

pub fn is_allowed_move(piece: &Piece, board: &Board, index: usize) -> bool {
    get_all_moves(piece, board).get(index)
}

pub fn get_all_moves(piece: &Piece, board: &Board) -> BitBoard {
    get_move_bitboard(piece, board) | get_attack_bitboard(piece, board)
}

fn get_move_bitboard(piece: &Piece, board: &Board) -> BitBoard {
    let mut bitboard = bitboard::EMPTY;
    let (x, y) = piece.get_pos_as_usize();

    match piece.color {
        Side::White => {
            if Board::is_inbounds(x, y + 1) && bitboard.is_empty_on_board_and_set(board, x, y + 1) && y == 1 {
                bitboard.is_empty_on_board_and_set(board, x, 3);
            }
        },
        Side::Black => {
            if Board::is_inbounds(x, y - 1) && bitboard.is_empty_on_board_and_set(board, x, y - 1) && y == 6 {
                bitboard.is_empty_on_board_and_set(board, x, 4);
            }
        }
    }

    bitboard
}

fn get_attack_bitboard(piece: &Piece, board: &Board) -> BitBoard {
    let mut bitboard = bitboard::EMPTY;
    let opponent = board.get_opponent_board(piece);
    let (x, y) = piece.get_pos_as_usize();

    let attack_level = match piece.color {
        Side::White => y + 1,
        Side::Black => y - 1
    };

    // bounds check: not within 0..7
    if !Board::is_inbounds(x, attack_level) {
        return bitboard;
    }

    if let Some(ep_index) = board.get_ep_target() {
        bitboard.predicate_and_set(x - 1, attack_level, |x, y| y * 8 + x == ep_index as usize);
        bitboard.predicate_and_set(x + 1, attack_level, |x, y| y * 8 + x == ep_index as usize);
    }
    
    bitboard.compare_and_set(opponent, true, x - 1, attack_level);
    bitboard.compare_and_set(opponent, true, x + 1, attack_level);

    bitboard
}

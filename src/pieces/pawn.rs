use crate::{bitboard, BitBoard};
use super::{Board, Piece, Side};

pub fn is_allowed_move(piece: &Piece, board: &Board, x: u8, y: u8) -> bool {
    get_all_moves(piece, board).get(y * 8 + x)
}

pub fn get_all_moves(piece: &Piece, board: &Board) -> BitBoard {
    get_move_bitboard(piece, board) & get_attack_bitboard(piece, board)
}

fn get_move_bitboard(piece: &Piece, board: &Board) -> BitBoard {
    let mut bitboard = bitboard::EMPTY;
    let [x, y] = piece.pos;

    if bitboard.is_empty_on_board_and_set(board, x, y + 1) {
        if piece.color == Side::White && piece.pos[1] == 1 {
            bitboard.is_empty_on_board_and_set(board, x, 3);
        } else if piece.color == Side::Black && piece.pos[1] == 6 {
            bitboard.is_empty_on_board_and_set(board, x, 6);
        }
    }

    bitboard
}

fn get_attack_bitboard(piece: &Piece, board: &Board) -> BitBoard {
    let mut bitboard = bitboard::EMPTY;
    let opponent = board.get_opponent_board(piece);
    let [x, y] = piece.pos;

    let attack_level = match piece.color {
        Side::White => y + 1,
        Side::Black => y - 1
    };

    // bounds check: not within 0..7
    if attack_level >= 8 {
        return bitboard;
    }
    
    bitboard.compare_and_set(opponent, true, x - 1, attack_level);
    bitboard.compare_and_set(opponent, true, x + 1, attack_level);

    bitboard
}

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
    let opponent = board.get_opponent_board(piece);
    let (x, y) = piece.pos;

    let attack_level = match piece.color {
        Side::White => y + 1,
        Side::Black => y - 1
    } as usize;

    // bounds check: not within 0..7
    if !Board::is_inbounds(x as usize, attack_level) {
        return bitboard::EMPTY;
    }

    if let Some(ep_index) = board.get_ep_target() {
        // check for en passant and normal attack
        board.check_and_set_piece_iter(piece, 
            vec![((x - 1) as usize, attack_level), ((x + 1) as usize, attack_level)].iter().map(|(x, y)| (*x, *y)),  |bitboard, x, y| {
               let _ = bitboard.predicate_and_set(x, y, |x, y| (y * 8 + x == ep_index as usize)) || bitboard.compare_and_set(opponent, true, x, y);
                false
            })
    } else {
        // check for normal attack
        board.check_and_set_piece_iter(piece, 
            vec![((x - 1) as usize, attack_level), ((x + 1) as usize, attack_level)].iter().map(|(x, y)| (*x, *y)), |bitboard, x, y| {
                let _ = bitboard.compare_and_set(opponent, true, x, y);
                false
            })
    }
}

mod chess;
pub use crate::chess::*;

mod board;
pub use crate::board::*;

mod pieces;
pub use crate::pieces::{PieceType, Piece};

mod bitboard;
use crate::bitboard::BitBoard;

mod coordinateiterator;
use crate::coordinateiterator::*;

mod file;

mod cli;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stalemate() {
        let chess = Chess::from_fen("8/8/2p1k3/8/8/1q6/8/K7 w - - 0 1".to_string());
        assert!(chess.get_state() == State::Stalemate);
    }

    #[test]
    fn check() {
        let chess = Chess::from_fen("8/8/2p1k3/8/8/8/1q6/K7 w - - 0 1".to_string());
        assert!(chess.get_state() == State::Check);
    }

    #[test]
    fn checkmate() {
        let chess = Chess::from_fen("8/8/2p5/8/8/k7/1q6/K7 w - - 0 1".to_string());
        assert!(chess.get_state() == State::Checkmate);
    }

    #[test]
    fn draw() {
        let chess = Chess::from_fen("8/8/5k2/8/3K4/8/8/8 w - - 0 1".to_string());
        assert!(chess.get_state() == State::Draw);
    }

    #[test]
    fn promotion() {
        let mut chess = Chess::from_fen("8/6P1/2p1k3/8/8/1q6/8/4K3 w - - 0 1".to_string());
        let g7 = notation_to_index("G7");
        let g8 = notation_to_index("G8");

        assert!(chess.board.get_playing_side() == Side::White);
        assert!(chess.make_move(g7, g8));
        println!("State: {:?}", chess.get_state());
        assert!(chess.get_state() == State::Promotion);
        assert!(chess.board.get_playing_side() == Side::Black);
        
        chess.promote(PieceType::Queen); 
        assert!(chess.board.get_playing_side() == Side::Black);

        if let Some(queen) = chess.board.get_piece_at_pos(g8) {
            assert!(queen.get_piece_type() == PieceType::Queen);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn move_piece_to_block_check() {
        let mut chess = Chess::from_fen("8/1k6/1bp5/8/8/1q6/3P4/6K1 w - - 0 1".to_string());

        assert!(chess.make_move(notation_to_index("D2"), notation_to_index("D4")));
    }

    #[test]
    fn move_piece_to_not_block_check() {
        let mut chess = Chess::from_fen("8/1k6/1bp5/8/8/1q6/3P4/6K1 w - - 0 1".to_string());

        assert!(!chess.make_move(notation_to_index("D2"), notation_to_index("D3")));
    }

    #[test]
    fn en_passant() {
        let mut chess = Chess::new();

        assert!(chess.get_playing_side() == Side::White);
        assert!(chess.make_move(notation_to_index("E2"), notation_to_index("E4")));

        assert!(chess.get_playing_side() == Side::Black);
        assert!(chess.make_move(notation_to_index("D7"), notation_to_index("D5")));

        assert!(chess.get_playing_side() == Side::White);
        assert!(chess.make_move(notation_to_index("E4"), notation_to_index("D5")));

        assert!(chess.get_playing_side() == Side::Black);
        assert!(chess.make_move(notation_to_index("C7"), notation_to_index("C5")));

        assert!(chess.get_playing_side() == Side::White);
        assert!(chess.make_move(notation_to_index("D5"), notation_to_index("C6")));

        assert!(chess.get_playing_side() == Side::Black);
    }

    #[test]
    fn take_attacking_piece_not_with_king_while_in_check() {
        let mut chess = Chess::new();

        assert!(chess.get_playing_side() == Side::White);
        assert!(chess.make_move(notation_to_index("B1"), notation_to_index("C3")));

        assert!(chess.get_playing_side() == Side::Black);
        assert!(chess.make_move(notation_to_index("D7"), notation_to_index("D6")));

        assert!(chess.get_playing_side() == Side::White);
        assert!(chess.make_move(notation_to_index("C3"), notation_to_index("B5")));

        assert!(chess.get_playing_side() == Side::Black);
        assert!(chess.make_move(notation_to_index("D6"), notation_to_index("D5")));

        assert!(chess.get_playing_side() == Side::White);
        assert!(chess.make_move(notation_to_index("B5"), notation_to_index("C7")));

        assert!(chess.get_state() == State::Check);
        assert!(chess.get_playing_side() == Side::Black);
        assert!(chess.make_move(notation_to_index("D8"), notation_to_index("C7")));
    }

    #[test]
    fn take_attacking_piece_with_king_while_in_check() {
        let mut chess = Chess::new();

        assert!(chess.get_playing_side() == Side::White);
        assert!(chess.make_move(notation_to_index("E2"), notation_to_index("E4")));

        assert!(chess.get_playing_side() == Side::Black);
        assert!(chess.make_move(notation_to_index("E7"), notation_to_index("E6")));

        assert!(chess.get_playing_side() == Side::White);
        assert!(chess.make_move(notation_to_index("D1"), notation_to_index("H5")));

        assert!(chess.get_playing_side() == Side::Black);
        assert!(chess.make_move(notation_to_index("E6"), notation_to_index("E5")));

        assert!(chess.get_playing_side() == Side::White);
        assert!(chess.make_move(notation_to_index("H5"), notation_to_index("F7")));

        assert!(chess.get_state() == State::Check);
        assert!(chess.get_playing_side() == Side::Black);
        assert!(chess.make_move(notation_to_index("E8"), notation_to_index("F7")));
    }

    #[test]
    fn king_not_able_to_attack_king() {
        let mut chess = Chess::from_fen("8/8/5k2/7p/P2K4/8/8/8 w - - 0 1".to_string());

        assert!(chess.get_playing_side() == Side::White);
        assert!(!chess.make_move(notation_to_index("D4"), notation_to_index("E5")));

        assert!(chess.get_playing_side() == Side::White);
        assert!(chess.make_move(notation_to_index("D4"), notation_to_index("D5")));

        assert!(chess.get_playing_side() == Side::Black);
        assert!(!chess.make_move(notation_to_index("F6"), notation_to_index("E5")));
    }

    #[test]
    fn special_checkmate() {
        let chess = Chess::from_fen("8/8/3Q4/4k3/5Q2/8/8/4K3 b - - 0 1".to_string());
    
        assert!(chess.get_state() == State::Checkmate);
    }

    #[test]
    fn castle_while_in_check() {
        let mut chess = Chess::from_fen("4k3/8/8/q7/7q/8/8/R3K2R w KQ - 0 1".to_string());

        assert!(chess.get_state() == State::Check);
        assert!(chess.get_playing_side() == Side::White);
        assert!(!chess.make_move(notation_to_index("E1"), notation_to_index("G1")));

        assert!(chess.get_playing_side() == Side::White);
        assert!(!chess.make_move(notation_to_index("E1"), notation_to_index("C1")));

        assert!(chess.get_playing_side() == Side::White);
    }

    #[test]
    fn not_jump_over_piece_to_capture_king_attacker_with_rook() {
        let mut chess = Chess::new();

        assert!(chess.get_playing_side() == Side::White);
        assert!(chess.make_move(notation_to_index("E2"), notation_to_index("E4")));

        assert!(chess.get_playing_side() == Side::Black);
        assert!(chess.make_move(notation_to_index("F7"), notation_to_index("F6")));

        assert!(chess.get_playing_side() == Side::White);
        assert!(chess.make_move(notation_to_index("D1"), notation_to_index("H5")));

        assert!(chess.get_playing_side() == Side::Black);
        assert!(!chess.make_move(notation_to_index("H8"), notation_to_index("H5")));
    }

    #[test]
    fn not_jump_over_piece_to_capture_king_attacker_with_bishop() {
        let mut chess = Chess::new();

        assert!(chess.get_playing_side() == Side::White);
        assert!(chess.make_move(notation_to_index("E2"), notation_to_index("E4")));

        assert!(chess.get_playing_side() == Side::Black);
        assert!(chess.make_move(notation_to_index("D7"), notation_to_index("D5")));

        assert!(chess.get_playing_side() == Side::White);
        assert!(chess.make_move(notation_to_index("E4"), notation_to_index("D5")));

        assert!(chess.get_playing_side() == Side::Black);
        assert!(chess.make_move(notation_to_index("D8"), notation_to_index("D5")));

        assert!(chess.get_playing_side() == Side::White);
        assert!(chess.make_move(notation_to_index("G1"), notation_to_index("F3")));

        assert!(chess.get_playing_side() == Side::Black);
        assert!(chess.make_move(notation_to_index("D5"), notation_to_index("E4")));

        assert!(chess.get_state() == State::Check);
        assert!(chess.get_playing_side() == Side::White);
        assert!(!chess.make_move(notation_to_index("C1"), notation_to_index("E3")));

        assert!(chess.get_playing_side() == Side::White);
        assert!(chess.make_move(notation_to_index("F1"), notation_to_index("E2")));
    }

    #[test]
    fn take_piece_protected_by_pawn_with_king() {
        let mut chess = Chess::from_fen("8/3p4/4q3/4K3/8/8/8/8 w - - 0 1".to_string());

        assert!(chess.get_state() == State::Check);
        assert!(chess.get_playing_side() == Side::White);
        assert!(!chess.make_move(notation_to_index("E5"), notation_to_index("E6")));
    }

    #[test]
    fn take_piece_protected_by_bishop_with_king() {
        let mut chess = Chess::from_fen("2k5/8/4q3/4K3/2b5/8/8/8 w - - 0 1".to_string());

        assert!(chess.get_state() == State::Check);
        assert!(chess.get_playing_side() == Side::White);
        assert!(!chess.make_move(notation_to_index("E5"), notation_to_index("E6")));
    }

    #[test]
    fn take_piece_protected_by_knight_with_king() {
        let mut chess = Chess::from_fen("2k5/8/4q3/2n1K3/8/8/8/8 w - - 0 1".to_string());

        assert!(chess.get_state() == State::Check);
        assert!(chess.get_playing_side() == Side::White);
        assert!(!chess.make_move(notation_to_index("E5"), notation_to_index("E6")));
    }

    #[test]
    fn take_piece_protected_by_rook_with_king() {
        let mut chess = Chess::from_fen("2k5/8/1r2q3/4K3/8/8/8/8 w - - 0 1".to_string());

        assert!(chess.get_state() == State::Check);
        assert!(chess.get_playing_side() == Side::White);
        assert!(!chess.make_move(notation_to_index("E5"), notation_to_index("E6")));
    }

    #[test]
    fn take_piece_protected_by_queen_with_king() {
        let mut chess = Chess::from_fen("2k5/3q4/4q3/4K3/8/8/8/8 w - - 0 1".to_string());

        assert!(chess.get_state() == State::Check);
        assert!(chess.get_playing_side() == Side::White);
        assert!(!chess.make_move(notation_to_index("E5"), notation_to_index("E6")));
    }

    #[test]
    fn take_piece_protected_by_king_with_king() {
        let mut chess = Chess::from_fen("8/3k4/4q3/4K3/8/8/8/8 w - - 0 1".to_string());

        assert!(chess.get_state() == State::Check);
        assert!(chess.get_playing_side() == Side::White);
        assert!(!chess.make_move(notation_to_index("E5"), notation_to_index("E6")));
    }

    #[test]
    fn cli() {
        cli::start();
    }
}

fn notation_to_index(move_notation: &'static str) -> usize {
    let mut chars = move_notation.chars();
    let file = (chars.next().unwrap() as u8) - b'A';
    let rank = (chars.next().unwrap() as u8) - b'1';
    (rank * 8 + file) as usize
}

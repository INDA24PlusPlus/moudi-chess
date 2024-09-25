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
use crate::file::*;

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
        assert!(chess.make_move(notation_to_index("D5"), notation_to_index("C5")));

        assert!(chess.get_playing_side() == Side::Black);
    }

    // #[test]
    // fn cli() {
    //     cli::start();
    // }
}

fn notation_to_index(move_notation: &'static str) -> usize {
    let mut chars = move_notation.chars();
    let file = (chars.next().unwrap() as u8) - b'A';
    let rank = (chars.next().unwrap() as u8) - b'1';
    (rank * 8 + file) as usize
}

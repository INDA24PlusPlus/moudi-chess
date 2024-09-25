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

    // #[test]
    // fn stalemate() {
    //     let chess = Chess::from_fen("8/8/2p1k3/8/8/1q6/8/K7 w - - 0 1".to_string());
    //     assert!(chess.get_state() == State::Stalemate);
    // }

    // #[test]
    // fn check() {
    //     let chess = Chess::from_fen("8/8/2p1k3/8/8/8/1q6/K7 w - - 0 1".to_string());
    //     assert!(chess.get_state() == State::Check);
    // }

    // #[test]
    // fn checkmate() {
    //     let chess = Chess::from_fen("8/8/2p5/8/8/k7/1q6/K7 w - - 0 1".to_string());
    //     assert!(chess.get_state() == State::Checkmate);
    // }

    // #[test]
    // fn promotion() {
    //     let mut chess = Chess::from_fen("8/6P1/2p1k3/8/8/1q6/8/4K3 w - - 0 1".to_string());
    //     let g7 = 6 * 8 + 6;
    //     let g8 = 7 * 8 + 6;

    //     assert!(chess.make_move(g7, g8));
    //     println!("State: {:?}", chess.get_state());
    //     assert!(chess.get_state() == State::Promotion);
    //     
    //     chess.promote(PieceType::Queen); 

    //     if let Some(queen) = chess.board.get_piece_at_pos(g8) {
    //         assert!(queen.get_piece_type() == PieceType::Queen);
    //     } else {
    //         assert!(false);
    //     }
    // }

    // #[test]
    // fn move_piece_to_block_check() {
    //     let mut chess = Chess::from_fen("8/1k6/1bp5/8/8/1q6/3P4/6K1 w - - 0 1".to_string());
    //     let d2 = 8 * 1 + 3;
    //     let d4 = 8 * 3 + 3;

    //     assert!(chess.make_move(d2, d4));
    // }

    // #[test]
    // fn move_piece_to_not_block_check() {
    //     let mut chess = Chess::from_fen("8/1k6/1bp5/8/8/1q6/3P4/6K1 w - - 0 1".to_string());
    //     let d2 = 8 * 1 + 3;
    //     let d3 = 8 * 2 + 3;

    //     assert!(!chess.make_move(d2, d3));
    // }

    #[test]
    fn cli() {
        cli::start();
    }
}

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

mod CLI;
use crate::CLI::*;

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn fen_parser_test() {
    //     let mut chess = Chess::new();
    //     chess.board = Board::from_fen("8/4Pp2/1qp1k3/6P1/2R4b/1Ppnp3/2PK4/2B4B w - - 0 1".to_string()).unwrap(); 
    //     // for piece in chess.get_all_pieces() {
    //     //     println!("{}", piece);
    //     // }
    //     let piece = &chess.get_all_pieces()[3];
    //     println!("{}", piece.get_possible_moves(&chess.board));
    //     println!("Piece: {}", piece);
    //     for (x, y) in chess.get_moves(piece.get_occupied_slot()) {
    //         println!("({}, {})", x, y);
    //     }
    // }

    #[test]
    fn open_cli() {
        CLI::start();
    }
}

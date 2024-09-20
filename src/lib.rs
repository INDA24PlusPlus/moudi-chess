mod chess;
pub use crate::chess::*;

mod board;
pub use crate::board::*;

mod pieces;
pub use crate::pieces::*;

mod bitboard;
pub use crate::bitboard::*;

mod CoordinateIterator;
use crate::CoordinateIterator::*;

mod file;
pub use crate::file::*;

mod CLI;
pub use crate::CLI::*;

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn fen_parser_test() {
    //     let board = Board::from_fen("8/4Pp2/1qp1k3/6P1/2R4b/1Ppnp3/2PK4/2B4B w - - 0 1".to_string()).unwrap(); 
    //     println!("White:\n{}", board.white);
    //     println!("Black:\n{}", board.black);
    //     let piece = board.get_piece_at_pos(5 + 8 * 6).expect("No piece found");
    //     println!("{}", piece);
    //     let moves = piece.get_possible_moves(&board);
    //     println!("Moves:\n{}", moves);
    // }

    #[test]
    fn open_cli() {
        CLI::start();
    }
}

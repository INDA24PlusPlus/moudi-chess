mod chess;
pub use crate::chess::*;

mod board;
pub use crate::board::*;

mod pieces;
pub use crate::pieces::*;

mod bitboard;
pub use crate::bitboard::*;

mod file;
pub use crate::file::*;

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn default_board_piece_count() {
    //     let game = Chess::new();
    //     let pieces = game.board.get_all_pieces();
    //     println!("White:\n{}", game.board.white);
    //     println!("Black:\n{}", game.board.black);
    //     println!("Piece count: {}", pieces.len());
    //     assert_eq!(pieces.len(), 32);
    // }

    // #[test]
    // fn correctness() {
    //     let mut board = bitboard::EMPTY;
    //     board.set(0, true);
    //
    //     println!("{}", board);
    // }

    #[test]
    fn fen_parser_test() {
        let board = Board::from_fen("8/4Pp2/1qp1k3/6P1/2R4b/1Ppnp3/2PK4/2B4B w - - 0 1".to_string()).unwrap(); 
        println!("White:\n{}", board.white);
        println!("Black:\n{}", board.black);
        let piece = board.get_piece_at_pos(5, 6).expect("No piece found");
        println!("{}", piece);
        let moves = piece.get_possible_moves(&board);
        println!("Moves:\n{}", moves);
    }
    //
    // #[test]
    // fn default_board_piece_movement() {
    //     let game = Chess::new();
    //     let pieces = game.board.get_all_pieces();
    //     let piece = pieces.get(10).expect("No piece found");
    //     let moves = piece.get_possible_moves(&game.board);
    //     println!("{}", moves);
    // }
}

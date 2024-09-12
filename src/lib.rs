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

    #[test]
    fn it_works() {
        let game = Chess::new();
    }
}

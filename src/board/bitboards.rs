use super::{Board, Piece, Side, BitBoard, PieceType};

impl Board {
    #[inline]
    pub(crate) fn get_opponent_board(&self, piece: &Piece) -> BitBoard {
        match piece.get_color() {
            Side::White => self.black,
            Side::Black => self.white
        }
    }

    #[inline]
    pub(crate) fn get_opponent_and_empty_squares_board(&self, side: Side) -> BitBoard {
        (match side {
            Side::White => self.black,
            Side::Black => self.white,
        }) | !(self.black | self.white)
    }

    pub(crate) fn get_sides_board(&self, side: Side) -> BitBoard {
        match side {
            Side::White => self.white,
            Side::Black => self.black,
        }
    }

    pub(crate) fn get_playing_sides_board(&self) -> BitBoard {
        match self.side {
            Side::White => self.white,
            Side::Black => self.black
        }
    }

    pub(crate) fn all_pieces_bitboard(&self) -> BitBoard {
        self.white | self.black
    }

    pub(crate) fn set_piece(&mut self, index: usize, piece: PieceType, side: Side, value: bool) {
        self.pieces[piece.to_value() as usize].set(index, value);
         match side {
            Side::White => self.white.set(index, value),
            Side::Black => self.black.set(index, value),
        };
    }

    pub(crate) fn print_side(&self, side: Side) {
        match side {
            Side::White => println!("{}", self.white),
            Side::Black => println!("{}", self.black),
        }
    }
}

use core::fmt::Display;
use crate::{bitboard, board::*, BitBoard};

pub(crate) mod pawn;
pub use pawn::*;
mod knight;
pub use knight::*;
mod bishop;
pub use bishop::*;
mod rook;
pub use rook::*;
mod queen;
pub use queen::*;
mod king;
pub use king::*;


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
/// The types of pieces
///
/// This is represents what is on a slot of the board and can therefore be Empty as well
pub enum PieceType {
    Empty,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

impl PieceType {
    pub fn to_value(self) -> usize {
        (self as usize) - 1
    }
    
    pub fn from_value(value: i8) -> PieceType {
        match value {
            0 => PieceType::Pawn,
            1 => PieceType::Knight,
            2 => PieceType::Bishop,
            3 => PieceType::Rook,
            4 => PieceType::Queen,
            5 => PieceType::King,
            _ => PieceType::Empty
        }
    }
}

pub struct Piece {
    piece: PieceType,
    color: Side,
    pos: (i8, i8)
}

impl Piece {
    pub fn new(piece: PieceType, color: Side, x: i8, y: i8) -> Self {
        Piece {
            piece,
            color,
            pos: (x, y)
        }
    }

    pub fn is_allowed_move(&self, board: &Board, index: usize) -> bool {
        match self.piece {
            PieceType::Pawn => pawn::is_allowed_move(self, board, index),
            PieceType::Knight => knight::is_allowed_move(self, board, index),
            PieceType::Bishop => bishop::is_allowed_move(self, board, index),
            PieceType::Rook => rook::is_allowed_move(self, board, index),
            PieceType::Queen => queen::is_allowed_move(self, board, index),
            PieceType::King => king::is_allowed_move(self, board, index),
            _ => false
        }
    }

    pub fn get_possible_moves(&self, board: &Board) -> BitBoard {
        let moves = match self.piece {
            PieceType::Pawn => pawn::get_all_moves(self, board),
            PieceType::Knight => knight::get_all_moves(self, board),
            PieceType::Bishop => bishop::get_all_moves(self, board),
            PieceType::Rook => rook::get_all_moves(self, board),
            PieceType::Queen => queen::get_all_moves(self, board),
            PieceType::King => king::get_all_moves(self, board),
            _ => bitboard::EMPTY,
        };
        
        moves
    }

    pub fn get_piece_type(&self) -> PieceType {
        self.piece
    }

    pub fn get_occupied_slot(&self) -> usize {
        (self.pos.1 * 8 + self.pos.0) as usize
    }

    pub fn get_pos_as_usize(&self) -> (usize, usize) {
        (self.pos.0 as usize, self.pos.1 as usize)
    }

    pub fn get_color(&self) -> Side {
        self.color
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}({}{})", self.color, self.piece, ((self.pos.0 as u8) + b'A') as char, self.pos.1 + 1)
    }
}

use std::convert::Into;
use crate::{board::*, BitBoard};

mod pawn;
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


#[derive(Copy, Clone, PartialEq, Eq)]
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
    pub fn to_value(self) -> u8 {
        (self as u8) - 1
    }
    
    pub fn from_value(value: u8) -> PieceType {
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
    pos: [u8; 2]
}

impl Piece {
    pub fn new(piece: PieceType, color: Side, x: u8, y: u8) -> Self {
        Piece {
            piece,
            color,
            pos: [x, y]
        }
    }

    pub fn attack(&self, board: &Board, x: u8, y: u8) -> bool {
        match self.piece {
            PieceType::Pawn => pawn::is_allowed_move(self, board, x, y),
            PieceType::Knight => knight::is_allowed_move(self, x, y),
            PieceType::Bishop => bishop::is_allowed_move(self, board, x, y),
            PieceType::Rook => rook::is_allowed_move(self, board, x, y),
            PieceType::Queen => queen::is_allowed_move(self, board, x, y),
            PieceType::King => king::is_allowed_move(self, board, x, y),
            _ => false
        }
    }

    pub fn get_piece_type(&self) -> PieceType {
        self.piece
    }

    pub fn get_occupied_slot(&self) -> usize {
        (self.pos[1] * 8 + self.pos[0]) as usize
   }

    pub fn get_color(&self) -> Side {
        self.color
    }
}


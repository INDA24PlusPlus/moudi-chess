use core::panic;
use std::convert::From;

use crate::pieces::*;
use crate::bitboard::{self, *};
use crate::file::*;

mod piece;
mod analysis;
mod bitboards;
mod state;

const NUM_PIECES: usize = 6;
pub(crate) const NUM_INDECES: usize = 64;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
/// The different player sides
pub enum Side {
    White,
    Black
}

impl Side {
    pub fn get_opposite(&self) -> Side {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) enum CastlingAbility {
    None,
    King,
    Queen,
    Both,
}

impl CastlingAbility {
    fn add(&mut self, rhs: Self) {
        *self = match *self {
            CastlingAbility::None => rhs,
            _ => CastlingAbility::Both
        };
    }

    fn inverse(&self) -> CastlingAbility {
        match *self {
            CastlingAbility::King => CastlingAbility::Queen,
            CastlingAbility::Queen => CastlingAbility::King,
            _ => panic!("Unable to inverse none or both castling ability"),
        }
    }

    pub fn remove(&mut self, castling_side: CastlingAbility) {
        match *self {
            CastlingAbility::None => *self = CastlingAbility::None,
            CastlingAbility::Both => *self = castling_side.inverse(),
            _ => {
                if castling_side == *self {
                    *self = CastlingAbility::None;
                }
            }
        }
    }

    pub fn has(&self, castling: CastlingAbility) -> bool {
        *self == CastlingAbility::Both || *self == castling
    }
}

/// All information about a chess board
pub struct Board {
    pieces: [ BitBoard; NUM_PIECES ], // piece placement
    white: BitBoard,    // placement of all white pieces
    black: BitBoard,    // placement of all black pieces
    side: Side,         // side to move
    castling: [CastlingAbility; 2], // castling rights [0: white, 1: black]
    ep_target: Option<i8>, // en passant target square
    moves_to_50: i8,    // halfmove clock
    move_counter: u32,  // fullmove clock

    white_attacking_king: Vec<Piece>, // all black pieces attacking white king
    black_attacking_king: Vec<Piece>, // all white pieces attacking black king
    white_attacked: BitBoard,   // all slots that are attacked by black
    black_attacked: BitBoard,   // all slots that are attacked by white
    white_pinned: BitBoard, // pinned white pieces
    black_pinned: BitBoard  // pinned black pieces
}

impl Default for Board {
    /// Get the default chess board layout for your chess board
    fn default() -> Board {
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()).unwrap()
    }
}

impl Board {
    pub(crate) fn new() -> Board {
        Board {
            pieces: [bitboard::EMPTY; NUM_PIECES],
            white: bitboard::EMPTY,
            black: bitboard::EMPTY,
            side: Side::White,
            castling: [CastlingAbility::None, CastlingAbility::None],
            ep_target: None,
            moves_to_50: 0,
            move_counter: 0,

            white_attacking_king: vec![],
            black_attacking_king: vec![],
            white_attacked: bitboard::EMPTY,
            black_attacked: bitboard::EMPTY,
            white_pinned: bitboard::EMPTY,
            black_pinned: bitboard::EMPTY,
        }
    }

    pub(crate) fn from_fen(fen: String) -> Result<Board, String> {
        let mut board = Board::new();
        let parts : Vec<_> = fen.split_whitespace().collect();
 
        for (rank, line) in parts[0].split('/').rev().enumerate() {
            let mut file = 0;
            for c in line.chars() {
                match c {
                    'p' => board.set_piece(rank * 8 + file, PieceType::Pawn, Side::Black, true),
                    'n' => board.set_piece(rank * 8 + file, PieceType::Knight, Side::Black, true),
                    'b' => board.set_piece(rank * 8 + file, PieceType::Bishop, Side::Black, true),
                    'r' => board.set_piece(rank * 8 + file, PieceType::Rook, Side::Black, true),
                    'q' => board.set_piece(rank * 8 + file, PieceType::Queen, Side::Black, true),
                    'k' => board.set_piece(rank * 8 + file, PieceType::King, Side::Black, true),
                    'P' => board.set_piece(rank * 8 + file, PieceType::Pawn, Side::White, true),
                    'N' => board.set_piece(rank * 8 + file, PieceType::Knight, Side::White, true),
                    'B' => board.set_piece(rank * 8 + file, PieceType::Bishop, Side::White, true),
                    'R' => board.set_piece(rank * 8 + file, PieceType::Rook, Side::White, true),
                    'Q' => board.set_piece(rank * 8 + file, PieceType::Queen, Side::White, true),
                    'K' => board.set_piece(rank * 8 + file, PieceType::King, Side::White, true),
                    '0'..='8' => file += c.to_digit(10).unwrap() as usize - 1,
                    _ => return Err(format!("Invalid FEN notation: {}", c)),
                }
                file += 1
            }
        }

        match parts[1] {
            "w" => board.side = Side::White,
            "b" => board.side = Side::Black,
            _ => return Err("Invalid FEN notation 2".to_string()),
        }
        
        for c in parts[2].chars() {
            match c {
                'K' => board.castling[0].add(CastlingAbility::King),
                'Q' => board.castling[0].add(CastlingAbility::Queen),
                'k' => board.castling[1].add(CastlingAbility::King),
                'q' => board.castling[1].add(CastlingAbility::Queen),
                '-' => break, // this should be handled in the new board initialization
                _ => return Err("Invalid FEN notation 3".to_string()),
            }
        }

        if parts[3].len() != 1 {
            let chars : Vec<_> = parts[3].chars().collect();
            let file = File::from(chars[0]) as i8;
            let rank = (chars[1].to_digit(10).unwrap() as i8) - 1;
            board.ep_target = Some(rank * 8 + file);
        }
        
        board.move_counter = parts[4].parse().unwrap();

        board.calculate_attacking_and_attacked(board.side);
        board.calculate_pinned_pieces(board.side);

        Ok(board)
    }

    #[inline]
    pub(crate) fn is_inbounds(x: usize, y: usize) -> bool {
        (0..8).contains(&x) && (0..8).contains(&y)
    }

    #[inline]
    pub(crate) fn is_empty(&self, index: usize) -> bool {
        self.get_piece_type_at_pos(index) == PieceType::Empty
    }

    #[inline]
    pub(crate) fn get_ep_target(&self) -> Option<i8> {
        self.ep_target
    }

    #[inline]
    pub(crate) fn get_playing_side(&self) -> Side {
        self.side
    }

    pub(crate) fn get_castling(&self, side: Side) -> CastlingAbility {
        match side {
            Side::White => self.castling[0],
            Side::Black => self.castling[1],
        }
    }

    #[inline]
    pub(crate) fn get_combined_piece_board(&self) -> BitBoard {
        self.white | self.black
    }

    #[inline]
    pub(crate) fn get_moves_to_50(&self) -> i8 {
        self.moves_to_50
    }

    pub(crate) fn get_side_computed_boards(&self, side: Side) -> (BitBoard, &Vec<Piece>, BitBoard) {
        match side {
            Side::White => (self.white_pinned, &self.white_attacking_king, self.white_attacked),
            Side::Black => (self.black_pinned, &self.black_attacking_king, self.black_attacked)
        }
    }

    fn take_en_passant(&mut self, piece: &Piece, ep_index: i8) {
        let index = (piece.get_pos_as_usize().1 as i8) * 8 + (ep_index % 8);
        self.set_piece(index as usize, piece.get_piece_type(), piece.get_color().get_opposite(), false);
    }
    
}

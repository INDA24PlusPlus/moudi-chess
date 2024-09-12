use std::convert::From;

use crate::pieces::*;
use crate::bitboard::{self, *};
use crate::file::*;

const NUM_PIECES: usize = 6;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Side {
    White,
    Black
}

enum CastlingAbility {
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
}

pub struct Board {
    pieces: [ BitBoard; NUM_PIECES ], // piece placement
    white: BitBoard, // placement of all white pieces
    black: BitBoard, // placement of all black pieces
    side: Side, // side to move
    castling: [CastlingAbility; 2], // castling rights [0: white, 1: black]
    ep_target: Option<u8>, // en passant target square
    moves_to_50: u8, // halfmove clock
    move_counter: u32 // fullmove clock
}

impl Default for Board {
    fn default() -> Board {
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()).unwrap()
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            pieces: [bitboard::EMPTY; NUM_PIECES],
            white: bitboard::EMPTY,
            black: bitboard::EMPTY,
            side: Side::White,
            castling: [CastlingAbility::None, CastlingAbility::None],
            ep_target: None,
            moves_to_50: 0,
            move_counter: 0,
        }
    }

    pub fn move_piece(&mut self, piece: Piece, x: u8, y: u8) -> bool {
        if !piece.attack(self, x, y) {
            return false;
        }

        // remove captured piece
        let capture = self.get_piece_type_at_pos(x, y);
        if capture != PieceType::Empty {
            self.pieces[capture.to_value() as usize].set((y * 8 + x) as usize, false);
        }
        // move current piece to new slot
        self.pieces[piece.get_piece_type().to_value() as usize].set(piece.get_occupied_slot(), false);
        self.pieces[piece.get_piece_type().to_value() as usize].set((y * 8 + x) as usize, true);

        return true;
    }

    pub fn get_piece_type_at_pos(&self, x: u8, y: u8) -> PieceType {
        if !self.all_pieces_bitboard().get(y * 8 + x) {
            return PieceType::Empty;
        }

        for (i, bitboard) in self.pieces.iter().enumerate() {
            if bitboard.get(y * 8 + x) {
                return PieceType::from_value(i as u8);
            }
        }

        PieceType::Empty
    }

    pub fn get_piece_at_pos(&mut self, x: u8, y: u8) -> Option<Piece> {
        let piecetype = self.get_piece_type_at_pos(x, y);
        
        if piecetype == PieceType::Empty {
            return None;
        }

        Some(Piece::new(piecetype, match self.white.get(y * 8 + x) {
            true => Side::White,
            false => Side::Black
        }, x, y))
    }

    pub fn get_all_pieces(&mut self) -> Vec<Piece> {
        (0..64).filter_map(|n| self.get_piece_at_pos(n % 8, n / 8)).collect::<Vec<_>>()
    }

    pub fn get_opponent_board(&self, piece: &Piece) -> &BitBoard {
        match piece.get_color() {
            Side::White => &self.black,
            Side::Black => &self.white
        }
    }

    pub fn get_sides_board(&self) -> BitBoard {
        match self.side {
            Side::White => self.white,
            Side::Black => self.black
        }
    }

    #[inline]
    pub fn is_inbounds(x: u8, y: u8) -> bool {
        x < 8 && y < 8
    }

    pub fn is_empty(&self, x: u8, y: u8) -> bool {
        self.get_piece_type_at_pos(x, y) == PieceType::Empty
    }

    pub fn from_fen(fen: String) -> Result<Board, String> {
        let mut board = Board::new();
        let parts : Vec<_> = fen.split_whitespace().collect();
 
        for (rank, line) in parts[0].split('/').rev().enumerate() {
            let mut file = 0;
            for c in line.chars() {
                match c {
                    'p' => board.set_piece(file, rank, PieceType::Pawn, Side::Black),
                    'P' => board.set_piece(file, rank, PieceType::Pawn, Side::White),
                    'n' => board.set_piece(file, rank, PieceType::Knight, Side::Black),
                    'N' => board.set_piece(file, rank, PieceType::Knight, Side::White),
                    'b' => board.set_piece(file, rank, PieceType::Bishop, Side::Black),
                    'B' => board.set_piece(file, rank, PieceType::Bishop, Side::White),
                    'r' => board.set_piece(file, rank, PieceType::Rook, Side::Black),
                    'R' => board.set_piece(file, rank, PieceType::Rook, Side::White),
                    'q' => board.set_piece(file, rank, PieceType::Queen, Side::Black),
                    'Q' => board.set_piece(file, rank, PieceType::Queen, Side::White),
                    'k' => board.set_piece(file, rank, PieceType::King, Side::Black),
                    'K' => board.set_piece(file, rank, PieceType::King, Side::White),
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
            let file = File::from(chars[0]) as u8;
            let rank = chars[1].to_digit(10).unwrap() as u8;
            board.ep_target = Some(file + rank * 8)
        }
        
        board.move_counter = parts[4].parse().unwrap();

        Ok(board)
    }

    fn set_piece(&mut self, x: usize, y: usize, piece: PieceType, side: Side) {
        let slot = y * 8 + x;
        self.pieces[piece.to_value() as usize].set(slot, true);
         match side {
            Side::White => self.white.set(slot, true),
            Side::Black => self.black.set(slot, true),
        };
    }

    fn all_pieces_bitboard(&self) -> BitBoard {
        self.white & self.black
    }
}

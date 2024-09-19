use core::panic;
use std::{borrow::BorrowMut, cmp::min, io::repeat};

use crate::{dualboard, CoordinateIterator::CoordinateIterator};

use crate::bitboard;

use super::{BitBoard, Board, DualBoard, PieceType, Side, NUM_INDECES};

impl Board {
    
    fn add_pinned(&self, pinned: &mut BitBoard, it: CoordinateIterator, opponent_pinner: BitBoard, sides_board: BitBoard) {
        let board = self.all_pieces_bitboard();
        let mut first_piece_index = 65;
        for (x, y) in it {
            let index = y * 8 + x;
            if !board.get(index) {
                continue;
            } else if first_piece_index == 65 && sides_board.get(index) {
                first_piece_index = index as usize;
                continue;
            } if first_piece_index != 65 && opponent_pinner.get(index) {
                pinned.set(first_piece_index, true);
                return;
            }
        }
    }

    pub fn calculate_pinned_pieces(&mut self, side: Side) {
        let sides_board = self.get_sides_board(side);
        let opponent = self.get_sides_board(side.get_opposite());
        let king_board = sides_board & self.pieces[PieceType::King.to_value()];

        assert!(king_board.to_number() != 0);
        
        let king_index = (NUM_INDECES) - (king_board.to_number().leading_zeros() + 1) as usize; 
        let king_pos = (king_index % 8, king_index / 8);

        let mut pinned = bitboard::EMPTY;
        let straight_pieces = (self.pieces[PieceType::Rook.to_value()] | self.pieces[PieceType::Queen.to_value()]) & opponent;
        let diagonal_pieces = (self.pieces[PieceType::Bishop.to_value()] | self.pieces[PieceType::Queen.to_value()] ) & opponent;
 
        // NORTH
        self.add_pinned(&mut pinned, CoordinateIterator::new(king_pos, (king_pos.0, 7)), straight_pieces, sides_board);
        // NORTH-EAST
        self.add_pinned(&mut pinned, CoordinateIterator::new(king_pos, (7, 7)), diagonal_pieces, sides_board);
        // EAST
        self.add_pinned(&mut pinned, CoordinateIterator::new(king_pos, (7, king_pos.1)), straight_pieces, sides_board);
        // SOUTH-EAST
        self.add_pinned(&mut pinned, CoordinateIterator::new(king_pos, (7, 0)), diagonal_pieces, sides_board);
        // SOUTH
        self.add_pinned(&mut pinned, CoordinateIterator::new(king_pos, (king_pos.0, 0)), straight_pieces, sides_board);
        // SOUTH-WEST
        self.add_pinned(&mut pinned, CoordinateIterator::new(king_pos, (0, 0)), diagonal_pieces, sides_board);
        // WEST
        self.add_pinned(&mut pinned, CoordinateIterator::new(king_pos, (0, king_pos.1)), straight_pieces, sides_board);
        // NORTH-WEST
        self.add_pinned(&mut pinned, CoordinateIterator::new(king_pos, (0, 7)), diagonal_pieces, sides_board);
        
        match side {
            Side::White => self.white_pinned = pinned,
            Side::Black => self.black_pinned = pinned,
        }
    }
    
    pub fn calculate_attacked(&mut self, side: Side) {
        let mut board = DualBoard::new();
        let pieces = self.get_all_pieces();

        for piece in pieces {
            if piece.get_color() != side  {
                continue;
            }
            let attack = piece.get_possible_moves(self);
            board.add(attack);
        }

        match side {
            Side::White => self.white_moves = board,
            Side::Black => self.black_moves = board,
        }
    }

}

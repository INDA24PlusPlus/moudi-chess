use crate::{bitboard, CoordinateIterator};

use super::{BitBoard, Board, Piece, PieceType, Side};

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
        
        let king_index = self.get_king(side);
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
        let mut attacking = vec![];
        let mut attacked = bitboard::EMPTY;
        let king_index = self.get_king(side);
        let pieces = self.get_all_pieces();

        for piece in pieces {
            if piece.get_color() == side  {
                continue;
            }

            let attack = piece.get_possible_moves(self);
            if attack.get(king_index) {
                attacking.push(piece);
            }

            attacked |= attack;
        }

        match side {
            Side::White => {
                self.white_attacking_king = attacking;
                self.white_attacked = attacked;
            },
            Side::Black => {
                self.black_attacking_king = attacking;
                self.black_attacked = attacked;
            },
        }
    }

    // checks:
    //  King is un-attacked
    //  Piece is not pinned
    //  King does not move to an attacked square
    pub fn check_external_piece_test(&self, piece: &Piece, x: usize, y: usize) -> bool {
        let (pinned, attacking, attacked) = self.get_side_computed_boards(piece.get_color());
 
        // if piece is pinned
        if pinned.get(piece.get_occupied_slot()) {
            return false;
        } else if piece.get_piece_type() == PieceType::King {
            // king moves to attacked square
            if attacked.get(y * 8 + x) {
                return false;
            }
            // if taking this piece results in being attacked
            else {

            }
        } else {
            // king is attacked by more than two and trying to move another piece
            if attacking.len() >= 2 {
                return false;
            } else if attacking.len() == 1 && !self.will_block_king_attack(piece, &attacking[0], x, y) {
                return false;
            }
        }

        true
    }

    fn will_block_king_attack(&self, piece: &Piece, attacking_piece: &Piece, x: usize, y: usize) -> bool {
        match piece.get_piece_type() {
            PieceType::Pawn | PieceType::Knight => return false,
            _ => {}
        }

        let king_index = self.get_king(piece.get_color());
        CoordinateIterator::new(attacking_piece.get_pos_as_usize(), (king_index % 8, king_index / 8)).contains((x, y))
    }

}

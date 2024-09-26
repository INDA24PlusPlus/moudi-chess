use crate::{bitboard, CoordinateIterator};

use super::{BitBoard, Board, Piece, PieceType, Side};

impl Board {
    
    fn add_pinned(&self, pinned: &mut BitBoard, it: CoordinateIterator, opponent_pinner: BitBoard, sides_board: BitBoard) {
        let board = self.all_pieces_bitboard();
        let mut first_piece_index = 65;

        for (x, y) in it {
            let index = y * 8 + x;

            if !board.get(index) {}
            else if first_piece_index == 65 && sides_board.get(index) {
                first_piece_index = index as usize;
            } else if first_piece_index != 65 && opponent_pinner.get(index) {
                pinned.set(first_piece_index, true);
                break;
            }
        }
    }

    pub(crate) fn calculate_pinned_pieces(&mut self, side: Side) {
        let sides_board = self.get_sides_board(side);
        let opponent = self.get_sides_board(side.get_opposite());
        
        let king_index = self.get_king(side);
        let king_pos = (king_index % 8, king_index / 8);

        let mut pinned = bitboard::EMPTY;
        let straight_pieces = (self.pieces[PieceType::Rook.to_value()] | self.pieces[PieceType::Queen.to_value()]) & opponent;
        let diagonal_pieces = (self.pieces[PieceType::Bishop.to_value()] | self.pieces[PieceType::Queen.to_value()] ) & opponent;
 
        // NORTH
        self.add_pinned(&mut pinned, CoordinateIterator::from_to(king_pos, (king_pos.0, 7)), straight_pieces, sides_board);
        // NORTH-EAST
        self.add_pinned(&mut pinned, CoordinateIterator::from_to(king_pos, (7, 7)), diagonal_pieces, sides_board);
        // EAST
        self.add_pinned(&mut pinned, CoordinateIterator::from_to(king_pos, (7, king_pos.1)), straight_pieces, sides_board);
        // SOUTH-EAST
        self.add_pinned(&mut pinned, CoordinateIterator::from_to(king_pos, (7, 0)), diagonal_pieces, sides_board);
        // SOUTH
        self.add_pinned(&mut pinned, CoordinateIterator::from_to(king_pos, (king_pos.0, 0)), straight_pieces, sides_board);
        // SOUTH-WEST
        self.add_pinned(&mut pinned, CoordinateIterator::from_to(king_pos, (0, 0)), diagonal_pieces, sides_board);
        // WEST
        self.add_pinned(&mut pinned, CoordinateIterator::from_to(king_pos, (0, king_pos.1)), straight_pieces, sides_board);
        // NORTH-WEST
        self.add_pinned(&mut pinned, CoordinateIterator::from_to(king_pos, (0, 7)), diagonal_pieces, sides_board);
        
        match side {
            Side::White => self.white_pinned = pinned,
            Side::Black => self.black_pinned = pinned,
        }
    }
    
    pub(crate) fn calculate_attacking_and_attacked(&mut self, side: Side) {
        let mut attacking = vec![];
        let mut attacked = bitboard::EMPTY;
        let king_index = self.get_king(side);
        let pieces = self.get_all_pieces();

        for piece in pieces {
            if piece.get_color() == side  {
                continue;
            }

            let attack = piece.get_attacked_square(self);

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

    pub(crate) fn is_king_safety(&self, piece: &Piece, x: usize, y: usize) -> bool {
        let (pinned, attacking, attacked) = self.get_side_computed_boards(piece.get_color());
 
        // if piece is pinned
        if pinned.get(piece.get_occupied_slot()) && !self.still_pinned_piece_at_pos(piece, x, y) {
            return false;
        } else if piece.get_piece_type() == PieceType::King {
            // king moves to attacked square
            if attacked.get(y * 8 + x) {
                return false;
            }
            // if king move still causes it to be attacked
            if self.king_moving_away_from_attacker(piece, attacking, x, y) {
                return false;
            }
        } else {
            // king is attacked by more than two and trying to move another piece
            if attacking.len() >= 2 {
                return false;
            }
            if attacking.len() == 1 && !self.will_block_or_capture_king_attack(piece, &attacking[0], x, y) {
                return false;
            }
        }

        true
    }

    fn will_block_or_capture_king_attack(&self, piece: &Piece, attacking_piece: &Piece, x: usize, y: usize) -> bool {
        match attacking_piece.get_piece_type() {
            PieceType::Pawn | PieceType::Knight => return y * 8 + x == attacking_piece.get_occupied_slot(),
            _ => {}
        }
        
        if y * 8 + x == attacking_piece.get_occupied_slot() {
            return true;
        }

        let king_index = self.get_king(piece.get_color());
        CoordinateIterator::from_to(attacking_piece.get_pos_as_usize(), (king_index % 8, king_index / 8)).contains((x, y))
    }

    pub(crate) fn is_no_possible_moves(&self, side: Side) -> bool {

        for piece in self.get_all_pieces() {
            if piece.get_color() == side && piece.get_possible_moves(self).to_number() != 0 {
                return false;
            }
        }

        true
    }

    pub(crate) fn still_pinned_piece_at_pos(&self, piece: &Piece, x: usize, y: usize) -> bool {
        let king_index = self.get_king(piece.get_color());
        let piece_movement = CoordinateIterator::from_to(piece.get_pos_as_usize(), (x, y)).get_change();
        let king_to_piece = CoordinateIterator::from_to((king_index % 8, king_index / 8), piece.get_pos_as_usize()).get_change();

        piece_movement == king_to_piece || (piece_movement.0 == -king_to_piece.0 && piece_movement.1 == -king_to_piece.1)
    }

    pub(crate) fn king_moving_away_from_attacker(&self, piece: &Piece, attackers: &Vec<Piece>, x: usize, y: usize) -> bool {
        let king_pos = piece.get_pos_as_usize();
        let move_dir = CoordinateIterator::from_to(king_pos, (x, y)).get_change();
        
        for attacker in attackers {
            let piece_to_king = CoordinateIterator::from_to(attacker.get_pos_as_usize(), king_pos).get_change();
            if move_dir == piece_to_king {
                return true;
            }
        }
        
        false
    }

}

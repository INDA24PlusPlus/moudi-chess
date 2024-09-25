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

            let attack = match piece.get_piece_type() {
                PieceType::Pawn => crate::pieces::pawn::get_attack_bitboard(&piece, self),
                _ => piece.get_possible_moves(self),
            };

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
            // if taking this piece results in being attacked
            if let Some(capturing_piece) = self.get_piece_at_pos(y * 8 + x) {
                // if piece is not same color
                if capturing_piece.get_color() != piece.get_color() && self.is_guarded_piece(&capturing_piece) {
                    return false;
                }
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
            PieceType::Pawn | PieceType::Knight => return false,
            _ => {}
        }
        if y * 8 + x == attacking_piece.get_occupied_slot() {
            return true;
        }

        let king_index = self.get_king(piece.get_color());
        CoordinateIterator::from_to(attacking_piece.get_pos_as_usize(), (king_index % 8, king_index / 8)).contains((x, y))
    }

    fn is_guarded_piece(&self, piece: &Piece) -> bool {
        let mut board = Board {
            pieces: self.pieces.clone(),
            white: self.white.clone(),
            black: self.black.clone(),
            side: self.side.clone(),
            castling: self.castling.clone(),
            ep_target: self.ep_target.clone(),
            moves_to_50: 0,
            move_counter: 0,

            white_attacking_king: vec![],
            black_attacking_king: vec![],
            white_attacked: bitboard::EMPTY,
            black_attacked: bitboard::EMPTY,
            white_pinned: bitboard::EMPTY,
            black_pinned: bitboard::EMPTY,
        };

        let index = piece.get_occupied_slot();
        let side = piece.get_color();
        // remove the piece from the board so we can do a get_possible_move and check if that index
        // is a possible move
        board.set_piece(index, piece.get_piece_type(), piece.get_color(), false);

        for protecting_piece in board.get_all_pieces() {
            if protecting_piece.get_color() != side {
                continue;
            }

            let attack = match protecting_piece.get_piece_type() {
                PieceType::Pawn => crate::pieces::pawn::get_attack_bitboard(&protecting_piece, &board),
                _ => protecting_piece.get_possible_moves(&board),
            };

            if attack.get(index) {
                return true;
            }
        }

        false
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

}

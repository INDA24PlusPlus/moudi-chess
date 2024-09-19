use std::{collections::btree_map::IterMut, iter::FilterMap, path::Iter};

use super::{Board, Piece, PieceType, Side, NUM_INDECES};

impl Board {
    pub fn move_piece(&mut self, piece: &Piece, index: usize) -> bool {
        if !piece.is_allowed_move(self, index) {
            return false;
        }

        // if there is an active en passant target
        if let Some(ep_index) = self.ep_target {
            if index == (ep_index as usize) {
                self.take_en_passant(&piece, ep_index);
            }
            self.ep_target = None;
        }
        
        // swtich the active playing side here so that we can use that in the discarding of the
        // captured piece
        self.side = self.side.get_opposite();

        // remove captured piece
        let capture_type = self.get_piece_type_at_pos(index);
        if capture_type != PieceType::Empty {
            self.set_piece(index as usize, capture_type, piece.get_color().get_opposite(), false);
            self.moves_to_50 = 0; // reset to 0 on capture
        } else if piece.get_piece_type() == PieceType::Pawn {
            self.encode_en_passant(&piece, index as i8);
            self.moves_to_50 = 0; // reset if a pawn is moved
        } else {
            self.moves_to_50 += 1;
        }

        // move current piece to new index
        self.set_piece(piece.get_occupied_slot(), piece.get_piece_type(), piece.get_color(), false);
        self.set_piece(index as usize, piece.get_piece_type(), piece.get_color(), true);

        if self.side == Side::Black {
            self.move_counter += 1;
        }

        self.calculate_attacked(self.side);
        self.calculate_pinned_pieces(self.side);

        true
    }

    pub fn get_piece_type_at_pos(&self, index: usize) -> PieceType {
        if !self.all_pieces_bitboard().get(index) {
            return PieceType::Empty;
        }

        for (i, bitboard) in self.pieces.iter().enumerate() {
            if bitboard.get(index) {
                return PieceType::from_value(i as i8);
            }
        }

        PieceType::Empty
    }

    fn index_to_piece(&self, index: usize, piecetype: PieceType) -> Piece {
        Piece::new(piecetype, match self.white.get(index) {
            true => Side::White,
            _ => Side::Black
        }, (index % 8) as i8, (index / 8) as i8)
    }

    pub fn get_piece_at_pos(&self, index: usize) -> Option<Piece> {
        let piecetype = self.get_piece_type_at_pos(index);
        
        if piecetype == PieceType::Empty {
            return None;
        }

        Some(self.index_to_piece(index, piecetype))
    }

    pub fn get_all_pieces(&self) -> Vec<Piece> {
        (0..NUM_INDECES).filter_map(|n| self.get_piece_at_pos(n)).collect::<Vec<_>>()
    }
    
    fn encode_en_passant(&mut self, piece: &Piece, new_index: i8) {
        if ((piece.get_occupied_slot() as i8) - new_index).abs() == 16 {
            self.ep_target = Some(new_index);
        }
    }
}

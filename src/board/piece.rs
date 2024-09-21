use crate::bitboard;

use super::{BitBoard, Board, Piece, PieceType, Side, NUM_INDECES};

impl Board {
    pub(crate) fn move_piece(&mut self, piece: &Piece, index: usize) -> bool {
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

        // remove captured piece
        let capture_type = self.get_piece_type_at_pos(index);
        if capture_type != PieceType::Empty {
            self.set_piece(index, capture_type, piece.get_color().get_opposite(), false);
            self.moves_to_50 = 0; // reset to 0 on capture
        } else if piece.get_piece_type() == PieceType::Pawn {
            self.encode_en_passant(&piece, index as i8);
            self.moves_to_50 = 0; // reset if a pawn is moved
        } else {
            self.moves_to_50 += 1;
        }

        // move current piece to new index
        self.set_piece(piece.get_occupied_slot(), piece.get_piece_type(), piece.get_color(), false);
        self.set_piece(index, piece.get_piece_type(), piece.get_color(), true);

        // castling 
        if piece.get_piece_type() == PieceType::King && index.abs_diff(piece.get_occupied_slot()) == 2 {
            let (rook_offset, rook_x): (i8, usize) = match (index as i8 - piece.get_occupied_slot() as i8).signum() {
                1 => (-1, 7),
                -1 => (1, 0),
                _ => (0, 0),
            };
            if rook_offset != 0 {
                self.set_piece((index / 8) * 8 + rook_x, PieceType::Rook, piece.get_color(), false);
                self.set_piece((index as i8 + rook_offset) as usize, PieceType::Rook, piece.get_color(), true);
            }
        }

        self.update_castling_ability(piece);
        
        if self.side == Side::Black {
            self.move_counter += 1;
        }
        
        self.update_calculations();

        true
    }

    pub(crate) fn get_piece_type_at_pos(&self, index: usize) -> PieceType {
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

    pub(crate) fn get_piece_at_pos(&self, index: usize) -> Option<Piece> {
        let piecetype = self.get_piece_type_at_pos(index);
        
        if piecetype == PieceType::Empty {
            return None;
        }

        Some(self.index_to_piece(index, piecetype))
    }

    pub(crate) fn get_all_pieces(&self) -> Vec<Piece> {
        (0..NUM_INDECES).filter_map(|n| self.get_piece_at_pos(n)).collect::<Vec<_>>()
    }
    
    pub(crate) fn get_king(&self, side: Side) -> usize {
        let king_board = self.get_sides_board(side) & self.pieces[PieceType::King.to_value()];

        assert!(king_board.to_number() != 0);
        
        return (NUM_INDECES) - (king_board.to_number().leading_zeros() + 1) as usize;
    }

    pub(crate) fn check_and_set_piece_iter<F>(&self, piece: &Piece, moves: impl Iterator<Item = (usize, usize)>, action: F) -> BitBoard 
        where F: Fn(&mut BitBoard, usize, usize) -> bool
    {
        let mut board = bitboard::EMPTY;
        for (x, y) in moves {
            if !Board::is_inbounds(x, y) || !self.is_king_safety(piece, x, y) {
                continue
            } else if action(&mut board, x, y) {
                break;
            }
        }

        board
    }
}

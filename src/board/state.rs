use crate::{Board, Piece, Side, CastlingAbility, PieceType};

impl Board {
    pub(crate) fn update_calculations(&mut self) {
        // recalculate attacked pieces for checkmate test
        self.calculate_attacking_and_attacked(self.side);
        self.calculate_attacking_and_attacked(self.side.get_opposite());
        self.calculate_pinned_pieces(self.side.get_opposite());
    }

    pub(crate) fn update_castling_ability(&mut self, piece: &Piece) {
        let color = match piece.get_color() {
            Side::White => 0,
            Side::Black => 1,
        };

        match piece.get_piece_type() {
            PieceType::King => {
                self.castling[color] = CastlingAbility::None;
            },
            PieceType::Rook => {
                if self.castling[color] != CastlingAbility::None {
                    return;
                }

                match piece.get_pos_as_usize().0 {
                    0 => self.castling[color].remove(CastlingAbility::King),
                    7 => self.castling[color].remove(CastlingAbility::Queen),
                    _ => {}
                }
            },
            _ => {}
        }
    }

    pub(crate) fn encode_en_passant(&mut self, piece: &Piece, new_index: i8) {
        if piece.get_occupied_slot().abs_diff(new_index as usize) == 16 {
            self.ep_target = Some(new_index);
        }
    }

    pub(crate) fn is_only_kings_left(&self) -> bool {
        let mut board = self.get_combined_piece_board();
        // remove kings from board
        board.set(self.get_king(self.get_playing_side()), false);
        board.set(self.get_king(self.get_playing_side().get_opposite()), false);

        board.to_number() == 0
    }

}

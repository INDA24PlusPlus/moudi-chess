use crate::{Board, Piece, PieceType};

pub struct Chess {
    pub board: Board,
    state: State,
    promoting_index: Option<usize>
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum State {
    Playing,
    Check,
    CheckMate,
    Stalemate,
    Promotion
}

impl Default for Chess {
    fn default() -> Self {
        Chess::new()
    }
}

impl Chess {
    pub fn new() -> Chess {
        Chess {
            board: Board::default(),
            state: State::Playing,
            promoting_index: None,
        }
    }

    // Check if a piece at index is selectable
    pub fn is_selectable(&self, index: usize) -> bool {
        self.board.get_playing_sides_board().get(index)
    }

    // Attempt to move a piece at start_index to end_index and return whether if it was possible or
    // not
    pub fn make_move(&mut self, start_index: usize, end_index: usize) -> bool {
        if self.state == State::Promotion {
            return false;
        }

        if let Some(piece) = self.board.get_piece_at_pos(start_index) {
            self.board.move_piece(&piece, end_index);

            let y = end_index / 8;
            if piece.get_piece_type() == PieceType::Pawn && (y == 0 || y == 7) {
                self.promoting_index = Some(end_index);
            } else {
                self.update_state();
            }

            return true;
        }

        false
    }

    fn update_state(&mut self) {
        if self.promoting_index != None {
            self.state = State::Promotion;
        } else if self.board.get_side_computed_boards(self.board.get_playing_side().get_opposite()).1.len() != 0 {
            if self.board.get_side_computed_boards(self.board.get_playing_side()).2.to_number() == 0 {
                self.state = State::CheckMate;
            } else {
                self.state = State::Check;
            }
        } else if self.board.get_side_computed_boards(self.board.get_playing_side()).2.to_number() == 0 {
            self.state = State::Stalemate;
        } else {
            self.state = State::Playing;
        }
    }

    // Replace a promoting pawn with a piecetype of type new_piece
    pub fn promote(&mut self, new_piece: PieceType) {
        if let Some(index) = self.promoting_index {
            let color = self.board.get_playing_side().get_opposite();
            self.board.set_piece(index, PieceType::Rook, color, false);
            self.board.set_piece(index, new_piece, color, true);

            self.board.update_calculations();
            self.promoting_index = None;
            self.update_state();
        }
    }

    // Return all pieces on the board
    pub fn get_all_pieces(&self) -> Vec<Piece> {
        self.board.get_all_pieces()
    }

    // Get the current state of the game
    pub fn get_state(&self) -> State {
        self.state
    }


    
}

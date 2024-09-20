use crate::{Board, Piece};

pub struct Chess {
    pub board: Board,
    state: State
}

#[derive(Copy, Clone)]
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
        }
    }

    pub fn is_selectable(&self, x: usize, y: usize) -> bool {
        self.board.get_playing_sides_board().get(y * 8 + x)
    }

    pub fn make_move(&mut self, start_index: usize, end_index: usize) {
        if let Some(piece) = self.board.get_piece_at_pos(start_index) {
            self.board.move_piece(&piece, end_index);

            if self.board.get_side_computed_boards(piece.get_color().get_opposite()).1.len() != 0 {
                if self.board.get_side_computed_boards(piece.get_color()).2.to_number() == 0 {
                    self.state = State::CheckMate;
                } else {
                    self.state = State::Check;
                }
            } else if self.board.get_side_computed_boards(piece.get_color()).2.to_number() == 0 {
                self.state = State::Stalemate;
            }
        }
    }

    pub fn get_all_pieces(&self) -> Vec<Piece> {
        self.board.get_all_pieces()
    }

    pub fn get_state(&self) -> State {
        self.state
    }


    
}

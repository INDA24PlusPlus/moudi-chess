use crate::Board;

pub struct Chess {
    pub board: Board,
}

impl Default for Chess {
    fn default() -> Self {
        Chess::new()
    }
}

impl Chess {
    pub fn new() -> Chess {
        Chess {
            board: Board::default()
        }
    }

    pub fn is_selectable(&self, x: usize, y: usize) -> bool {
        self.board.get_playing_sides_board().get(y * 8 + x)
    }

    pub fn make_move(&mut self, start_index: usize, end_index: usize) {
        if let Some(piece) = self.board.get_piece_at_pos(start_index) {
            self.board.move_piece(&piece, end_index);
        }
    }
    
}

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

    pub fn is_selectable(&self, x: u8, y: u8) -> bool {
        self.board.get_sides_board().get(y * 8 + x)
    }
    
    pub fn 

}

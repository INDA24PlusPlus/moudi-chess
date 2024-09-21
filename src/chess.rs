use crate::{Board, Piece, PieceType};

/// Initialize this object
///
/// This is the main chess game object
pub struct Chess {
    pub board: Board,
    state: State,
    promoting_index: Option<usize>
}

#[derive(Copy, Clone, Eq, PartialEq)]
/// Special moments in the game
///
/// State is used to differentiate between neccessary key moments on the board
///
/// # Playing: The game is ongoing as normal
/// # Check: The current player is in check
/// # Checkmate: The current player is now in checkmate meaning that they lost
/// # Stalemate: The game is in stalemate
/// # Draw: The game is a draw
/// # Promotion: There is a pawn promotion that must be handled
///
pub enum State {
    Playing,
    Check,
    Checkmate,
    Stalemate,
    Draw,
    Promotion
}

impl Default for Chess {
    fn default() -> Self {
        Chess::new()
    }
}

impl Chess {
    /// Create a new chess game
    pub fn new() -> Chess {
        Chess {
            board: Board::default(),
            state: State::Playing,
            promoting_index: None,
        }
    }

    /// Check if a piece at index is selectable
    ///
    /// # Example:
    /// ```
    /// let chess = Chess::new();
    /// let user_clicked_index = 4;
    /// if !chess.is_selectable(user_clicked_index) {
    ///     panic!("User can not choose this index");
    /// }
    /// ```
    pub fn is_selectable(&self, index: usize) -> bool {
        self.board.get_playing_sides_board().get(index)
    }

    /// Move a piece from index to another index
    ///
    /// # Return: If the move was possible
    ///
    /// # Example:
    /// ```
    /// let chess = Chess::new();
    /// let d2 = 1 * 8 + 3;
    /// let d4 = 3 * 8 + 3;
    /// chess.make_move(d2, d4);
    /// ```
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

    pub fn get_moves(&self, index: usize) -> Vec<(usize, usize)> {
        let mut moves = vec![];

        if let Some(piece) = self.board.get_piece_at_pos(index) {
            moves.extend(piece.get_possible_moves(&self.board).map(|index| (index % 8, index / 8)));
        }

        moves
    }

    fn update_state(&mut self) {
        if self.promoting_index != None {
            self.state = State::Promotion;
        } else if self.board.get_side_computed_boards(self.board.get_playing_side().get_opposite()).1.len() != 0 {
            if self.board.get_side_computed_boards(self.board.get_playing_side()).2.to_number() == 0 {
                self.state = State::Checkmate;
            } else {
                self.state = State::Check;
            }
        } else if self.board.get_side_computed_boards(self.board.get_playing_side()).2.to_number() == 0 {
            self.state = State::Stalemate;
        } else if self.board.is_only_kings_left() {
            self.state = State::Draw;
        } else {
            self.state = State::Playing;
        }
    }

    /// Replace a promoting pawn with a piecetype of type new_piece
    ///
    /// # Example:
    /// ```
    /// let chess = Chess::new();
    /// let b7 = 6 * 8 + 1;
    /// let b8 = 7 * 8 + 1;
    /// chess.make_move(b7, b8);
    /// if chess.get_state() == State::Promotion {
    ///     chess.promote(PieceType::Queen); // promote pawn to a queen
    /// }
    /// ```
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

    /// Get all pieces on the board
    ///
    /// # Example:
    /// Print all pieces
    /// ```
    /// let chess = Chess::new();
    /// // do something
    /// for piece in chess.get_all_pieces() {
    ///     println!("{}", piece);
    /// }
    /// ```
    pub fn get_all_pieces(&self) -> Vec<Piece> {
        self.board.get_all_pieces()
    }

    /// Get the current state of the game
    ///
    /// # Example:
    /// ```
    /// let chess = Chess::new();
    /// // do something
    /// while chess.get_state() == State::Playing {
    ///     // do something
    /// }
    /// panic!("Something happend. No longer normal play");
    /// ```
    pub fn get_state(&self) -> State {
        self.state
    }


    
}

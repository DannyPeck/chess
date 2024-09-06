use crate::{
    board::{self, Board, MoveError, MoveRequest, MoveState},
    fen,
};

#[derive(Debug)]
pub struct Game {
    board: Board,
    index: usize,
    history: Vec<String>,
}

impl Game {
    pub fn new(board: Board) -> Game {
        let board_fen = fen::generate(&board);
        Game {
            board,
            index: 0,
            history: vec![board_fen],
        }
    }

    pub fn next(&mut self) -> bool {
        if self.index + 1 < self.history.len() {
            self.index += 1;

            let next_board = &self.history[self.index];
            self.board = fen::parse(&next_board).unwrap();

            true
        } else {
            false
        }
    }

    pub fn previous(&mut self) -> bool {
        if self.index > 0 {
            self.index -= 1;

            let previous_board = &self.history[self.index];
            self.board = fen::parse(&previous_board).unwrap();

            true
        } else {
            false
        }
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn attempt_move(&mut self, request: MoveRequest) -> Result<(), MoveError> {
        let move_state = board::get_move_state(&self.board);
        if move_state == MoveState::Checkmate || move_state == MoveState::Stalemate {
            return Err(MoveError::new("Game is over."));
        }

        let all_legal_moves = board::get_all_legal_moves(&self.board);
        let valid_move = all_legal_moves
            .get(&request.start)
            .map_or(false, |piece_moves| piece_moves.get(&request.end).is_some());

        if !valid_move {
            return Err(MoveError::new("Invalid move."));
        }

        board::move_piece(&mut self.board, request)?;

        // Add the new board state to the top of the stack
        let new_fen = fen::generate(&self.board);

        // If a move is attempted while pointing to an older board state, delete the
        // future states because the user has changed history.
        let current_length = self.index + 1;
        if current_length < self.history.len() {
            self.history.resize(current_length, String::new());
        }

        self.history.push(new_fen);
        self.index += 1;

        Ok(())
    }

    pub fn get_white_score(&self) -> i32 {
        let mut score = 0;
        for position in self.board.get_white_positions() {
            if let Some(piece) = self.board.get_piece(position) {
                score += piece.piece_type.value();
            }
        }

        score
    }

    pub fn get_black_score(&self) -> i32 {
        let mut score = 0;
        for position in self.board.get_black_positions() {
            if let Some(piece) = self.board.get_piece(position) {
                score += piece.piece_type.value();
            }
        }

        score
    }

    pub fn get_relative_score(&self) -> i32 {
        let white_score = self.get_white_score();
        let black_score = self.get_black_score();
        white_score - black_score
    }
}

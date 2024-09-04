use crate::{
    board::{Board, MoveError, MoveRequest},
    fen,
};

#[derive(Debug)]
pub struct Game {
    pub board: Board,
    index: usize,
    history: Vec<String>,
}

impl Game {
    pub fn new(board: Board) -> Game {
        let board_fen = fen::generate_fen(&board);
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
            self.board = fen::parse_fen(&next_board).unwrap();

            true
        } else {
            false
        }
    }

    pub fn previous(&mut self) -> bool {
        if self.index > 0 {
            self.index -= 1;

            let previous_board = &self.history[self.index];
            self.board = fen::parse_fen(&previous_board).unwrap();

            true
        } else {
            false
        }
    }

    pub fn attempt_move(&mut self, request: MoveRequest) -> Result<(), MoveError> {
        let result = self.board.move_piece(request);
        match result {
            Ok(_) => {
                // Add the new board state to the top of the stack
                let new_fen = fen::generate_fen(&self.board);

                // If a move is attempted while pointing to an older board state, delete the
                // future states because the user has changed history.
                let current_length = self.index + 1;
                if current_length < self.history.len() {
                    self.history.resize(current_length, String::new());
                }

                self.history.push(new_fen);
                self.index += 1;
            }
            Err(_) => {
                // Revert to previous board state
                let previous_board = &self.history[self.index];
                self.board = fen::parse_fen(&previous_board).unwrap();
            }
        }

        result
    }

    pub fn get_relative_score(&self) -> i32 {
        let white_score = self.board.get_white_score() as i32;
        let black_score = self.board.get_black_score() as i32;
        white_score - black_score
    }
}

use std::collections::HashMap;

use crate::{
    board::{self, Board, MoveError, MoveInfo, MoveRequest, MoveState, RepetitionState},
    fen,
};

#[derive(Debug)]
pub struct Game {
    board: Board,
    index: usize,
    history: Vec<String>,
    repetitions: HashMap<RepetitionState, u32>,
}

impl Game {
    pub fn new(board: Board) -> Game {
        let board_fen = fen::generate(&board);
        let repetition_state = board.get_repetition_state();
        Game {
            board,
            index: 0,
            history: vec![board_fen],
            repetitions: HashMap::from([(repetition_state, 1)]),
        }
    }

    pub fn next_move(&mut self) -> bool {
        if self.index + 1 < self.history.len() {
            self.index += 1;

            let next_board = &self.history[self.index];
            self.board = fen::parse(next_board).unwrap();

            true
        } else {
            false
        }
    }

    pub fn previous_move(&mut self) -> bool {
        if self.index > 0 {
            self.index -= 1;

            let previous_board = &self.history[self.index];
            self.board = fen::parse(previous_board).unwrap();

            true
        } else {
            false
        }
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn attempt_move(&mut self, request: MoveRequest) -> Result<MoveInfo, MoveError> {
        let move_state = self.get_move_state();
        if move_state == MoveState::Checkmate || move_state == MoveState::Stalemate {
            return Err(MoveError::new("Game is over."));
        }

        let all_legal_moves =
            board::get_all_legal_moves(&self.board, self.board.get_current_turn());

        let valid_move = all_legal_moves
            .get(&request.start)
            .map_or(false, |piece_moves| piece_moves.get(&request.end).is_some());
        if !valid_move {
            return Err(MoveError::new("Invalid move."));
        }

        // Calculate if we need to do any move disambiguation before we change the state of the board.
        let mut rank_disambiguation = false;
        let mut file_disambiguation = false;
        let moving_piece = self.board.get_piece(&request.start).unwrap();
        for (piece_position, moves) in all_legal_moves {
            if piece_position != request.start {
                let piece = self.board.get_piece(&piece_position).unwrap();
                if piece.piece_type == moving_piece.piece_type && moves.contains_key(&request.end) {
                    if piece_position.file() == request.start.file() {
                        rank_disambiguation = true;
                    }

                    if piece_position.rank() == request.start.rank() {
                        file_disambiguation = true;
                    }
                }
            }
        }

        let mut move_info = board::move_piece(&mut self.board, request)?;
        move_info.move_state = Some(self.get_move_state());
        move_info.rank_disambiguation = rank_disambiguation;
        move_info.file_disambiguation = file_disambiguation;

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

        let repetition_state = self.board.get_repetition_state();
        self.repetitions
            .entry(repetition_state)
            .and_modify(|v| *v += 1)
            .or_insert(1);

        Ok(move_info)
    }

    pub fn get_move_state(&self) -> MoveState {
        let mut stalemate_by_repetition = false;
        for repetition_count in self.repetitions.values() {
            if *repetition_count >= 3 {
                stalemate_by_repetition = true;
                break;
            }
        }

        if stalemate_by_repetition {
            MoveState::Stalemate
        } else {
            board::get_move_state(&self.board)
        }
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
}

#[cfg(test)]
mod test {
    use board::position::Position;

    use crate::{piece::PromotionType, ParseError};

    use super::*;

    #[test]
    fn test_normal_pawn_move_notation() -> Result<(), ParseError> {
        // Move forward
        {
            let board =
                fen::parse("rnbqkbnr/pp1p1ppp/8/2p1p3/3P4/P7/1PP1PPPP/RNBQKBNR w KQkq e6 0 3")?;
            let mut game = Game::new(board);

            let request = MoveRequest::new(Position::d4(), Position::d5());
            let result = game.attempt_move(request).unwrap();
            let notation = result.to_notation();
            assert_eq!(notation, "d5".to_string());
        }

        // Capture left
        {
            let board =
                fen::parse("rnbqkbnr/pp1p1ppp/8/2p1p3/3P4/P7/1PP1PPPP/RNBQKBNR w KQkq e6 0 3")?;
            let mut game = Game::new(board);

            let request = MoveRequest::new(Position::d4(), Position::c5());
            let result = game.attempt_move(request).unwrap();
            let notation = result.to_notation();
            assert_eq!(notation, "dxc5".to_string());
        }

        // Capture right
        {
            let board =
                fen::parse("rnbqkbnr/pp1p1ppp/8/2p1p3/3P4/P7/1PP1PPPP/RNBQKBNR w KQkq e6 0 3")?;
            let mut game = Game::new(board);

            let request = MoveRequest::new(Position::d4(), Position::e5());
            let result = game.attempt_move(request).unwrap();
            let notation = result.to_notation();
            assert_eq!(notation, "dxe5".to_string());
        }

        Ok(())
    }

    #[test]
    fn test_pawn_promotion() -> Result<(), ParseError> {
        // Promotion to Queen
        {
            let board =
                fen::parse("r1bqkbnr/pP3p2/2np3p/2p1p1p1/3P4/1P6/2P1PPPP/RNBQKBNR w KQkq - 0 8")?;
            let mut game = Game::new(board);

            let request =
                MoveRequest::promotion(Position::b7(), Position::b8(), PromotionType::Queen);
            let result = game.attempt_move(request).unwrap();
            let notation = result.to_notation();
            assert_eq!(notation, "b8=Q".to_string());
        }

        // Promotion to Knight
        {
            let board =
                fen::parse("r1bqkbnr/pP3p2/2np3p/2p1p1p1/3P4/1P6/2P1PPPP/RNBQKBNR w KQkq - 0 8")?;
            let mut game = Game::new(board);

            let request =
                MoveRequest::promotion(Position::b7(), Position::b8(), PromotionType::Knight);
            let result = game.attempt_move(request).unwrap();
            let notation = result.to_notation();
            assert_eq!(notation, "b8=N".to_string());
        }

        // Promotion to Rook
        {
            let board =
                fen::parse("r1bqkbnr/pP3p2/2np3p/2p1p1p1/3P4/1P6/2P1PPPP/RNBQKBNR w KQkq - 0 8")?;
            let mut game = Game::new(board);

            let request =
                MoveRequest::promotion(Position::b7(), Position::b8(), PromotionType::Rook);
            let result = game.attempt_move(request).unwrap();
            let notation = result.to_notation();
            assert_eq!(notation, "b8=R".to_string());
        }

        // Promotion to Bishop
        {
            let board =
                fen::parse("r1bqkbnr/pP3p2/2np3p/2p1p1p1/3P4/1P6/2P1PPPP/RNBQKBNR w KQkq - 0 8")?;
            let mut game = Game::new(board);

            let request =
                MoveRequest::promotion(Position::b7(), Position::b8(), PromotionType::Bishop);
            let result = game.attempt_move(request).unwrap();
            let notation = result.to_notation();
            assert_eq!(notation, "b8=B".to_string());
        }

        // Promotion by capture left
        {
            let board =
                fen::parse("r1bqkbnr/pP3p2/2np3p/2p1p1p1/3P4/1P6/2P1PPPP/RNBQKBNR w KQkq - 0 8")?;
            let mut game = Game::new(board);

            let request =
                MoveRequest::promotion(Position::b7(), Position::a8(), PromotionType::Queen);
            let result = game.attempt_move(request).unwrap();
            let notation = result.to_notation();
            assert_eq!(notation, "bxa8=Q".to_string());
        }

        // Promotion by capture right into check
        {
            let board =
                fen::parse("r1b1kbnr/pP1pqp2/2n4p/2p1p1p1/3P4/1P6/2P1PPPP/RNBQKBNR w KQkq - 1 8")?;
            let mut game = Game::new(board);

            let request =
                MoveRequest::promotion(Position::b7(), Position::c8(), PromotionType::Queen);
            let result = game.attempt_move(request).unwrap();
            let notation = result.to_notation();
            assert_eq!(notation, "bxc8=Q+".to_string());
        }

        Ok(())
    }

    #[test]
    fn test_knight_move_notation() -> Result<(), ParseError> {
        // Normal knight move
        {
            let board = Board::default();
            let mut game = Game::new(board);

            let request = MoveRequest::new(Position::b1(), Position::c3());
            let result = game.attempt_move(request).unwrap();
            let notation = result.to_notation();
            assert_eq!(notation, "Nc3".to_string());
        }

        // Knight file disambiguation
        {
            let board =
                fen::parse("rnb1kbnr/1pp2ppp/3p4/8/p3q3/2N3N1/PPPPPPPP/R1BQKB1R w KQkq - 0 8")?;
            let mut game = Game::new(board);

            let request = MoveRequest::new(Position::c3(), Position::e4());
            let result = game.attempt_move(request).unwrap();
            let notation = result.to_notation();
            assert_eq!(notation, "Ncxe4".to_string());
        }

        // Knight rank disambiguation
        {
            let board =
                fen::parse("rnb1kbnr/ppp1ppp1/3p4/2N5/4q2p/2N5/PPPPPPPP/R1BQKB1R w KQkq - 0 8")?;
            let mut game = Game::new(board);

            let request = MoveRequest::new(Position::c3(), Position::e4());
            let result = game.attempt_move(request).unwrap();
            let notation = result.to_notation();
            assert_eq!(notation, "N3xe4".to_string());
        }

        // Knight rank & file disambiguation
        {
            let board =
                fen::parse("rnb1kbnr/ppp1ppp1/3p4/2N5/4q2p/2N3N1/PPPPP1PP/R1BQKB1R w KQkq - 0 8")?;
            let mut game = Game::new(board);

            let request = MoveRequest::new(Position::c3(), Position::e4());
            let result = game.attempt_move(request).unwrap();
            let notation = result.to_notation();
            assert_eq!(notation, "Nc3xe4".to_string());
        }

        Ok(())
    }

    #[test]
    fn test_rook_move_notation() -> Result<(), ParseError> {
        // Normal rook move
        {
            let board = fen::parse("rnbqkbnr/1ppppppp/8/p7/P7/8/1PPPPPPP/RNBQKBNR w KQkq a6 0 2")?;
            let mut game = Game::new(board);

            let request = MoveRequest::new(Position::a1(), Position::a3());
            let result = game.attempt_move(request).unwrap();
            let notation = result.to_notation();
            assert_eq!(notation, "Ra3".to_string());
        }

        Ok(())
    }

    #[test]
    fn test_bishop_move_notation() -> Result<(), ParseError> {
        // Normal bishop move
        {
            let board =
                fen::parse("rnbqkbnr/ppp1pppp/8/3p4/3P4/8/PPP1PPPP/RNBQKBNR w KQkq d6 0 2")?;
            let mut game = Game::new(board);

            let request = MoveRequest::new(Position::c1(), Position::g5());
            let result = game.attempt_move(request).unwrap();
            let notation = result.to_notation();
            assert_eq!(notation, "Bg5".to_string());
        }

        Ok(())
    }

    #[test]
    fn test_queen_move_notation() -> Result<(), ParseError> {
        // Normal queen move
        {
            let board = fen::parse("rnbqkbnr/pppp1ppp/8/8/3p4/7P/PPP1PPP1/RNBQKBNR w KQkq - 0 3")?;
            let mut game = Game::new(board);

            let request = MoveRequest::new(Position::d1(), Position::d4());
            let result = game.attempt_move(request).unwrap();
            let notation = result.to_notation();
            assert_eq!(notation, "Qxd4".to_string());
        }

        Ok(())
    }

    #[test]
    fn test_king_move_notation() -> Result<(), ParseError> {
        // Normal king move
        {
            let board =
                fen::parse("rnbqkbnr/p2p4/1pp2pp1/7p/3p4/N2QBNPP/PPP1PPB1/R3K2R w KQkq - 0 9")?;
            let mut game = Game::new(board);

            let request = MoveRequest::new(Position::e1(), Position::d1());
            let result = game.attempt_move(request).unwrap();
            let notation = result.to_notation();
            assert_eq!(notation, "Kd1".to_string());
        }

        // Short Castle
        {
            let board =
                fen::parse("rnbqkbnr/p2p4/1pp2pp1/7p/3p4/N2QBNPP/PPP1PPB1/R3K2R w KQkq - 0 9")?;
            let mut game = Game::new(board);

            let request = MoveRequest::new(Position::e1(), Position::g1());
            let result = game.attempt_move(request).unwrap();
            let notation = result.to_notation();
            assert_eq!(notation, "O-O".to_string());
        }

        // Long Castle
        {
            let board =
                fen::parse("rnbqkbnr/p2p4/1pp2pp1/7p/3p4/N2QBNPP/PPP1PPB1/R3K2R w KQkq - 0 9")?;
            let mut game = Game::new(board);

            let request = MoveRequest::new(Position::e1(), Position::c1());
            let result = game.attempt_move(request).unwrap();
            let notation = result.to_notation();
            assert_eq!(notation, "O-O-O".to_string());
        }

        // Long Castle Checkmate
        {
            let board = fen::parse("3k4/8/8/2Q1Q3/8/8/8/R3K3 w Q - 0 1")?;
            let mut game = Game::new(board);

            let request = MoveRequest::new(Position::e1(), Position::c1());
            let result = game.attempt_move(request).unwrap();
            let notation = result.to_notation();
            assert_eq!(notation, "O-O-O#".to_string());
        }

        Ok(())
    }

    #[test]
    fn test_check_notation() -> Result<(), ParseError> {
        // Check
        {
            let board =
                fen::parse("rnbqkbnr/ppppp1pp/8/5p2/4P3/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 2")?;
            let mut game = Game::new(board);

            let request = MoveRequest::new(Position::d1(), Position::h5());
            let result = game.attempt_move(request).unwrap();
            let notation = result.to_notation();
            assert_eq!(notation, "Qh5+".to_string());
        }

        // Checkmate
        {
            let board =
                fen::parse("rnbqkbnr/ppppp2p/5p2/6p1/4P3/P7/1PPP1PPP/RNBQKBNR w KQkq g6 0 3")?;
            let mut game = Game::new(board);

            let request = MoveRequest::new(Position::d1(), Position::h5());
            let result = game.attempt_move(request).unwrap();
            let notation = result.to_notation();
            assert_eq!(notation, "Qh5#".to_string());
        }

        Ok(())
    }

    #[test]
    fn test_disambiguation() -> Result<(), ParseError> {
        // File disambiguation
        {
            let board = fen::parse("3r3r/8/8/R7/4Q2Q/8/8/R6Q b - - 0 1")?;
            let mut game = Game::new(board);

            let request = MoveRequest::new(Position::d8(), Position::f8());
            let result = game.attempt_move(request).unwrap();
            let notation = result.to_notation();
            assert_eq!(notation, "Rdf8".to_string());
        }

        // Rank disambiguation
        {
            let board = fen::parse("3r3r/8/8/R7/4Q2Q/8/8/R6Q w - - 0 1")?;
            let mut game = Game::new(board);

            let request = MoveRequest::new(Position::a1(), Position::a3());
            let result = game.attempt_move(request).unwrap();
            let notation = result.to_notation();
            assert_eq!(notation, "R1a3".to_string());
        }

        // Rank and file disambiguation
        {
            let board = fen::parse("3r3r/8/8/R7/4Q2Q/8/8/R6Q w - - 0 1")?;
            let mut game = Game::new(board);

            let request = MoveRequest::new(Position::h4(), Position::e1());
            let result = game.attempt_move(request).unwrap();
            let notation = result.to_notation();
            assert_eq!(notation, "Qh4e1".to_string());
        }

        Ok(())
    }
}

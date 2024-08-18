pub mod file;
pub mod position;
pub mod rank;
pub mod utils;

use crate::piece::{Piece, PieceType, Side};
use position::Position;

const BOARD_SIZE: usize = 64;
const EMPTY: BoardPosition = BoardPosition { opt_piece: None };

pub struct BoardPosition {
    opt_piece: Option<Piece>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum PositionState {
    Piece(Side),
    Empty
  }

impl BoardPosition {
    pub fn new(opt_piece: Option<Piece>) -> BoardPosition {
        BoardPosition { opt_piece }
    }

    pub fn from(piece: Piece) -> BoardPosition {
        BoardPosition {
            opt_piece: Some(piece),
        }
    }

    pub fn empty() -> BoardPosition {
        EMPTY
    }

    pub fn state(&self) -> PositionState {
        match &self.opt_piece {
            Some(piece) => PositionState::Piece(piece.side.clone()),
            None => PositionState::Empty
        }
    }

    pub fn get_piece(&self) -> &Option<Piece> {
        &self.opt_piece
    }

    pub fn set(&mut self, opt_piece: Option<Piece>) {
        self.opt_piece = opt_piece;
    }

    pub fn take_piece(&mut self) -> Option<Piece> {
        let opt_piece = self.opt_piece.clone();

        self.opt_piece = None;

        opt_piece
    }
}

pub struct Board {
    positions: [BoardPosition; BOARD_SIZE],
}

impl Board {
    pub fn empty() -> Board {
        let positions: [BoardPosition; BOARD_SIZE] = [EMPTY; BOARD_SIZE];
        Board { positions }
    }

    pub fn from(pieces: Vec<(Piece, Position)>) -> Board {
        let mut board = Board::empty();
        board.add_pieces(pieces);

        board
    }

    pub fn default() -> Board {
        let mut pieces = Vec::new();

        // White piece
        pieces.push((Piece::new(PieceType::Pawn, Side::White), Position::a2()));
        pieces.push((Piece::new(PieceType::Pawn, Side::White), Position::b2()));
        pieces.push((Piece::new(PieceType::Pawn, Side::White), Position::c2()));
        pieces.push((Piece::new(PieceType::Pawn, Side::White), Position::d2()));
        pieces.push((Piece::new(PieceType::Pawn, Side::White), Position::e2()));
        pieces.push((Piece::new(PieceType::Pawn, Side::White), Position::f2()));
        pieces.push((Piece::new(PieceType::Pawn, Side::White), Position::g2()));
        pieces.push((Piece::new(PieceType::Pawn, Side::White), Position::h2()));
        pieces.push((Piece::new(PieceType::Rook, Side::White), Position::a1()));
        pieces.push((Piece::new(PieceType::Rook, Side::White), Position::h1()));
        pieces.push((Piece::new(PieceType::Knight, Side::White), Position::b1()));
        pieces.push((Piece::new(PieceType::Knight, Side::White), Position::g1()));
        pieces.push((Piece::new(PieceType::Bishop, Side::White), Position::c1()));
        pieces.push((Piece::new(PieceType::Bishop, Side::White), Position::f1()));
        pieces.push((Piece::new(PieceType::King, Side::White), Position::e1()));
        pieces.push((Piece::new(PieceType::Queen, Side::White), Position::d1()));

        // Black pieces
        pieces.push((Piece::new(PieceType::Pawn, Side::Black), Position::a7()));
        pieces.push((Piece::new(PieceType::Pawn, Side::Black), Position::b7()));
        pieces.push((Piece::new(PieceType::Pawn, Side::Black), Position::c7()));
        pieces.push((Piece::new(PieceType::Pawn, Side::Black), Position::d7()));
        pieces.push((Piece::new(PieceType::Pawn, Side::Black), Position::e7()));
        pieces.push((Piece::new(PieceType::Pawn, Side::Black), Position::f7()));
        pieces.push((Piece::new(PieceType::Pawn, Side::Black), Position::g7()));
        pieces.push((Piece::new(PieceType::Pawn, Side::Black), Position::h7()));
        pieces.push((Piece::new(PieceType::Rook, Side::Black), Position::a8()));
        pieces.push((Piece::new(PieceType::Rook, Side::Black), Position::h8()));
        pieces.push((Piece::new(PieceType::Knight, Side::Black), Position::b8()));
        pieces.push((Piece::new(PieceType::Knight, Side::Black), Position::g8()));
        pieces.push((Piece::new(PieceType::Bishop, Side::Black), Position::c8()));
        pieces.push((Piece::new(PieceType::Bishop, Side::Black), Position::f8()));
        pieces.push((Piece::new(PieceType::King, Side::Black), Position::d8()));
        pieces.push((Piece::new(PieceType::Queen, Side::Black), Position::e8()));

        Board::from(pieces)
    }

    pub fn get_position_state(&self, position: &Position) -> PositionState {
        self.positions[position.value()].state()
    }

    pub fn add_piece(&mut self, piece: Piece, position: Position) {
        self.positions[position.value()] = BoardPosition::from(piece);
    }

    pub fn add_pieces(&mut self, pieces: Vec<(Piece, Position)>) {
        for (piece, position) in pieces {
            self.add_piece(piece, position);
        }
    }

    pub fn move_piece(&mut self, start: &Position, end: &Position) {
        if utils::valid_move(&self, start, end) {
            let start_position = &mut self.positions[start.value()];
            let opt_moving_piece = start_position.take_piece();
    
            let end_position = &mut self.positions[end.value()];
            end_position.set(opt_moving_piece);
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board_string = String::new();
        for rank in (rank::ONE..=rank::EIGHT).rev() {
            let mut rank_string = String::new();
            for file in file::A..=file::H {
                let position = Position::from_file_and_rank(file, rank);
                let piece_notation = match self.positions[position.value()].get_piece() {
                    Some(piece) => piece.to_string(),
                    None => String::from(" "),
                };

                let position_string = format!("[{piece_notation}]");
                rank_string.push_str(&position_string);
            }

            board_string.push_str(&rank_string);

            if rank != rank::ONE {
                board_string.push_str("\n");
            }
        }

        write!(f, "{board_string}")
    }
}

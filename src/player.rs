use crate::piece::{Piece, PieceType};
use crate::board::position::Position;
use crate::Side;

pub fn white_pieces() -> Vec<(Piece, Position)> {
  let mut pieces = Vec::new();

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

  pieces
}

pub fn black_pieces() -> Vec<(Piece, Position)> {
  let mut pieces = Vec::new();

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

  pieces
}
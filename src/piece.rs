use crate::Side;

#[derive(Clone)]
pub enum PieceType {
  Pawn,
  Knight,
  Bishop,
  Rook,
  Queen,
  King
}

#[derive(Clone)]
pub struct Piece {
  pub piece_type: PieceType,
  pub side: Side
}

impl Piece {
  pub fn new(piece_type: PieceType, side: Side) -> Piece {
    Piece {
      piece_type,
      side
    }
  }
}
use crate::piece::*;
use crate::Side;

#[test]
fn validate_piece_notation() {
  assert_eq!(Piece::new(PieceType::Pawn, Side::White).to_string(), "P");
  assert_eq!(Piece::new(PieceType::Pawn, Side::Black).to_string(), "p");

  assert_eq!(Piece::new(PieceType::Knight, Side::White).to_string(), "N");
  assert_eq!(Piece::new(PieceType::Knight, Side::Black).to_string(), "n");

  assert_eq!(Piece::new(PieceType::Bishop, Side::White).to_string(), "B");
  assert_eq!(Piece::new(PieceType::Bishop, Side::Black).to_string(), "b");

  assert_eq!(Piece::new(PieceType::Rook, Side::White).to_string(), "R");
  assert_eq!(Piece::new(PieceType::Rook, Side::Black).to_string(), "r");

  assert_eq!(Piece::new(PieceType::Queen, Side::White).to_string(), "Q");
  assert_eq!(Piece::new(PieceType::Queen, Side::Black).to_string(), "q");

  assert_eq!(Piece::new(PieceType::King, Side::White).to_string(), "K");
  assert_eq!(Piece::new(PieceType::King, Side::Black).to_string(), "k");
}
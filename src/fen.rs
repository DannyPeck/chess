use crate::{
    board::{file, position::Position, rank, Board, CastleRights},
    piece::{Piece, Side},
};

#[derive(Debug)]
pub struct ParseError(String);

impl ParseError {
    pub fn new(error: &str) -> ParseError {
        ParseError(String::from(error))
    }
}

pub fn parse_fen(fen: &str) -> Result<Board, ParseError> {
    let mut fen_iter = fen.split(' ');

    let piece_placement = fen_iter
        .next()
        .ok_or(ParseError::new("Missing piece placement data."))?;
    let active_color = fen_iter
        .next()
        .ok_or(ParseError::new("Missing active color data."))?;
    let castling_availability = fen_iter
        .next()
        .ok_or(ParseError::new("Missing castling availability data."))?;
    let en_passant_target_square = fen_iter
        .next()
        .ok_or(ParseError::new("Missing en passant target data."))?;
    let half_moves = fen_iter
        .next()
        .ok_or(ParseError::new("Missing half move data."))?;
    let full_moves = fen_iter
        .next()
        .ok_or(ParseError::new("Missing full move data."))?;

    let pieces = parse_piece_placement(piece_placement)?;
    let current_turn = parse_active_color(active_color)?;
    let castle_rights = parse_castling_availability(castling_availability)?;
    let en_passant_target = parse_en_passant_target(en_passant_target_square)?;
    let half_moves = parse_half_moves(half_moves)?;
    let full_moves = parse_full_moves(full_moves)?;

    let board = Board::new(
        pieces,
        current_turn,
        castle_rights,
        en_passant_target,
        half_moves,
        full_moves,
    );

    Ok(board)
}

pub fn parse_piece_placement(piece_notation: &str) -> Result<Vec<(Piece, Position)>, ParseError> {
    let mut pieces = Vec::new();

    let mut current_rank = rank::LENGTH;
    for rank_positions in piece_notation.split('/') {
        current_rank -= 1;

        let mut current_file: usize = file::A;
        for item in rank_positions.chars() {
            if item.is_ascii_digit() {
                let empty_positions = item.to_digit(10).unwrap() as usize;
                current_file += empty_positions;
            } else {
                let position = Position::from_file_and_rank(current_file, current_rank);
                if let Some(piece) = Piece::from(item) {
                    pieces.push((piece, position));
                    current_file += 1;
                } else {
                    let error = format!("Invalid piece notation found on {}", position);
                    return Err(ParseError::new(error.as_str()));
                }
            }

            // Invalid FEN notation
            if current_file > file::LENGTH {
                let error = format!(
                    "Rank {}'s notation exceeded the board length.",
                    rank::to_char(current_rank)
                );
                return Err(ParseError::new(error.as_str()));
            }
        }

        if current_file != file::LENGTH {
            let error = format!(
                "Rank {}'s notation was too short. Stopped on file {}",
                rank::to_char(current_rank),
                file::to_char(current_file)
            );
            return Err(ParseError::new(error.as_str()));
        }

        if current_rank == 0 {
            break;
        }
    }

    // We were given an insufficient number of ranks
    if current_rank != 0 {
        let error = format!(
            "Insufficient number of ranks found. Stopped on rank {}.",
            rank::to_char(current_rank)
        );
        return Err(ParseError::new(error.as_str()));
    }

    Ok(pieces)
}

pub fn parse_active_color(active_color: &str) -> Result<Side, ParseError> {
    Side::from(active_color).ok_or({
        let error = format!("Invalid active color {active_color}");
        ParseError::new(error.as_str())
    })
}

pub fn parse_castling_availability(castling_availibity: &str) -> Result<CastleRights, ParseError> {
    let mut white_short_castle_rights = false;
    let mut white_long_castle_rights = false;
    let mut black_short_castle_rights = false;
    let mut black_long_castle_rights = false;

    if castling_availibity.contains("K") {
        white_short_castle_rights = true;
    }

    if castling_availibity.contains("Q") {
        white_long_castle_rights = true;
    }

    if castling_availibity.contains("k") {
        black_short_castle_rights = true;
    }

    if castling_availibity.contains("q") {
        black_long_castle_rights = true;
    }

    let castling_rights = CastleRights {
        white_short_castle_rights,
        white_long_castle_rights,
        black_short_castle_rights,
        black_long_castle_rights,
    };

    Ok(castling_rights)
}

pub fn parse_en_passant_target(en_passant_target: &str) -> Result<Option<Position>, ParseError> {
    if en_passant_target == "-" {
        return Ok(None);
    }

    match Position::from_str(en_passant_target) {
        Some(position) => Ok(Some(position)),
        None => Err(ParseError::new("Invalid en passant target position.")),
    }
}

pub fn parse_half_moves(half_moves: &str) -> Result<u32, ParseError> {
    half_moves
        .parse()
        .map_err(|_| ParseError::new("Invalid half moves value."))
}

pub fn parse_full_moves(full_moves: &str) -> Result<u32, ParseError> {
    full_moves
        .parse()
        .map_err(|_| ParseError::new("Invalid full moves value."))
}

#[cfg(test)]
mod tests {
    use crate::piece::PieceType;

    use super::*;

    #[test]
    fn parse_fen_valid() -> Result<(), ParseError> {
        let fen = "rnbqkbn1/1p1p1pp1/7r/pBp1p2p/P2PP3/R4N2/1PP2PPP/1NBQK2R b Kq d3 0 6";

        let board = parse_fen(fen)?;

        assert_eq!(*board.get_piece(&Position::a1()), None);
        assert_eq!(
            *board.get_piece(&Position::b1()),
            Some(Piece::new(PieceType::Knight, Side::White))
        );
        assert_eq!(
            *board.get_piece(&Position::c1()),
            Some(Piece::new(PieceType::Bishop, Side::White))
        );
        assert_eq!(
            *board.get_piece(&Position::d1()),
            Some(Piece::new(PieceType::Queen, Side::White))
        );
        assert_eq!(
            *board.get_piece(&Position::e1()),
            Some(Piece::new(PieceType::King, Side::White))
        );
        assert_eq!(*board.get_piece(&Position::f1()), None);
        assert_eq!(*board.get_piece(&Position::g1()), None);
        assert_eq!(
            *board.get_piece(&Position::h1()),
            Some(Piece::new(PieceType::Rook, Side::White))
        );

        assert_eq!(*board.get_piece(&Position::a2()), None);
        assert_eq!(
            *board.get_piece(&Position::b2()),
            Some(Piece::new(PieceType::Pawn, Side::White))
        );
        assert_eq!(
            *board.get_piece(&Position::c2()),
            Some(Piece::new(PieceType::Pawn, Side::White))
        );
        assert_eq!(*board.get_piece(&Position::d2()), None);
        assert_eq!(*board.get_piece(&Position::e2()), None);
        assert_eq!(
            *board.get_piece(&Position::f2()),
            Some(Piece::new(PieceType::Pawn, Side::White))
        );
        assert_eq!(
            *board.get_piece(&Position::g2()),
            Some(Piece::new(PieceType::Pawn, Side::White))
        );
        assert_eq!(
            *board.get_piece(&Position::h2()),
            Some(Piece::new(PieceType::Pawn, Side::White))
        );

        assert_eq!(
            *board.get_piece(&Position::a3()),
            Some(Piece::new(PieceType::Rook, Side::White))
        );
        assert_eq!(*board.get_piece(&Position::b3()), None);
        assert_eq!(*board.get_piece(&Position::c3()), None);
        assert_eq!(*board.get_piece(&Position::d3()), None);
        assert_eq!(*board.get_piece(&Position::e3()), None);
        assert_eq!(
            *board.get_piece(&Position::f3()),
            Some(Piece::new(PieceType::Knight, Side::White))
        );
        assert_eq!(*board.get_piece(&Position::g3()), None);
        assert_eq!(*board.get_piece(&Position::h3()), None);

        assert_eq!(
            *board.get_piece(&Position::a4()),
            Some(Piece::new(PieceType::Pawn, Side::White))
        );
        assert_eq!(*board.get_piece(&Position::b4()), None);
        assert_eq!(*board.get_piece(&Position::c4()), None);
        assert_eq!(
            *board.get_piece(&Position::d4()),
            Some(Piece::new(PieceType::Pawn, Side::White))
        );
        assert_eq!(
            *board.get_piece(&Position::e4()),
            Some(Piece::new(PieceType::Pawn, Side::White))
        );
        assert_eq!(*board.get_piece(&Position::f4()), None);
        assert_eq!(*board.get_piece(&Position::g4()), None);
        assert_eq!(*board.get_piece(&Position::h4()), None);

        assert_eq!(
            *board.get_piece(&Position::a5()),
            Some(Piece::new(PieceType::Pawn, Side::Black))
        );
        assert_eq!(
            *board.get_piece(&Position::b5()),
            Some(Piece::new(PieceType::Bishop, Side::White))
        );
        assert_eq!(
            *board.get_piece(&Position::c5()),
            Some(Piece::new(PieceType::Pawn, Side::Black))
        );
        assert_eq!(*board.get_piece(&Position::d5()), None);
        assert_eq!(
            *board.get_piece(&Position::e5()),
            Some(Piece::new(PieceType::Pawn, Side::Black))
        );
        assert_eq!(*board.get_piece(&Position::f5()), None);
        assert_eq!(*board.get_piece(&Position::g5()), None);
        assert_eq!(
            *board.get_piece(&Position::h5()),
            Some(Piece::new(PieceType::Pawn, Side::Black))
        );

        assert_eq!(*board.get_piece(&Position::a6()), None);
        assert_eq!(*board.get_piece(&Position::b6()), None);
        assert_eq!(*board.get_piece(&Position::c6()), None);
        assert_eq!(*board.get_piece(&Position::d6()), None);
        assert_eq!(*board.get_piece(&Position::e6()), None);
        assert_eq!(*board.get_piece(&Position::f6()), None);
        assert_eq!(*board.get_piece(&Position::g6()), None);
        assert_eq!(
            *board.get_piece(&Position::h6()),
            Some(Piece::new(PieceType::Rook, Side::Black))
        );

        assert_eq!(*board.get_piece(&Position::a7()), None);
        assert_eq!(
            *board.get_piece(&Position::b7()),
            Some(Piece::new(PieceType::Pawn, Side::Black))
        );
        assert_eq!(*board.get_piece(&Position::c7()), None);
        assert_eq!(
            *board.get_piece(&Position::d7()),
            Some(Piece::new(PieceType::Pawn, Side::Black))
        );
        assert_eq!(*board.get_piece(&Position::e7()), None);
        assert_eq!(
            *board.get_piece(&Position::f7()),
            Some(Piece::new(PieceType::Pawn, Side::Black))
        );
        assert_eq!(
            *board.get_piece(&Position::g7()),
            Some(Piece::new(PieceType::Pawn, Side::Black))
        );
        assert_eq!(*board.get_piece(&Position::h7()), None);

        assert_eq!(
            *board.get_piece(&Position::a8()),
            Some(Piece::new(PieceType::Rook, Side::Black))
        );
        assert_eq!(
            *board.get_piece(&Position::b8()),
            Some(Piece::new(PieceType::Knight, Side::Black))
        );
        assert_eq!(
            *board.get_piece(&Position::c8()),
            Some(Piece::new(PieceType::Bishop, Side::Black))
        );
        assert_eq!(
            *board.get_piece(&Position::d8()),
            Some(Piece::new(PieceType::Queen, Side::Black))
        );
        assert_eq!(
            *board.get_piece(&Position::e8()),
            Some(Piece::new(PieceType::King, Side::Black))
        );
        assert_eq!(
            *board.get_piece(&Position::f8()),
            Some(Piece::new(PieceType::Bishop, Side::Black))
        );
        assert_eq!(
            *board.get_piece(&Position::g8()),
            Some(Piece::new(PieceType::Knight, Side::Black))
        );
        assert_eq!(*board.get_piece(&Position::h8()), None);

        assert_eq!(board.get_current_turn(), Side::Black);

        assert_eq!(
            *board.get_castle_rights(),
            CastleRights::new(true, false, false, true)
        );

        assert_eq!(*board.get_en_passant_target(), Some(Position::d3()));

        assert_eq!(board.get_half_moves(), 0);

        assert_eq!(board.get_full_moves(), 6);

        Ok(())
    }

    #[test]
    fn parse_fen_invalid() -> Result<(), ParseError> {
        // Missing full moves
        assert!(
            parse_fen("rnbqkbn1/1p1p1pp1/7r/pBp1p2p/P2PP3/R4N2/1PP2PPP/1NBQK2R b Kq d3 0").is_err()
        );

        // Missing half moves
        assert!(
            parse_fen("rnbqkbn1/1p1p1pp1/7r/pBp1p2p/P2PP3/R4N2/1PP2PPP/1NBQK2R b Kq d3").is_err()
        );

        // Missing en passant target
        assert!(parse_fen("rnbqkbn1/1p1p1pp1/7r/pBp1p2p/P2PP3/R4N2/1PP2PPP/1NBQK2R b Kq").is_err());

        // Missing castling availability
        assert!(parse_fen("rnbqkbn1/1p1p1pp1/7r/pBp1p2p/P2PP3/R4N2/1PP2PPP/1NBQK2R b").is_err());

        // Missing active color
        assert!(parse_fen("rnbqkbn1/1p1p1pp1/7r/pBp1p2p/P2PP3/R4N2/1PP2PPP/1NBQK2R").is_err());

        // Empty
        assert!(parse_fen("").is_err());

        Ok(())
    }

    #[test]
    fn parse_piece_notation_valid() -> Result<(), ParseError> {
        let pieces =
            parse_piece_placement("rnbqkbn1/1p1p1pp1/7r/pBp1p2p/P2PP3/R4N2/1PP2PPP/1NBQK2R")?;

        let mut board = Board::empty();

        board.add_pieces(pieces);

        assert_eq!(*board.get_piece(&Position::a1()), None);
        assert_eq!(
            *board.get_piece(&Position::b1()),
            Some(Piece::new(PieceType::Knight, Side::White))
        );
        assert_eq!(
            *board.get_piece(&Position::c1()),
            Some(Piece::new(PieceType::Bishop, Side::White))
        );
        assert_eq!(
            *board.get_piece(&Position::d1()),
            Some(Piece::new(PieceType::Queen, Side::White))
        );
        assert_eq!(
            *board.get_piece(&Position::e1()),
            Some(Piece::new(PieceType::King, Side::White))
        );
        assert_eq!(*board.get_piece(&Position::f1()), None);
        assert_eq!(*board.get_piece(&Position::g1()), None);
        assert_eq!(
            *board.get_piece(&Position::h1()),
            Some(Piece::new(PieceType::Rook, Side::White))
        );

        assert_eq!(*board.get_piece(&Position::a2()), None);
        assert_eq!(
            *board.get_piece(&Position::b2()),
            Some(Piece::new(PieceType::Pawn, Side::White))
        );
        assert_eq!(
            *board.get_piece(&Position::c2()),
            Some(Piece::new(PieceType::Pawn, Side::White))
        );
        assert_eq!(*board.get_piece(&Position::d2()), None);
        assert_eq!(*board.get_piece(&Position::e2()), None);
        assert_eq!(
            *board.get_piece(&Position::f2()),
            Some(Piece::new(PieceType::Pawn, Side::White))
        );
        assert_eq!(
            *board.get_piece(&Position::g2()),
            Some(Piece::new(PieceType::Pawn, Side::White))
        );
        assert_eq!(
            *board.get_piece(&Position::h2()),
            Some(Piece::new(PieceType::Pawn, Side::White))
        );

        assert_eq!(
            *board.get_piece(&Position::a3()),
            Some(Piece::new(PieceType::Rook, Side::White))
        );
        assert_eq!(*board.get_piece(&Position::b3()), None);
        assert_eq!(*board.get_piece(&Position::c3()), None);
        assert_eq!(*board.get_piece(&Position::d3()), None);
        assert_eq!(*board.get_piece(&Position::e3()), None);
        assert_eq!(
            *board.get_piece(&Position::f3()),
            Some(Piece::new(PieceType::Knight, Side::White))
        );
        assert_eq!(*board.get_piece(&Position::g3()), None);
        assert_eq!(*board.get_piece(&Position::h3()), None);

        assert_eq!(
            *board.get_piece(&Position::a4()),
            Some(Piece::new(PieceType::Pawn, Side::White))
        );
        assert_eq!(*board.get_piece(&Position::b4()), None);
        assert_eq!(*board.get_piece(&Position::c4()), None);
        assert_eq!(
            *board.get_piece(&Position::d4()),
            Some(Piece::new(PieceType::Pawn, Side::White))
        );
        assert_eq!(
            *board.get_piece(&Position::e4()),
            Some(Piece::new(PieceType::Pawn, Side::White))
        );
        assert_eq!(*board.get_piece(&Position::f4()), None);
        assert_eq!(*board.get_piece(&Position::g4()), None);
        assert_eq!(*board.get_piece(&Position::h4()), None);

        assert_eq!(
            *board.get_piece(&Position::a5()),
            Some(Piece::new(PieceType::Pawn, Side::Black))
        );
        assert_eq!(
            *board.get_piece(&Position::b5()),
            Some(Piece::new(PieceType::Bishop, Side::White))
        );
        assert_eq!(
            *board.get_piece(&Position::c5()),
            Some(Piece::new(PieceType::Pawn, Side::Black))
        );
        assert_eq!(*board.get_piece(&Position::d5()), None);
        assert_eq!(
            *board.get_piece(&Position::e5()),
            Some(Piece::new(PieceType::Pawn, Side::Black))
        );
        assert_eq!(*board.get_piece(&Position::f5()), None);
        assert_eq!(*board.get_piece(&Position::g5()), None);
        assert_eq!(
            *board.get_piece(&Position::h5()),
            Some(Piece::new(PieceType::Pawn, Side::Black))
        );

        assert_eq!(*board.get_piece(&Position::a6()), None);
        assert_eq!(*board.get_piece(&Position::b6()), None);
        assert_eq!(*board.get_piece(&Position::c6()), None);
        assert_eq!(*board.get_piece(&Position::d6()), None);
        assert_eq!(*board.get_piece(&Position::e6()), None);
        assert_eq!(*board.get_piece(&Position::f6()), None);
        assert_eq!(*board.get_piece(&Position::g6()), None);
        assert_eq!(
            *board.get_piece(&Position::h6()),
            Some(Piece::new(PieceType::Rook, Side::Black))
        );

        assert_eq!(*board.get_piece(&Position::a7()), None);
        assert_eq!(
            *board.get_piece(&Position::b7()),
            Some(Piece::new(PieceType::Pawn, Side::Black))
        );
        assert_eq!(*board.get_piece(&Position::c7()), None);
        assert_eq!(
            *board.get_piece(&Position::d7()),
            Some(Piece::new(PieceType::Pawn, Side::Black))
        );
        assert_eq!(*board.get_piece(&Position::e7()), None);
        assert_eq!(
            *board.get_piece(&Position::f7()),
            Some(Piece::new(PieceType::Pawn, Side::Black))
        );
        assert_eq!(
            *board.get_piece(&Position::g7()),
            Some(Piece::new(PieceType::Pawn, Side::Black))
        );
        assert_eq!(*board.get_piece(&Position::h7()), None);

        assert_eq!(
            *board.get_piece(&Position::a8()),
            Some(Piece::new(PieceType::Rook, Side::Black))
        );
        assert_eq!(
            *board.get_piece(&Position::b8()),
            Some(Piece::new(PieceType::Knight, Side::Black))
        );
        assert_eq!(
            *board.get_piece(&Position::c8()),
            Some(Piece::new(PieceType::Bishop, Side::Black))
        );
        assert_eq!(
            *board.get_piece(&Position::d8()),
            Some(Piece::new(PieceType::Queen, Side::Black))
        );
        assert_eq!(
            *board.get_piece(&Position::e8()),
            Some(Piece::new(PieceType::King, Side::Black))
        );
        assert_eq!(
            *board.get_piece(&Position::f8()),
            Some(Piece::new(PieceType::Bishop, Side::Black))
        );
        assert_eq!(
            *board.get_piece(&Position::g8()),
            Some(Piece::new(PieceType::Knight, Side::Black))
        );
        assert_eq!(*board.get_piece(&Position::h8()), None);

        Ok(())
    }

    #[test]
    fn parse_piece_notation_invalid() {
        // Removed a piece from the 8th rank
        assert!(
            parse_piece_placement("nbqkbn1/1p1p1pp1/7r/pBp1p2p/P2PP3/R4N2/1PP2PPP/1NBQK2R")
                .is_err()
        );

        // Rank exceeds board length
        assert!(
            parse_piece_placement("rnbq5/1p1p1pp1/7r/pBp1p2p/P2PP3/R4N2/1PP2PPP/1NBQK2R").is_err()
        );

        // Insufficient number of ranks
        assert!(parse_piece_placement("rnbqkbn1/1p1p1pp1/7r/pBp1p2p/P2PP3/R4N2/1PP2PPP").is_err());

        // Invalid piece notation
        assert!(
            parse_piece_placement("Xnbqkbn1/1p1p1pp1/7r/pBp1p2p/P2PP3/R4N2/1PP2PPP/1NBQK2R")
                .is_err()
        );
    }

    #[test]
    fn parse_active_color_test() -> Result<(), ParseError> {
        let white = parse_active_color("w")?;
        assert_eq!(white, Side::White);

        let black = parse_active_color("b")?;
        assert_eq!(black, Side::Black);

        assert!(parse_active_color("X").is_err());

        Ok(())
    }

    #[test]
    fn parse_castling_availability_test() -> Result<(), ParseError> {
        // All combinations
        assert_eq!(
            parse_castling_availability("KQkq")?,
            CastleRights::new(true, true, true, true)
        );
        assert_eq!(
            parse_castling_availability("KQk")?,
            CastleRights::new(true, true, true, false)
        );
        assert_eq!(
            parse_castling_availability("KQq")?,
            CastleRights::new(true, true, false, true)
        );
        assert_eq!(
            parse_castling_availability("Kkq")?,
            CastleRights::new(true, false, true, true)
        );
        assert_eq!(
            parse_castling_availability("Qkq")?,
            CastleRights::new(false, true, true, true)
        );
        assert_eq!(
            parse_castling_availability("KQ")?,
            CastleRights::new(true, true, false, false)
        );
        assert_eq!(
            parse_castling_availability("Kq")?,
            CastleRights::new(true, false, false, true)
        );
        assert_eq!(
            parse_castling_availability("Kk")?,
            CastleRights::new(true, false, true, false)
        );
        assert_eq!(
            parse_castling_availability("kq")?,
            CastleRights::new(false, false, true, true)
        );
        assert_eq!(
            parse_castling_availability("Qk")?,
            CastleRights::new(false, true, true, false)
        );
        assert_eq!(
            parse_castling_availability("Qq")?,
            CastleRights::new(false, true, false, true)
        );
        assert_eq!(
            parse_castling_availability("K")?,
            CastleRights::new(true, false, false, false)
        );
        assert_eq!(
            parse_castling_availability("k")?,
            CastleRights::new(false, false, true, false)
        );
        assert_eq!(
            parse_castling_availability("Q")?,
            CastleRights::new(false, true, false, false)
        );
        assert_eq!(
            parse_castling_availability("q")?,
            CastleRights::new(false, false, false, true)
        );

        // None
        assert_eq!(
            parse_castling_availability("-")?,
            CastleRights::new(false, false, false, false)
        );

        // Different order is valid
        assert_eq!(
            parse_castling_availability("qkQK")?,
            CastleRights::new(true, true, true, true)
        );

        Ok(())
    }

    #[test]
    fn parse_en_passant_target_test() -> Result<(), ParseError> {
        assert_eq!(parse_en_passant_target("d3")?, Some(Position::d3()));
        assert_eq!(parse_en_passant_target("-")?, None);

        assert!(parse_en_passant_target("a9").is_err());

        Ok(())
    }

    #[test]
    fn parse_half_moves_test() -> Result<(), ParseError> {
        assert_eq!(parse_half_moves("0")?, 0);
        assert_eq!(parse_half_moves("1")?, 1);
        assert_eq!(parse_half_moves("13")?, 13);

        assert!(parse_half_moves("X").is_err());

        Ok(())
    }

    #[test]
    fn parse_full_moves_test() -> Result<(), ParseError> {
        assert_eq!(parse_full_moves("0")?, 0);
        assert_eq!(parse_full_moves("1")?, 1);
        assert_eq!(parse_full_moves("13")?, 13);

        assert!(parse_full_moves("X").is_err());

        Ok(())
    }
}

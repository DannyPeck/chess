use crate::{
    board::{file, position::Position, rank, Board, CastleRights},
    piece::{Piece, Side},
};

use anyhow::{anyhow, bail};

pub fn parse(fen: &str) -> anyhow::Result<Board> {
    let mut fen_iter = fen.split(' ');

    let piece_placement = fen_iter
        .next()
        .ok_or(anyhow!("Missing piece placement data."))?;
    let active_color = fen_iter
        .next()
        .ok_or(anyhow!("Missing active color data."))?;
    let castling_availability = fen_iter
        .next()
        .ok_or(anyhow!("Missing castling availability data."))?;
    let en_passant_target_square = fen_iter
        .next()
        .ok_or(anyhow!("Missing en passant target data."))?;
    let half_moves = fen_iter.next().ok_or(anyhow!("Missing half move data."))?;
    let full_moves = fen_iter.next().ok_or(anyhow!("Missing full move data."))?;

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

pub fn parse_piece_placement(piece_notation: &str) -> anyhow::Result<Vec<(Position, Piece)>> {
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
                    pieces.push((position, piece));
                    current_file += 1;
                } else {
                    bail!("Invalid piece notation found on {position}");
                }
            }

            // Invalid FEN notation
            if current_file > file::LENGTH {
                let error = format!(
                    "Rank {}'s notation exceeded the board length.",
                    rank::to_char(current_rank)
                );
                bail!(error);
            }
        }

        if current_file != file::LENGTH {
            let error = format!(
                "Rank {}'s notation was too short. Stopped on file {}",
                rank::to_char(current_rank),
                file::to_char(current_file)
            );
            bail!(error);
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
        bail!(error);
    }

    Ok(pieces)
}

pub fn parse_active_color(active_color: &str) -> anyhow::Result<Side> {
    Side::from(active_color).ok_or(anyhow!("Invalid active color {active_color}"))
}

pub fn parse_castling_availability(castling_availibity: &str) -> anyhow::Result<CastleRights> {
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

pub fn parse_en_passant_target(en_passant_target: &str) -> anyhow::Result<Option<Position>> {
    if en_passant_target == "-" {
        return Ok(None);
    }

    match Position::from_notation(en_passant_target) {
        Some(position) => Ok(Some(position)),
        None => Err(anyhow!("Invalid en passant target position.")),
    }
}

pub fn parse_half_moves(half_moves: &str) -> anyhow::Result<u32> {
    half_moves
        .parse()
        .map_err(|_| anyhow!("Invalid half moves value."))
}

pub fn parse_full_moves(full_moves: &str) -> anyhow::Result<u32> {
    full_moves
        .parse()
        .map_err(|_| anyhow!("Invalid full moves value."))
}

#[cfg(test)]
mod tests {
    use crate::{board_position, piece::PieceType};

    use super::*;

    #[test]
    fn parse_valid() -> anyhow::Result<()> {
        let board = parse("rnbqkbn1/1p1p1pp1/7r/pBp1p2p/P2PP3/R4N2/1PP2PPP/1NBQK2R b Kq d3 0 6")?;

        let position_tests: Vec<(Position, Option<Piece>)> = vec![
            board_position!(a1, None),
            board_position!(b1, Knight, White),
            board_position!(c1, Bishop, White),
            board_position!(d1, Queen, White),
            board_position!(e1, King, White),
            board_position!(f1, None),
            board_position!(g1, None),
            board_position!(h1, Rook, White),
            board_position!(a2, None),
            board_position!(b2, Pawn, White),
            board_position!(c2, Pawn, White),
            board_position!(d2, None),
            board_position!(e2, None),
            board_position!(f2, Pawn, White),
            board_position!(g2, Pawn, White),
            board_position!(h2, Pawn, White),
            board_position!(a3, Rook, White),
            board_position!(b3, None),
            board_position!(c3, None),
            board_position!(d3, None),
            board_position!(e3, None),
            board_position!(f3, Knight, White),
            board_position!(g3, None),
            board_position!(h3, None),
            board_position!(a4, Pawn, White),
            board_position!(b4, None),
            board_position!(c4, None),
            board_position!(d4, Pawn, White),
            board_position!(e4, Pawn, White),
            board_position!(f4, None),
            board_position!(g4, None),
            board_position!(h4, None),
            board_position!(a5, Pawn, Black),
            board_position!(b5, Bishop, White),
            board_position!(c5, Pawn, Black),
            board_position!(d5, None),
            board_position!(e5, Pawn, Black),
            board_position!(f5, None),
            board_position!(g5, None),
            board_position!(h5, Pawn, Black),
            board_position!(a6, None),
            board_position!(b6, None),
            board_position!(c6, None),
            board_position!(d6, None),
            board_position!(e6, None),
            board_position!(f6, None),
            board_position!(g6, None),
            board_position!(h6, Rook, Black),
            board_position!(a7, None),
            board_position!(b7, Pawn, Black),
            board_position!(c7, None),
            board_position!(d7, Pawn, Black),
            board_position!(e7, None),
            board_position!(f7, Pawn, Black),
            board_position!(g7, Pawn, Black),
            board_position!(h7, None),
            board_position!(a8, Rook, Black),
            board_position!(b8, Knight, Black),
            board_position!(c8, Bishop, Black),
            board_position!(d8, Queen, Black),
            board_position!(e8, King, Black),
            board_position!(f8, Bishop, Black),
            board_position!(g8, Knight, Black),
            board_position!(h8, None),
        ];

        for (position, piece) in position_tests {
            assert_eq!(board.get_piece(&position), piece.as_ref());
        }

        assert_eq!(*board.get_current_turn(), Side::Black);

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
    fn parse_invalid() -> anyhow::Result<()> {
        // Missing full moves
        assert!(
            parse("rnbqkbn1/1p1p1pp1/7r/pBp1p2p/P2PP3/R4N2/1PP2PPP/1NBQK2R b Kq d3 0").is_err()
        );

        // Missing half moves
        assert!(parse("rnbqkbn1/1p1p1pp1/7r/pBp1p2p/P2PP3/R4N2/1PP2PPP/1NBQK2R b Kq d3").is_err());

        // Missing en passant target
        assert!(parse("rnbqkbn1/1p1p1pp1/7r/pBp1p2p/P2PP3/R4N2/1PP2PPP/1NBQK2R b Kq").is_err());

        // Missing castling availability
        assert!(parse("rnbqkbn1/1p1p1pp1/7r/pBp1p2p/P2PP3/R4N2/1PP2PPP/1NBQK2R b").is_err());

        // Missing active color
        assert!(parse("rnbqkbn1/1p1p1pp1/7r/pBp1p2p/P2PP3/R4N2/1PP2PPP/1NBQK2R").is_err());

        // Empty
        assert!(parse("").is_err());

        Ok(())
    }

    #[test]
    fn parse_piece_notation_valid() -> anyhow::Result<()> {
        let pieces =
            parse_piece_placement("rnbqkbn1/1p1p1pp1/7r/pBp1p2p/P2PP3/R4N2/1PP2PPP/1NBQK2R")?;

        let mut board = Board::empty();

        board.add_pieces(pieces);

        let position_tests: Vec<(Position, Option<Piece>)> = vec![
            board_position!(a1, None),
            board_position!(b1, Knight, White),
            board_position!(c1, Bishop, White),
            board_position!(d1, Queen, White),
            board_position!(e1, King, White),
            board_position!(f1, None),
            board_position!(g1, None),
            board_position!(h1, Rook, White),
            board_position!(a2, None),
            board_position!(b2, Pawn, White),
            board_position!(c2, Pawn, White),
            board_position!(d2, None),
            board_position!(e2, None),
            board_position!(f2, Pawn, White),
            board_position!(g2, Pawn, White),
            board_position!(h2, Pawn, White),
            board_position!(a3, Rook, White),
            board_position!(b3, None),
            board_position!(c3, None),
            board_position!(d3, None),
            board_position!(e3, None),
            board_position!(f3, Knight, White),
            board_position!(g3, None),
            board_position!(h3, None),
            board_position!(a4, Pawn, White),
            board_position!(b4, None),
            board_position!(c4, None),
            board_position!(d4, Pawn, White),
            board_position!(e4, Pawn, White),
            board_position!(f4, None),
            board_position!(g4, None),
            board_position!(h4, None),
            board_position!(a5, Pawn, Black),
            board_position!(b5, Bishop, White),
            board_position!(c5, Pawn, Black),
            board_position!(d5, None),
            board_position!(e5, Pawn, Black),
            board_position!(f5, None),
            board_position!(g5, None),
            board_position!(h5, Pawn, Black),
            board_position!(a6, None),
            board_position!(b6, None),
            board_position!(c6, None),
            board_position!(d6, None),
            board_position!(e6, None),
            board_position!(f6, None),
            board_position!(g6, None),
            board_position!(h6, Rook, Black),
            board_position!(a7, None),
            board_position!(b7, Pawn, Black),
            board_position!(c7, None),
            board_position!(d7, Pawn, Black),
            board_position!(e7, None),
            board_position!(f7, Pawn, Black),
            board_position!(g7, Pawn, Black),
            board_position!(h7, None),
            board_position!(a8, Rook, Black),
            board_position!(b8, Knight, Black),
            board_position!(c8, Bishop, Black),
            board_position!(d8, Queen, Black),
            board_position!(e8, King, Black),
            board_position!(f8, Bishop, Black),
            board_position!(g8, Knight, Black),
            board_position!(h8, None),
        ];

        for (position, piece) in position_tests {
            assert_eq!(board.get_piece(&position), piece.as_ref());
        }

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
    fn parse_active_color_test() -> anyhow::Result<()> {
        let white = parse_active_color("w")?;
        assert_eq!(white, Side::White);

        let black = parse_active_color("b")?;
        assert_eq!(black, Side::Black);

        assert!(parse_active_color("X").is_err());

        Ok(())
    }

    #[test]
    fn parse_castling_availability_test() -> anyhow::Result<()> {
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
    fn parse_en_passant_target_test() -> anyhow::Result<()> {
        assert_eq!(parse_en_passant_target("d3")?, Some(Position::d3()));
        assert_eq!(parse_en_passant_target("-")?, None);

        assert!(parse_en_passant_target("a9").is_err());

        Ok(())
    }

    #[test]
    fn parse_half_moves_test() -> anyhow::Result<()> {
        assert_eq!(parse_half_moves("0")?, 0);
        assert_eq!(parse_half_moves("1")?, 1);
        assert_eq!(parse_half_moves("13")?, 13);

        assert!(parse_half_moves("X").is_err());

        Ok(())
    }

    #[test]
    fn parse_full_moves_test() -> anyhow::Result<()> {
        assert_eq!(parse_full_moves("0")?, 0);
        assert_eq!(parse_full_moves("1")?, 1);
        assert_eq!(parse_full_moves("13")?, 13);

        assert!(parse_full_moves("X").is_err());

        Ok(())
    }
}

use crate::{
    board::{file, position::Position, rank, Board, CastleRights},
    piece::Side,
};

pub fn generate(board: &Board) -> String {
    let piece_placement = generate_piece_placement(board);
    let active_color = generate_active_color(board.get_current_turn());
    let castling_availability = generate_castling_availability(board.get_castle_rights());
    let en_passant_target = generate_en_passant_target(board.get_en_passant_target());
    let half_moves = generate_half_moves(board.get_half_moves());
    let full_moves = generate_full_moves(board.get_full_moves());

    format!("{piece_placement} {active_color} {castling_availability} {en_passant_target} {half_moves} {full_moves}")
}

pub fn generate_piece_placement(board: &Board) -> String {
    let mut piece_placement = String::new();
    for current_rank in (rank::ONE..=rank::EIGHT).rev() {
        let mut rank_string = String::new();
        let mut current_empty_count = 0;
        for current_file in file::A..=file::H {
            let position = Position::from_file_and_rank(current_file, current_rank);
            match board.get_piece(&position) {
                Some(piece) => {
                    if current_empty_count > 0 {
                        rank_string.push_str(&current_empty_count.to_string());
                        current_empty_count = 0;
                    }

                    rank_string.push_str(&piece.to_string());
                }
                None => {
                    current_empty_count += 1;
                }
            }
        }

        if current_empty_count > 0 {
            rank_string.push_str(&current_empty_count.to_string());
        }

        piece_placement.push_str(&rank_string);

        if current_rank != rank::ONE {
            piece_placement.push('/');
        }
    }

    piece_placement
}

pub fn generate_active_color(side: &Side) -> String {
    side.to_string()
}

pub fn generate_castling_availability(castle_rights: &CastleRights) -> String {
    let mut castling_availability = String::new();

    if castle_rights.white_short_castle_rights {
        castling_availability.push('K');
    }

    if castle_rights.white_long_castle_rights {
        castling_availability.push('Q');
    }

    if castle_rights.black_short_castle_rights {
        castling_availability.push('k');
    }

    if castle_rights.black_long_castle_rights {
        castling_availability.push('q');
    }

    if castling_availability.is_empty() {
        castling_availability.push('-');
    }

    castling_availability
}

pub fn generate_en_passant_target(target: &Option<Position>) -> String {
    match target {
        Some(piece) => piece.to_string(),
        None => String::from("-"),
    }
}

pub fn generate_half_moves(half_moves: u32) -> String {
    half_moves.to_string()
}

pub fn generate_full_moves(full_moves: u32) -> String {
    full_moves.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{fen, ParseError};

    #[test]
    fn generate_test() -> Result<(), ParseError> {
        assert_eq!(
            generate(&Board::default()),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );

        let custom_fen = "rnbqkbn1/1p1p1pp1/7r/pBp1p2p/P2PP3/R4N2/1PP2PPP/1NBQK2R b Kq d3 0 6";
        let custom_board = fen::parse(custom_fen)?;
        let generated_fen = generate(&custom_board);
        assert_eq!(generated_fen, custom_fen);

        Ok(())
    }

    #[test]
    fn generate_piece_placement_test() -> Result<(), ParseError> {
        assert_eq!(
            generate_piece_placement(&Board::default()),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
        );

        let custom_fen = "rnbqkbn1/1p1p1pp1/7r/pBp1p2p/P2PP3/R4N2/1PP2PPP/1NBQK2R b Kq d3 0 6";
        let custom_board = fen::parse(custom_fen)?;

        let expected_piece_placement = "rnbqkbn1/1p1p1pp1/7r/pBp1p2p/P2PP3/R4N2/1PP2PPP/1NBQK2R";
        let generated_piece_placement = generate_piece_placement(&custom_board);
        assert_eq!(generated_piece_placement, expected_piece_placement);

        Ok(())
    }

    #[test]
    fn generate_active_color_test() {
        assert_eq!(generate_active_color(&Side::White), "w");
        assert_eq!(generate_active_color(&Side::Black), "b");
    }

    #[test]
    fn generate_castling_availability_test() {
        assert_eq!(
            "KQkq",
            generate_castling_availability(&CastleRights::new(true, true, true, true))
        );
        assert_eq!(
            "KQk",
            generate_castling_availability(&CastleRights::new(true, true, true, false))
        );
        assert_eq!(
            "KQq",
            generate_castling_availability(&CastleRights::new(true, true, false, true))
        );
        assert_eq!(
            "Kkq",
            generate_castling_availability(&CastleRights::new(true, false, true, true))
        );
        assert_eq!(
            "Qkq",
            generate_castling_availability(&CastleRights::new(false, true, true, true))
        );
        assert_eq!(
            "KQ",
            generate_castling_availability(&CastleRights::new(true, true, false, false))
        );
        assert_eq!(
            "Kq",
            generate_castling_availability(&CastleRights::new(true, false, false, true))
        );
        assert_eq!(
            "Kk",
            generate_castling_availability(&CastleRights::new(true, false, true, false))
        );
        assert_eq!(
            "kq",
            generate_castling_availability(&CastleRights::new(false, false, true, true))
        );
        assert_eq!(
            "Qk",
            generate_castling_availability(&CastleRights::new(false, true, true, false))
        );
        assert_eq!(
            "Qq",
            generate_castling_availability(&CastleRights::new(false, true, false, true))
        );
        assert_eq!(
            "K",
            generate_castling_availability(&CastleRights::new(true, false, false, false))
        );
        assert_eq!(
            "k",
            generate_castling_availability(&CastleRights::new(false, false, true, false))
        );
        assert_eq!(
            "Q",
            generate_castling_availability(&CastleRights::new(false, true, false, false))
        );
        assert_eq!(
            "q",
            generate_castling_availability(&CastleRights::new(false, false, false, true))
        );
        assert_eq!(
            "-",
            generate_castling_availability(&CastleRights::new(false, false, false, false))
        );
    }

    #[test]
    fn generate_en_passant_target_test() {
        assert_eq!(generate_en_passant_target(&Some(Position::d3())), "d3");
        assert_eq!(generate_en_passant_target(&None), "-");
    }

    #[test]
    fn generate_half_moves_test() {
        assert_eq!(generate_half_moves(0), "0");
        assert_eq!(generate_half_moves(1), "1");
        assert_eq!(generate_half_moves(13), "13");
    }

    #[test]
    fn generate_full_moves_test() {
        assert_eq!(generate_full_moves(0), "0");
        assert_eq!(generate_full_moves(1), "1");
        assert_eq!(generate_full_moves(13), "13");
    }
}

use crate::{
    board::{file, position::Position, rank, Board, CastleRights},
    piece::{Piece, Side},
};

#[derive(Debug)]
pub struct ParseError {
    error: String,
}

impl ParseError {
    pub fn new(error: &str) -> ParseError {
        ParseError {
            error: String::from(error),
        }
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
    let en_passant_target: Option<Position> = parse_en_passant_target(en_passant_target_square)?;
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
    use super::*;

    #[test]
    fn parse_fen_test() -> Result<(), ParseError> {
        let fen = "rnbqkbn1/1p1p1pp1/7r/pBp1p2p/P2PP3/R4N2/1PP2PPP/1NBQK2R b Kq d3 0 6";

        let board = parse_fen(fen)?;

        println!("{board:#?}");

        println!("{board}");

        Ok(())
    }

    #[test]
    fn piece_notation_test() -> Result<(), ParseError> {
        let _pieces = parse_piece_placement("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R")?;

        Ok(())
    }
}

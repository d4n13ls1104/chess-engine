use super::{
    piece::{Piece, Color},
    Position
};

use std::fmt;
use std::error::Error;

pub const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Clone)]
pub struct CastleFlags {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl CastleFlags {
    pub fn parse_castle_flags(castle_flags: &str) -> Self {
        let mut white_kingside = false;
        let mut white_queenside = false;
        let mut black_kingside = false;
        let mut black_queenside = false;

        for c in castle_flags.chars() {
            match c {
                'K' => white_kingside = true,
                'Q' => white_queenside = true,
                'k' => black_kingside = true,
                'q' => black_queenside = true,
                _ => ()
            }
        }

        Self {
            white_kingside,
            white_queenside,
            black_kingside,
            black_queenside
        }
    }
}

pub fn parse_squares(ranks: &str) -> [[Option<Piece>; 8]; 8] {
    let mut squares = [[None; 8]; 8];
    let ranks: Vec<&str> = ranks.split('/').collect();

    for (r_index, rank) in ranks.iter().enumerate() {
        let mut f_index = 0;

        for c in rank.chars() {
            if c.is_numeric() {
                f_index += c.to_digit(10).unwrap() as usize;
            } else {
                squares[r_index][f_index] = Some(Piece::from_char(c));
                f_index += 1;
            }
        }
    }
    squares
}

pub fn parse_en_passant_target_square(en_passant_target_square: &str) -> Result<Option<Position>, ParseFenError> {
    if en_passant_target_square != "-" {
        let rank = en_passant_target_square.chars().nth(1);
        let file = en_passant_target_square.chars().next();

        let (rank, file) = match (rank, file) {
            (Some(rank), Some(file)) => (rank, file),
            _ => { return Err(ParseFenError::ParseEnPassantSquare(en_passant_target_square.to_string())); }
        };

        let rank = 7 - (rank as usize - 49);
        let file = file as usize - 97;

        return Ok(Some((rank, file)));
    }
    Ok(None)
}

pub fn parse_active_turn(active_turn: &str) -> Result<Color, ParseFenError> {
    match active_turn {
        "w" => Ok(Color::White),
        "b" => Ok(Color::Black),
        _ => Err(ParseFenError::ParseActiveTurn(active_turn.to_string()))
    }
}

#[derive(Debug)]
pub enum ParseFenError {
    ParseActiveTurn(String),
    ParseEnPassantSquare(String),
    MissingKing,
}

impl fmt::Display for ParseFenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ParseActiveTurn(active_turn) => {
                write!(f, "Failed to parse active_turn: {active_turn}")
            },
            
            Self::ParseEnPassantSquare(en_passant_target_square) => {
                write!(f, "Failed to parse en_passant_target_square: {en_passant_target_square}")
            },

            Self::MissingKing => {
                write!(f, "Position is invalid because one or both kings are missing")
            }
        }
    }
}

impl Error for ParseFenError {}

use super::{board::Board, castle_flags::CastleFlags, color::Color};

pub struct Position {
    board: Board,
    active_turn: Color,
    castle_flags: CastleFlags,
    en_passant_square: u8,
    halfmove_clock: u8,
    fullmove_number: u16,
}

impl Position {
    pub fn empty() -> Self {
        Self {
            board: Board::empty(),
            active_turn: Color::White,
            castle_flags: CastleFlags::parse_castle_flags("-"),
            en_passant_square: 0,
            halfmove_clock: 0,
            fullmove_number: 0,
        }
    }
    pub fn from_fen(fen: &str) -> Self {
        Self {
            board: Board::from_fen(fen),
            active_turn: Color::White,
            castle_flags: CastleFlags::parse_castle_flags("-"),
            en_passant_square: 0,
            halfmove_clock: 0,
            fullmove_number: 0,
        }
    }
}

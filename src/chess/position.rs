use super::{board::Board, castle_flags::CastleFlags, color::Color};

pub const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct Position {
    pub board: Board,
    pub active_turn: Color,
    pub castle_flags: CastleFlags,
    pub en_passant_square: u8,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            board: Board::from_fen(START_FEN),
            active_turn: Color::default(),
            castle_flags: CastleFlags::default(),
            en_passant_square: 0,
            halfmove_clock: 0,
            fullmove_number: 0,
        }
    }
}

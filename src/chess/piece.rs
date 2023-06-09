use super::color::Color;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceKind,
    pub pos: u8,
}

impl Piece {
    pub fn new(color: Color, kind: PieceKind, pos: u8) -> Self {
        Self { color, kind, pos }
    }
}

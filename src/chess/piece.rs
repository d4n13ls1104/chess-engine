use std::collections::HashMap;
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

impl PieceKind {
    pub fn from_char(c: char) -> PieceKind {
        match c.to_ascii_uppercase() {
            'P' => PieceKind::Pawn, 
            'N' => PieceKind::Knight, 
            'B' => PieceKind::Bishop, 
            'R' => PieceKind::Rook, 
            'Q' => PieceKind::Queen, 
            'K' => PieceKind::King, 
            _ => panic!("Invalid piece char: '{c}'"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceKind,
    pub pos: u8,
}

impl Piece {
    pub fn new( kind: PieceKind, color: Color, pos: u8) -> Piece {
        Self { kind, color, pos }
    }
}

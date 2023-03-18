#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Copy, Clone)]
pub struct Piece {
    pub piece_kind: PieceKind,
    pub color: Color,
}

impl Piece {
    pub fn to_char(self) -> char {
        let character = match self.piece_kind {
            PieceKind::Pawn => 'P',
            PieceKind::Knight => 'N',
            PieceKind::Bishop => 'B',
            PieceKind::Rook => 'R',
            PieceKind::Queen => 'Q',
            PieceKind::King => 'K',
        };

        if self.color == Color::White {
            character
        } else { character.to_ascii_lowercase() }
    }

    pub fn from_char(c: char) -> Self {
        let color = if c.is_uppercase() { Color::White } else { Color::Black };

        match c.to_ascii_uppercase() {
            'P' => Piece {
                piece_kind: PieceKind::Pawn,
                color
            },
            'N' => Piece {
                piece_kind: PieceKind::Knight,
                color
            },
            'B' => Piece {
                piece_kind: PieceKind::Bishop,
                color
            },
            'R' => Piece {
                piece_kind: PieceKind::Rook,
                color
            },
            'Q' => Piece {
                piece_kind: PieceKind::Queen,
                color
            },
            'K' => Piece {
                piece_kind: PieceKind::King,
                color
            },
            _ => panic!("Invalid Piece Character")
        }
    }

    pub fn get_direction_offsets(&self) -> Vec<(isize, isize)> {
        match self.piece_kind {
            PieceKind::Pawn => vec![(1, 0), (-1, 0), (2, 0), (-2, 0), (1, 1), (-1, -1), (-1, 1), (1, -1)],
            PieceKind::Knight => vec![(2, 1), (2, -1), (1, 2), (1, -2), (-1, 2), (-1, -2), (-2, 1), (-2, -1)],
            PieceKind::Bishop => vec![(1, 1), (1, -1), (-1, 1), (-1, -1)],
            PieceKind::Rook => vec![(0, 1), (0, -1), (1, 0), (-1, 0)],
            PieceKind::Queen => vec![(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)],
            PieceKind::King => vec![(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)],
        }
    }
}

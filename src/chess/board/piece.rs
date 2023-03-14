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
    pub fn get_value(&self) -> i32 {
        if self.color == Color::White {
            match self.piece_kind {
                PieceKind::Pawn => 1,
                PieceKind::Knight => 3,
                PieceKind::Bishop => 3,
                PieceKind::Rook => 5,
                PieceKind::Queen => 9,
                PieceKind::King => 10_000_000
            }
        } else {
            match self.piece_kind {
                PieceKind::Pawn => -1,
                PieceKind::Knight => -3,
                PieceKind::Bishop => -3,
                PieceKind::Rook => -5,
                PieceKind::Queen => -9,
                PieceKind::King => -10_000_000
            }
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

    pub fn to_char(self) -> char {
        match (self.piece_kind, self.color) {
            (PieceKind::Pawn, Color::White) => 'P',
            (PieceKind::Knight, Color::White) => 'N',
            (PieceKind::Bishop, Color::White) => 'B',
            (PieceKind::Rook, Color::White) => 'R',
            (PieceKind::Queen, Color::White) => 'Q',
            (PieceKind::King, Color::White) => 'K',

            (PieceKind::Pawn, Color::Black) => 'p',
            (PieceKind::Knight, Color::Black) => 'n',
            (PieceKind::Bishop, Color::Black) => 'b',
            (PieceKind::Rook, Color::Black) => 'r',
            (PieceKind::Queen, Color::Black) => 'q',
            (PieceKind::King, Color::Black) => 'k',
        }
    }

    pub fn from_char(c: char) -> Self {
        let color = if c.is_uppercase() { Color::White } else { Color::Black };

        match c.to_uppercase().next().unwrap() {
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
}

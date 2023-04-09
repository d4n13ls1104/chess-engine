#[allow(dead_code)]
#[allow(unused_variables)]
use super::{Board, Position};

pub(super) struct AttackOffsets;
impl AttackOffsets {
    pub const WHITE_PAWN: [(isize, isize); 2] = [(-1, -1), (-1, 1)];
    pub const BLACK_PAWN: [(isize, isize); 2] = [(1, 1), (1, -1)];

    pub const KNIGHT: [(isize, isize); 8] = [
        (2, 1),
        (2, -1),
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
        (-2, 1),
        (-2, -1),
    ];

    pub const BISHOP: [(isize, isize); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

    pub const KING: [(isize, isize); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    pub const ROOK: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    pub const QUEEN: [(isize, isize); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
}

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
    pub fn new(piece_kind: PieceKind, color: Color) -> Self {
        Self { piece_kind, color }
    }

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
        } else {
            character.to_ascii_lowercase()
        }
    }

    pub fn from_char(c: char) -> Self {
        let color = if c.is_uppercase() {
            Color::White
        } else {
            Color::Black
        };

        match c.to_ascii_uppercase() {
            'P' => Piece::new(PieceKind::Pawn, color),
            'N' => Piece::new(PieceKind::Knight, color),
            'B' => Piece::new(PieceKind::Bishop, color),
            'R' => Piece::new(PieceKind::Rook, color),
            'Q' => Piece::new(PieceKind::Queen, color),
            'K' => Piece::new(PieceKind::King, color),

            _ => panic!("Invalid Piece Character"),
        }
    }

    pub fn get_attack_positions(self, origin: Position, board: &Board) -> Vec<Position> {
        let mut positions = Vec::new();

        let attack_offsets = self.get_attack_offsets();

        for offset in attack_offsets {
            let target = (
                origin.0.wrapping_add_signed(offset.0),
                origin.1.wrapping_add_signed(offset.1),
            );

            if target.0 >= 8 || target.1 >= 8 {
                continue;
            }

            match self.piece_kind {
                PieceKind::Bishop | PieceKind::Rook | PieceKind::Queen => {
                    let mut path = self.get_path_from_direction(origin, offset, &board);

                    positions.append(&mut path);
                }
                _ => positions.push(target),
            }
        }
        positions
    }

    fn get_path_from_direction(
        self,
        origin: Position,
        direction: (isize, isize),
        board: &Board,
    ) -> Vec<Position> {
        let mut positions = Vec::new();

        let mut r_index = origin.0.wrapping_add_signed(direction.0);
        let mut f_index = origin.1.wrapping_add_signed(direction.1);

        while r_index < 8 && f_index < 8 {
            if let Some(piece) = board.state[r_index][f_index] {
                positions.push((r_index, f_index));
                return positions;
            }

            positions.push((r_index, f_index));

            r_index = r_index.wrapping_add_signed(direction.0);
            f_index = f_index.wrapping_add_signed(direction.1);
        }
        positions
    }

    fn get_attack_offsets(self) -> Vec<(isize, isize)> {
        match self.piece_kind {
            PieceKind::Pawn => {
                if self.color == Color::White {
                    AttackOffsets::WHITE_PAWN.to_vec()
                } else {
                    AttackOffsets::BLACK_PAWN.to_vec()
                }
            }

            PieceKind::Knight => AttackOffsets::KNIGHT.to_vec(),
            PieceKind::Bishop => AttackOffsets::BISHOP.to_vec(),
            PieceKind::Rook => AttackOffsets::ROOK.to_vec(),
            PieceKind::Queen => AttackOffsets::QUEEN.to_vec(),
            PieceKind::King => AttackOffsets::KING.to_vec(),
        }
    }
}

use super::{
    bitboard::Bitboard,
    color::Color,
    piece::{Piece, PieceKind},
};

#[derive(Default)]
pub struct Board {
    pub bb_pawn: Bitboard,
    pub bb_knight: Bitboard,
    pub bb_bishop: Bitboard,
    pub bb_rook: Bitboard,
    pub bb_queen: Bitboard,
    pub bb_king: Bitboard,

    pub bb_white: Bitboard,
    pub bb_black: Bitboard,
}

impl Board {
    pub fn put_piece(&mut self, piece: Piece, pos: u8) {
        let piece_bb = self.bb_for_piece_mut(piece.kind);
        piece_bb.set(pos);
        let color_bb = self.bb_for_color_mut(piece.color);
        color_bb.set(pos);
    }

    pub fn remove_piece(&mut self, piece: Piece, pos: u8) {
        let piece = self.piece_at(pos).unwrap();
        let piece_bb = self.bb_for_piece_mut(piece.kind);
        piece_bb.unset(pos);
        let color_bb = self.bb_for_color_mut(piece.color);
        color_bb.unset(pos);
    }

    pub fn bb_for_color_mut(&mut self, color: Color) -> &mut Bitboard {
        match color {
            Color::White => &mut self.bb_white,
            Color::Black => &mut self.bb_black,
        }
    }

    pub fn bb_for_piece_mut(&mut self, kind: PieceKind) -> &mut Bitboard {
        match kind {
            PieceKind::Pawn => &mut self.bb_pawn,
            PieceKind::Knight => &mut self.bb_knight,
            PieceKind::Bishop => &mut self.bb_bishop,
            PieceKind::Rook => &mut self.bb_rook,
            PieceKind::Queen => &mut self.bb_queen,
            PieceKind::King => &mut self.bb_king,
        }
    }

    pub fn bb_for_piece(&self, kind: PieceKind) -> Bitboard {
        match kind {
            PieceKind::Pawn => self.bb_pawn,
            PieceKind::Knight => self.bb_knight,
            PieceKind::Bishop => self.bb_bishop,
            PieceKind::Rook => self.bb_rook,
            PieceKind::Queen => self.bb_queen,
            PieceKind::King => self.bb_king,
        }
    }

    pub fn bb_for_color(&self, color: Color) -> Bitboard {
        match color {
            Color::White => self.bb_white,
            Color::Black => self.bb_black,
        }
    }

    pub fn piece_at(&self, idx: u8) -> Option<Piece> {
        let pawn_occupies = self.bb_pawn.nth(idx);
        let knight_occupies = self.bb_knight.nth(idx);
        let bishop_occupies = self.bb_bishop.nth(idx);
        let rook_occupies = self.bb_rook.nth(idx);
        let queen_occupies = self.bb_queen.nth(idx);
        let king_occupies = self.bb_king.nth(idx);

        let color = if self.bb_white.nth(idx) {
            Color::White
        } else if self.bb_black.nth(idx) {
            Color::Black
        } else {
            return None;
        };

        if pawn_occupies {
            Some(Piece::new(color, PieceKind::Pawn, idx))
        } else if knight_occupies {
            Some(Piece::new(color, PieceKind::Knight, idx))
        } else if bishop_occupies {
            Some(Piece::new(color, PieceKind::Bishop, idx))
        } else if rook_occupies {
            Some(Piece::new(color, PieceKind::Rook, idx))
        } else if queen_occupies {
            Some(Piece::new(color, PieceKind::Queen, idx))
        } else if king_occupies {
            Some(Piece::new(color, PieceKind::King, idx))
        } else {
            None
        }
    }

    pub fn from_ranks(rank_str: &str) -> Self {
        let mut board = Self::default();

        let rank_chunks: Vec<&str> = rank_str.split('/').collect();
        for (r_idx, rank) in rank_chunks.iter().enumerate() {
            let mut f_idx = 0;

            for c in rank.chars() {
                if c.is_numeric() {
                    f_idx += c.to_digit(10).unwrap() as u8;
                } else {
                    let idx = (r_idx as u8 * 8) + f_idx;
                    let piece_color = if c.is_ascii_uppercase() {
                        Color::White
                    } else {
                        Color::Black
                    };

                    match c.to_ascii_uppercase() {
                        'P' => board.bb_pawn.set(idx),
                        'N' => board.bb_knight.set(idx),
                        'B' => board.bb_bishop.set(idx),
                        'R' => board.bb_rook.set(idx),
                        'Q' => board.bb_queen.set(idx),
                        'K' => board.bb_king.set(idx),
                        _ => panic!("Invalid piece char: '{c}'"),
                    }
                    match piece_color {
                        Color::White => board.bb_white.set(idx),
                        Color::Black => board.bb_black.set(idx),
                    }
                    f_idx += 1;
                }
            }
        }
        board
    }
}

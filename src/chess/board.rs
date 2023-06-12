use crate::utils;
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
        let piece_bb = self.bb_for_piece_kind_mut(piece.kind);
        piece_bb.set(pos);
        let color_bb = self.bb_for_color_mut(piece.color);
        color_bb.set(pos);
    }

    pub fn remove_piece(&mut self, piece: Piece, pos: u8) {
        let piece = self.piece_at(pos).unwrap();
        let piece_bb = self.bb_for_piece_kind_mut(piece.kind);
        piece_bb.unset(pos);
        let color_bb = self.bb_for_color_mut(piece.color);
        color_bb.unset(pos);
    }

    pub fn from_ranks(rank_str: &str) -> Self {
        let mut board = Self::default();

        let rank_chunks = rank_str.split('/');
        for (rank_idx, rank) in rank_chunks.enumerate() {
            let mut file_idx = 0;

            for c in rank.chars() {
                if c.is_numeric() {
                    file_idx += c.to_digit(10).unwrap() as u8;
                } else {
                    let piece_kind = PieceKind::from_char(c);
                    let piece_color = Color::from_char_case(c);
                    let pos = utils::bb_pos_from_2d(rank_idx as u8, file_idx);

                    board.put_piece(Piece::new(piece_kind, piece_color, pos), pos);
                    file_idx += 1;
                }
            }
        }
        board
    }

    pub fn piece_at(&self, pos: u8) -> Option<Piece> {
        let pawn_occupies = self.bb_pawn.nth(pos);
        let knight_occupies = self.bb_knight.nth(pos);
        let bishop_occupies = self.bb_bishop.nth(pos);
        let rook_occupies = self.bb_rook.nth(pos);
        let queen_occupies = self.bb_queen.nth(pos);
        let king_occupies = self.bb_king.nth(pos);

        let white_occupies = self.bb_white.nth(pos);
        let black_occupies = self.bb_black.nth(pos);

        let color = if white_occupies {
            Color::White
        } else if black_occupies {
            Color::Black
        } else {
            return None;
        };

        if pawn_occupies {
            Some(Piece::new(PieceKind::Pawn, color, pos))
        } else if knight_occupies {
            Some(Piece::new(PieceKind::Knight, color, pos))
        } else if bishop_occupies {
            Some(Piece::new(PieceKind::Pawn, color, pos))
        } else if rook_occupies {
            Some(Piece::new(PieceKind::Pawn, color, pos))
        } else if queen_occupies {
            Some(Piece::new(PieceKind::Pawn, color, pos))
        } else if king_occupies {
            Some(Piece::new(PieceKind::Pawn, color, pos))
        } else {
            None
        }
    }

    pub fn bb_for_color_mut(&mut self, color: Color) -> &mut Bitboard {
        match color {
            Color::White => &mut self.bb_white,
            Color::Black => &mut self.bb_black,
        }
    }

    pub fn bb_for_piece_kind_mut(&mut self, kind: PieceKind) -> &mut Bitboard {
        match kind {
            PieceKind::Pawn => &mut self.bb_pawn,
            PieceKind::Knight => &mut self.bb_knight,
            PieceKind::Bishop => &mut self.bb_bishop,
            PieceKind::Rook => &mut self.bb_rook,
            PieceKind::Queen => &mut self.bb_queen,
            PieceKind::King => &mut self.bb_king,
        }
    }

    pub fn bb_for_piece_kind(&self, kind: PieceKind) -> &Bitboard {
        match kind {
            PieceKind::Pawn => &self.bb_pawn,
            PieceKind::Knight => &self.bb_knight,
            PieceKind::Bishop => &self.bb_bishop,
            PieceKind::Rook => &self.bb_rook,
            PieceKind::Queen => &self.bb_queen,
            PieceKind::King => &self.bb_king,
        }
    }

    pub fn bb_for_color(&self, color: Color) -> &Bitboard {
        match color {
            Color::White => &self.bb_white,
            Color::Black => &self.bb_black,
        }
    }
}

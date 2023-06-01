use super::bitboard::Bitboard;

const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct Position {
    bb_white_pawn: Bitboard,
    bb_white_knight: Bitboard,
    bb_white_bishop: Bitboard,
    bb_white_rook: Bitboard,
    bb_white_queen: Bitboard,
    bb_white_king: Bitboard,

    bb_black_pawn: Bitboard,
    bb_black_knight: Bitboard,
    bb_black_bishop: Bitboard,
    bb_black_rook: Bitboard,
    bb_black_queen: Bitboard,
    bb_black_king: Bitboard,
}

impl Position {
    pub fn from_fen(fen: &str) -> Self {
        let mut fen_fields = Vec::with_capacity(6);
        let mut position = Self::empty();

        if fen == "start" {
            for field in START_FEN.split_whitespace() {
                fen_fields.push(field);
            }
        } else {
            for field in fen.split_whitespace() {
                fen_fields.push(field);
            }
        }

        let ranks = fen_fields[0];
        let rank_chunks: Vec<&str> = ranks.split('/').collect();

        for (r_idx, rank) in rank_chunks.iter().enumerate() {
            let mut f_idx = 0;

            for c in rank.chars() {
                if c.is_numeric() {
                    f_idx += c.to_digit(10).unwrap() as u8;
                } else {
                    let idx = (r_idx as u8 * 8) + f_idx;

                    match c {
                        'P' => position.bb_white_pawn.set(idx),
                        'N' => position.bb_white_knight.set(idx),
                        'B' => position.bb_white_bishop.set(idx),
                        'R' => position.bb_white_rook.set(idx),
                        'Q' => position.bb_white_queen.set(idx),
                        'K' => position.bb_white_king.set(idx),

                        'p' => position.bb_black_pawn.set(idx),
                        'n' => position.bb_black_knight.set(idx),
                        'b' => position.bb_black_bishop.set(idx),
                        'r' => position.bb_black_rook.set(idx),
                        'q' => position.bb_black_queen.set(idx),
                        'k' => position.bb_black_king.set(idx),

                        _ => panic!("Invalid piece character: '{c}'"),
                    }
                    f_idx += 1;
                }
            }
        }
        position
    }

    pub fn empty() -> Self {
        Self {
            bb_white_pawn: Bitboard::new(),
            bb_white_knight: Bitboard::new(),
            bb_white_bishop: Bitboard::new(),
            bb_white_rook: Bitboard::new(),
            bb_white_queen: Bitboard::new(),
            bb_white_king: Bitboard::new(),
            bb_black_pawn: Bitboard::new(),
            bb_black_knight: Bitboard::new(),
            bb_black_bishop: Bitboard::new(),
            bb_black_rook: Bitboard::new(),
            bb_black_queen: Bitboard::new(),
            bb_black_king: Bitboard::new(),
        }
    }
}

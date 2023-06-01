use super::bitboard::Bitboard;

pub const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct Board {
    pub bb_white_pawn: Bitboard,
    pub bb_white_knight: Bitboard,
    pub bb_white_bishop: Bitboard,
    pub bb_white_rook: Bitboard,
    pub bb_white_queen: Bitboard,
    pub bb_white_king: Bitboard,

    pub bb_black_pawn: Bitboard,
    pub bb_black_knight: Bitboard,
    pub bb_black_bishop: Bitboard,
    pub bb_black_rook: Bitboard,
    pub bb_black_queen: Bitboard,
    pub bb_black_king: Bitboard,
}

impl Board {
    pub fn from_fen(fen: &str) -> Self {
        let mut fen_fields = Vec::with_capacity(6);
        let mut board = Self::empty();

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
                        'P' => board.bb_white_pawn.set(idx),
                        'N' => board.bb_white_knight.set(idx),
                        'B' => board.bb_white_bishop.set(idx),
                        'R' => board.bb_white_rook.set(idx),
                        'Q' => board.bb_white_queen.set(idx),
                        'K' => board.bb_white_king.set(idx),

                        'p' => board.bb_black_pawn.set(idx),
                        'n' => board.bb_black_knight.set(idx),
                        'b' => board.bb_black_bishop.set(idx),
                        'r' => board.bb_black_rook.set(idx),
                        'q' => board.bb_black_queen.set(idx),
                        'k' => board.bb_black_king.set(idx),

                        _ => panic!("Invalid piece char: '{c}'"),
                    }
                    f_idx += 1;
                }
            }
        }
        board
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

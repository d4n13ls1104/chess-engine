use super::bitboard::Bitboard;

#[derive(Default)]
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
}

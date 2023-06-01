pub struct CastleFlags {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl CastleFlags {
    pub fn parse_castle_flags(castle_flags: &str) -> Self {
        let mut white_kingside = false;
        let mut white_queenside = false;
        let mut black_kingside = false;
        let mut black_queenside = false;

        for c in castle_flags.chars() {
            match c {
                'K' => white_kingside = true,
                'Q' => white_queenside = true,
                'k' => black_kingside = true,
                'q' => black_queenside = true,
                _ => (),
            }
        }

        Self {
            white_kingside,
            white_queenside,
            black_kingside,
            black_queenside,
        }
    }
}

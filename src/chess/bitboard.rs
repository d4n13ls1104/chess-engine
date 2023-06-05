pub struct Bitboard(u64);

impl Bitboard {
    pub fn set(&mut self, idx: u8) {
        self.0 |= 1 << idx;
    }
    pub fn unset(&mut self, idx: u8) {
        self.0 &= !(1 << idx);
    }
    pub fn bits(&self) -> u64 {
        self.0
    }
    pub fn nth(&self, n: u8) -> bool {
        (self.0 >> n) & 1 == 1
    }
}

impl Default for Bitboard {
    fn default() -> Self {
        Self(0)
    }
}

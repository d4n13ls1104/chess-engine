pub(super) struct Bitboard(u64);

impl Bitboard {
    pub fn set(&mut self, idx: u8) {
        self.0 |= 1 << idx;
    }
    pub fn unset(&mut self, idx: u8) {
        self.0 &= !(1 << idx);
    }
    pub fn bits(&mut self) -> u64 {
        self.0
    }
    pub fn new() -> Self {
        Self(0)
    }
}

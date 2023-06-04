pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn invert(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
    pub fn from_char(c: char) -> Self {
        match c.to_ascii_lowercase() {
            'w' => Color::White,
            'b' => Color::Black,
            _ => panic!("Invalid color char"),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::White
    }
}

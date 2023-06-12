#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub enum Color {
    #[default]
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
    pub fn from_char_case(c: char) -> Self {
        if c.is_ascii_uppercase() {
            Color::White
        } else {
            Color::Black
        }
    }
}

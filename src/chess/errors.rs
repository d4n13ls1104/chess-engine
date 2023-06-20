use super::piece::Piece;
use std::fmt;

#[derive(Debug)]
pub enum IllegalMoveError {
    SelfCapture {
        selected_piece: Piece,
        target_piece: Piece,
    },
    MissingOriginPiece,
}

impl fmt::Display for IllegalMoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IllegalMoveError::SelfCapture {
                selected_piece,
                target_piece,
            } => {
                write!(f, "Illegal move: attempt to self capture. Details: [selected_piece: {:?} | target_piece: {:?}]", selected_piece, target_piece)
            }

            IllegalMoveError::MissingOriginPiece => {
                write!(f, "A move was attempted but there was no origin piece.")
            }
        }
    }
}

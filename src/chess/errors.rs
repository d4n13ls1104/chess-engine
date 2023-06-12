use super::piece::Piece;
use std::fmt;

#[derive(Debug)]
pub enum IllegalMoveError {
    SelfCapture {
        origin_piece: Piece,
        target_piece: Piece,
    },
    MissingOriginPiece,
}

impl fmt::Display for IllegalMoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IllegalMoveError::SelfCapture {
                origin_piece,
                target_piece,
            } => {
                write!(f, "Illegal move: attempt to self capture. Details: [origin_piece: {:?} | target_piece: {:?}]", origin_piece, target_piece)
            }

            IllegalMoveError::MissingOriginPiece => {
                write!(f, "A move was attempted but there was no origin piece.")
            }
        }
    }
}

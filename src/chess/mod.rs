#![allow(unused)]
#![allow(dead_code)]
use std::fmt;
use std::io::BufReader;
use sdl2::rect::Point;

use self::{piece::Piece, position::Position};

pub mod color;
pub mod piece;
mod bitboard;
mod board;
mod castle_flags;
mod position;

pub enum MoveKind {
    MoveSelf,
    Capture,
    Check,
} 

pub struct Chess {
    pub position: Position,
    pub target_piece: Option<Piece>,
    pub mouse_pos: Point,
    pub mouse_drag: bool,
}

impl Chess {
    pub fn move_selected_piece(&mut self, dst_square: u8) -> Result<MoveKind, IllegalMoveError> {
        let board = &mut self.position.board;
        let origin_piece = match self.target_piece {
            Some(p) => p,
            None => {
                return Err(IllegalMoveError::MissingOriginPiece);
            }
        };

        if let Some(target_piece) = board.piece_at(dst_square) {
            let capturing_opponent = target_piece.color != origin_piece.color;
            if capturing_opponent {
                board.remove_piece(target_piece, dst_square);
                board.remove_piece(origin_piece, origin_piece.pos);

                board.put_piece(origin_piece, dst_square);
                return Ok(MoveKind::Capture);
            }
            return Err(IllegalMoveError::SelfCapture { origin_piece, target_piece });
        }

        board.remove_piece(origin_piece, origin_piece.pos);
        board.put_piece(origin_piece, dst_square);
        Ok(MoveKind::MoveSelf)
  }
}

impl Default for Chess {
    fn default() -> Self {
        Self {
            position: Position::default(),
            target_piece: None,
            mouse_pos: Point::new(0, 0),
            mouse_drag: false,
        }
    }
}

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
            IllegalMoveError::SelfCapture { origin_piece, target_piece } => {
                write!(f, "Illegal move: attempt to self capture. Details: [origin_piece: {:?} | target_piece: {:?}]", origin_piece, target_piece)
            }

            IllegalMoveError::MissingOriginPiece => {
                write!(f, "A move was attempted but there was no origin piece.")
            }
        }
    }
}

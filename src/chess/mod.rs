#![allow(unused)]
#![allow(dead_code)]
use sdl2::rect::Point;
use std::fmt;
use std::io::BufReader;

use self::{errors::IllegalMoveError, piece::Piece, position::Position};

pub mod color;
pub mod piece;

mod bitboard;
mod board;
mod castle_flags;
mod errors;
mod position;

pub enum MoveKind {
    MoveSelf,
    Capture,
    Check,
}

pub struct Chess {
    pub position: Position,
    pub selected_piece: Option<Piece>,
    pub mouse_pos: Point,
    pub mouse_drag: bool,
}

impl Chess {
    pub fn move_selected_piece(&mut self, dst_square: u8) -> Result<MoveKind, IllegalMoveError> {
        let origin_piece = match self.selected_piece {
            Some(p) => p,
            None => {
                return Err(IllegalMoveError::MissingOriginPiece);
            }
        };

        let board = &mut self.position.board;
        match self.selected_piece {
            Some(target_piece) => {
                let is_capturing_opponent = target_piece.color != origin_piece.color;

                if is_capturing_opponent {
                    board.remove_piece(target_piece, dst_square);
                    board.remove_piece(origin_piece, origin_piece.pos);

                    board.put_piece(origin_piece, dst_square);
                    return Ok(MoveKind::Capture);
                }
                return Err(IllegalMoveError::SelfCapture {
                    origin_piece,
                    target_piece,
                });
            },

            None => {
                board.remove_piece(origin_piece, origin_piece.pos);
                board.put_piece(origin_piece, dst_square);
                return Ok(MoveKind::MoveSelf);
            }
        }
    }
}

impl Default for Chess {
    fn default() -> Self {
        Self {
            position: Position::default(),
            selected_piece: None,
            mouse_pos: Point::new(0, 0),
            mouse_drag: false,
        }
    }
}

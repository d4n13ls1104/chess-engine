#![allow(unused)]
#![allow(dead_code)]
use self::position::Position;

mod bitboard;
mod board;
mod castle_flags;
mod color;
mod position;

#[derive(Default)]
pub struct Chess {
    pub position: Position,
}

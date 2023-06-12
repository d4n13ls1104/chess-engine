use sdl2::rect::Point;

use crate::render::{BOARD_SIZE, CELL_SIZE};

pub fn bb_pos_from_pixel_pos(point: Point) -> u8 {
    let rank = (point.y() / CELL_SIZE as i32) as u8;
    let file = (point.x() / CELL_SIZE as i32) as u8;

    bb_pos_from_2d(rank, file)
}

pub fn bb_pos_from_2d(x: u8, y: u8) -> u8 {
    x * BOARD_SIZE as u8 + y
}

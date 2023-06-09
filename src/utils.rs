use sdl2::rect::Point;

use crate::render::CELL_SIZE;

// get piece position (0-63) index of bitboard) from pixel position, e.g (231, 400) -> (3, 4)
pub fn bb_pos_from_point(point: Point) -> u8 {
    let rank = (point.y() / CELL_SIZE as i32) as u8;
    let file = (point.x() / CELL_SIZE as i32) as u8;
    rank * 8 + file
}

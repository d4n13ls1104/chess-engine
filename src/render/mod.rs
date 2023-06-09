use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

use crate::chess::{Chess, piece::Piece}; 

pub mod textures;

pub const BOARD_SIZE: u16 = 8;
pub const WINDOW_SIZE: u16 = 624;
pub const CELL_SIZE: u16 = WINDOW_SIZE / BOARD_SIZE;

const FOREGROUND_COLOR: sdl2::pixels::Color = sdl2::pixels::Color::RGB(181, 136, 99);
const BACKGROUND_COLOR: sdl2::pixels::Color = sdl2::pixels::Color::RGB(240, 217, 181);

pub fn render_window(
    canvas: &mut Canvas<Window>,
    texture_store: &Vec<(Piece, Texture)>,
    game: &Chess,
) {
    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();

    let board = &mut game.position.board;
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            let x = col * CELL_SIZE;
            let y = row * CELL_SIZE;
            let rect = Rect::new(x as i32, y as i32, CELL_SIZE as u32, CELL_SIZE as u32);

            let color = if (row + col) % 2 == 0 {
                BACKGROUND_COLOR
            } else {
                FOREGROUND_COLOR
            };

            canvas.set_draw_color(color);
            canvas.fill_rect(rect).unwrap();

            for item in texture_store {
                let (piece, texture) = item;

                let bit_idx = (row * BOARD_SIZE + col) as u8;
                // check if the piece being dragged is the one currently being rendered,
                // if it is skip over it to make it look like we picked it up
                if let Some(target_piece) = game.target_piece {
                    if target_piece.pos == bit_idx {
                        continue; // skip iteration and dont render grabbed piece
                    }
                }

                let piece_bit = board.bb_for_piece_kind(piece.kind).nth(bit_idx);
                let color_bit = board.bb_for_color(piece.color).nth(bit_idx);

                if piece_bit && color_bit {
                    let rect = Rect::new(x as i32, y as i32, CELL_SIZE as u32, CELL_SIZE as u32);
                    canvas.copy(texture, None, rect).unwrap();
                    break;
                }
            }
        }
    }

    if game.mouse_drag {
        if let Some((piece, _)) = texture_store.iter().find(|(p, _)| {
            let target_piece = game.target_piece.unwrap();
            p.kind == target_piece.kind && p.color == target_piece.color
        }) {
            render_piece_at_cursor(canvas, texture_store, *piece, game.mouse_pos);
        }
    }
    canvas.present();
}

fn render_piece_at_cursor(
    canvas: &mut Canvas<Window>,
    texture_store: &Vec<(Piece, Texture)>,
    piece: Piece,
    mouse_pos: Point
) {
    let texture = texture_store
        .iter()
        .find(|(p, _)| *p == piece)
        .map(|(_, texture)| texture)
        .unwrap();
    let rect = Rect::new(
        mouse_pos.x() - (CELL_SIZE / 2) as i32,
        mouse_pos.y() - (CELL_SIZE / 2) as i32,
        CELL_SIZE as u32,
        CELL_SIZE as u32,
    );

    canvas.copy(texture, None, rect).unwrap();
}

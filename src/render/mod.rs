use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

use crate::chess::color::Color;
use crate::chess::piece::{Piece, PieceKind};
use crate::chess::Chess;

pub mod textures;

const BOARD_SIZE: u16 = 8;
pub const WINDOW_SIZE: u16 = 500;
const CELL_SIZE: u16 = WINDOW_SIZE / BOARD_SIZE;

const FOREGROUND_COLOR: sdl2::pixels::Color = sdl2::pixels::Color::RGB(181, 136, 99);
const BACKGROUND_COLOR: sdl2::pixels::Color = sdl2::pixels::Color::RGB(240, 217, 181);

pub fn render_window(
    canvas: &mut Canvas<Window>,
    texture_store: &Vec<(Piece, Texture)>,
    game: &Chess,
) {
    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();

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

            let board = &game.position.board;
            for item in texture_store {
                let (piece, texture) = item;
                let bb = match (piece.color, piece.kind) {
                    (Color::White, PieceKind::Pawn) => &board.bb_white_pawn,
                    (Color::Black, PieceKind::Pawn) => &board.bb_black_pawn,
                    (Color::White, PieceKind::Knight) => &board.bb_white_knight,
                    (Color::Black, PieceKind::Knight) => &board.bb_black_knight,
                    (Color::White, PieceKind::Bishop) => &board.bb_white_bishop,
                    (Color::Black, PieceKind::Bishop) => &board.bb_black_bishop,
                    (Color::White, PieceKind::Rook) => &board.bb_white_rook,
                    (Color::Black, PieceKind::Rook) => &board.bb_black_rook,
                    (Color::White, PieceKind::Queen) => &board.bb_white_queen,
                    (Color::Black, PieceKind::Queen) => &board.bb_black_queen,
                    (Color::White, PieceKind::King) => &board.bb_white_king,
                    (Color::Black, PieceKind::King) => &board.bb_black_king,
                };

                let bit_idx = (row * BOARD_SIZE + col) as u8;
                if bb.nth(bit_idx) {
                    let rect = Rect::new(x as i32, y as i32, CELL_SIZE as u32, CELL_SIZE as u32);
                    canvas.copy(&texture, None, rect).unwrap();
                    break;
                }
            }
        }
    }
    canvas.present();
}

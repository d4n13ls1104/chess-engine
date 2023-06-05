mod chess;
use chess::Chess;

use sdl2::event::Event;
use sdl2::image::LoadSurface;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::{Window, WindowContext};
use std::env;

const BOARD_SIZE: u8 = 8;
const WINDOW_SIZE: u16 = 500;
const CELL_SIZE: u16 = WINDOW_SIZE / BOARD_SIZE as u16;

const FOREGROUND_COLOR: Color = Color::RGB(181, 136, 99);
const BACKGROUND_COLOR: Color = Color::RGB(240, 217, 181);

const PIECE_TEXTURE_PATHS: [&str; 12] = [
    "w_pawn.png",
    "b_pawn.png",
    "w_knight.png",
    "b_knight.png",
    "w_bishop.png",
    "b_bishop.png",
    "w_rook.png",
    "b_rook.png",
    "w_queen.png",
    "b_queen.png",
    "w_king.png",
    "b_king.png",
];

fn render(
    canvas: &mut Canvas<Window>,
    texture_creator: &TextureCreator<WindowContext>,
    game: &Chess,
) {
    let mut dir = env::current_exe().unwrap();
    dir.pop();
    dir.push("textures");

    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();

    for row in 0..BOARD_SIZE as u16 {
        for col in 0..BOARD_SIZE as u16 {
            let x = col * CELL_SIZE;
            let y = row * CELL_SIZE;

            let rect: Rect = Rect::new(x as i32, y as i32, CELL_SIZE as u32, CELL_SIZE as u32);

            let color = if (row + col) % 2 == 0 {
                BACKGROUND_COLOR
            } else {
                FOREGROUND_COLOR
            };

            canvas.set_draw_color(color);
            canvas.fill_rect(rect).unwrap();

            for &path in &PIECE_TEXTURE_PATHS {
                let board = &game.position.board;

                if let Some(bb) = match path {
                    "w_pawn.png" => Some(&board.bb_white_pawn),
                    "b_pawn.png" => Some(&board.bb_black_pawn),
                    "w_knight.png" => Some(&board.bb_white_knight),
                    "b_knight.png" => Some(&board.bb_black_knight),
                    "w_bishop.png" => Some(&board.bb_white_bishop),
                    "b_bishop.png" => Some(&board.bb_black_bishop),
                    "w_rook.png" => Some(&board.bb_white_rook),
                    "b_rook.png" => Some(&board.bb_black_rook),
                    "w_queen.png" => Some(&board.bb_white_queen),
                    "b_queen.png" => Some(&board.bb_black_queen),
                    "w_king.png" => Some(&board.bb_white_king),
                    "b_king.png" => Some(&board.bb_black_king),
                    _ => None,
                } {
                    let bit_idx = row * BOARD_SIZE as u16 + col;
                    if bb.nth(bit_idx as u8) {
                        let texture_path = dir.join(path);
                        let surface = Surface::from_file(texture_path).unwrap();
                        let texture = texture_creator
                            .create_texture_from_surface(&surface)
                            .unwrap();

                        let square_rect =
                            Rect::new(x as i32, y as i32, CELL_SIZE as u32, CELL_SIZE as u32);

                        canvas.copy(&texture, None, square_rect).unwrap();
                        break;
                    }
                }
            }
        }
    }
    canvas.present();
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Chess", WINDOW_SIZE as u32, WINDOW_SIZE as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let game = Chess::default();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        render(&mut canvas, &texture_creator, &game);
    }
}

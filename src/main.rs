mod chess;
mod render;

use crate::chess::Chess;
use crate::render::{render_window, textures::load_textures, WINDOW_SIZE};
use sdl2::{event::Event, keyboard::Keycode};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let window = video_subsystem
        .window("Chess", WINDOW_SIZE as u32, WINDOW_SIZE as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let texture_store = load_textures(&texture_creator);
    let game = Chess::default();

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
        render_window(&mut canvas, &texture_store, &game);
    }
}

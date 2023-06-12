use sdl2::rect::Point;
use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton};
use std::io::BufReader;

use crate::{
    chess::{Chess, MoveKind},
    render::{render_window, textures::load_textures, WINDOW_SIZE},
};

mod chess;
mod render;
mod utils;

fn main() {
    let (_rodio_stream, rodio_handle) = rodio::OutputStream::try_default().unwrap();

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
    let mut game = Chess::default();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::MouseMotion { x, y, .. } => {
                    game.mouse_pos = Point::new(x, y);
                }

                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => {
                    let target_square_idx = utils::bb_pos_from_pixel_pos(Point::new(x, y));
                    let piece = game.position.board.piece_at(target_square_idx);

                    if piece.is_some() {
                        game.mouse_drag = true;
                        game.selected_piece = piece;
                    }
                }

                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => {
                    let mouse_up_point = Point::new(x, y);
                    let dst_square = utils::bb_pos_from_pixel_pos(mouse_up_point);

                    let move_kind = game.move_selected_piece(dst_square).unwrap();
                    match move_kind {
                        MoveKind::MoveSelf => {
                            let move_self_wav_file = std::fs::File::open("assets/move-self.wav").unwrap();
                            let drop_sound = rodio_handle
                                .play_once(BufReader::new(move_self_wav_file))
                                .unwrap();
                            drop_sound.set_volume(0.3);
                            drop_sound.detach();
                        }

                        MoveKind::Capture => {
                            let move_capture_wav_file = std::fs::File::open("assets/capture.wav").unwrap();
                            let capture_sound = rodio_handle
                                .play_once(BufReader::new(move_capture_wav_file))
                                .unwrap();
                            capture_sound.set_volume(0.3);
                            capture_sound.detach();
                        }

                        _ => todo!("not implemented"),
                    }

                    game.mouse_drag = false;
                    game.selected_piece = None;
                },
                _ => {}
            }
        }
        render_window(&mut canvas, &texture_store, &game);
    }
}

use sdl2::{
    image::LoadSurface,
    render::{Texture, TextureCreator},
    surface::Surface,
    video::WindowContext,
};

use crate::chess::{
    color::Color,
    piece::{Piece, PieceKind},
};

const TEXTURE_BASE_DIR: &str = "textures";

pub fn load_textures(texture_creator: &TextureCreator<WindowContext>) -> Vec<(Piece, Texture)> {
    let mut result: Vec<(Piece, Texture)> = Vec::with_capacity(PIECE_TEXTURE_MAP.len());
    let mut dir = std::env::current_exe().unwrap();
    dir.pop();
    dir.push(TEXTURE_BASE_DIR);

    for item in PIECE_TEXTURE_MAP {
        let (texture_path, piece) = item;
        let file_path = dir.join(texture_path);
        let surface = Surface::from_file(file_path).unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        result.push((piece, texture));
    }
    result
}

pub const PIECE_TEXTURE_MAP: [(&str, Piece); 12] = [
    (
        "w_pawn.png",
        Piece {
            color: Color::White,
            kind: PieceKind::Pawn,
            pos: 0,
        },
    ),
    (
        "b_pawn.png",
        Piece {
            color: Color::Black,
            kind: PieceKind::Pawn,
            pos: 0,
        },
    ),
    (
        "w_knight.png",
        Piece {
            color: Color::White,
            kind: PieceKind::Knight,
            pos: 0,
        },
    ),
    (
        "b_knight.png",
        Piece {
            color: Color::Black,
            kind: PieceKind::Knight,
            pos: 0,
        },
    ),
    (
        "w_bishop.png",
        Piece {
            color: Color::White,
            kind: PieceKind::Bishop,
            pos: 0,
        },
    ),
    (
        "b_bishop.png",
        Piece {
            color: Color::Black,
            kind: PieceKind::Bishop,
            pos: 0,
        },
    ),
    (
        "w_rook.png",
        Piece {
            color: Color::White,
            kind: PieceKind::Rook,
            pos: 0,
        },
    ),
    (
        "b_rook.png",
        Piece {
            color: Color::Black,
            kind: PieceKind::Rook,
            pos: 0,
        },
    ),
    (
        "w_queen.png",
        Piece {
            color: Color::White,
            kind: PieceKind::Queen,
            pos: 0,
        },
    ),
    (
        "b_queen.png",
        Piece {
            color: Color::Black,
            kind: PieceKind::Queen,
            pos: 0,
        },
    ),
    (
        "w_king.png",
        Piece {
            color: Color::White,
            kind: PieceKind::King,
            pos: 0,
        },
    ),
    (
        "b_king.png",
        Piece {
            color: Color::Black,
            kind: PieceKind::King,
            pos: 0,
        },
    ),
];

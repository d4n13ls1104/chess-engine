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

pub fn load_textures(texture_creator: &TextureCreator<WindowContext>) -> Vec<(Piece, Texture)> {
    let mut result: Vec<(Piece, Texture)> = Vec::with_capacity(PIECE_TEXTURE_MAP.len());
    let mut dir = std::env::current_exe().unwrap();
    dir.pop();
    dir.push("textures");

    for item in PIECE_TEXTURE_MAP {
        let (path, piece) = item;
        let texture_path = dir.join(path);
        let surface = Surface::from_file(texture_path).unwrap();
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
        },
    ),
    (
        "b_pawn.png",
        Piece {
            color: Color::Black,
            kind: PieceKind::Pawn,
        },
    ),
    (
        "w_knight.png",
        Piece {
            color: Color::White,
            kind: PieceKind::Knight,
        },
    ),
    (
        "b_knight.png",
        Piece {
            color: Color::Black,
            kind: PieceKind::Knight,
        },
    ),
    (
        "w_bishop.png",
        Piece {
            color: Color::White,
            kind: PieceKind::Bishop,
        },
    ),
    (
        "b_bishop.png",
        Piece {
            color: Color::Black,
            kind: PieceKind::Bishop,
        },
    ),
    (
        "w_rook.png",
        Piece {
            color: Color::White,
            kind: PieceKind::Rook,
        },
    ),
    (
        "b_rook.png",
        Piece {
            color: Color::Black,
            kind: PieceKind::Rook,
        },
    ),
    (
        "w_queen.png",
        Piece {
            color: Color::White,
            kind: PieceKind::Queen,
        },
    ),
    (
        "b_queen.png",
        Piece {
            color: Color::Black,
            kind: PieceKind::Queen,
        },
    ),
    (
        "w_king.png",
        Piece {
            color: Color::White,
            kind: PieceKind::King,
        },
    ),
    (
        "b_king.png",
        Piece {
            color: Color::Black,
            kind: PieceKind::King,
        },
    ),
];

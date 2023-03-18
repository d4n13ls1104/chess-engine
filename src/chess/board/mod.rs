use std::env;
use image::{RgbaImage, Rgba, imageops};

use self::chess_move::Move;
use self::chess_move::MoveError;

pub use self::piece::*;
pub use self::fen::*;

pub mod piece;
pub mod fen;
pub mod chess_move;

const BOARD_SIZE: usize = 8;
const IMAGE_SIZE: u32 = 512;

const BOARD_FOREGROUND_COLOR: Rgba<u8> = Rgba([181, 136, 99, 255]);
const BOARD_BACKGROUND_COLOR: Rgba<u8> = Rgba([240, 217, 181, 255]);

const WHITE_KING_POS: Position = (7, 4);
const BLACK_KING_POS: Position = (0, 4);

const WHITE_KINGSIDE_ROOK_POS: Position = (7, 7);
const BLACK_KINGSIDE_ROOK_POS: Position = (0, 7);
const WHITE_QUEENSIDE_ROOK_POS: Position = (7, 0);
const BLACK_QUEENSIDE_ROOK_POS: Position = (0, 0);

pub type Position = (usize, usize);

#[derive(PartialEq)]
pub enum GameStatus {
    WhiteWin,
    BlackWin,
    Draw,
    Ongoing
}

#[derive(Clone)]
pub struct Board {
    pub state: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE],
    pub active_turn: Color,
    pub castle_flags: CastleFlags,
    pub en_passant_target_square: Option<Position>,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,
    pub history: Vec<Move>,
    pub white_king_position: Option<Position>,
    pub black_king_position: Option<Position>,
    pub white_kingside_rook_position: Option<Position>,
    pub white_queenside_rook_position: Option<Position>,
    pub black_kingside_rook_position: Option<Position>,
    pub black_queenside_rook_position: Option<Position>,
}

impl Board {
    pub fn render_to_rgba_image(&self) -> RgbaImage {
        let mut image = RgbaImage::new(IMAGE_SIZE, IMAGE_SIZE);

        for r_index in 0..BOARD_SIZE {
            for f_index in 0..BOARD_SIZE {
                let color = if (r_index + f_index) % 2 == 0 { 
                    BOARD_BACKGROUND_COLOR
                } else { BOARD_FOREGROUND_COLOR };

                let square_size = IMAGE_SIZE / 8;

                let start_x = f_index as u32 * square_size;
                let start_y = r_index as u32 * square_size;

                let end_x = start_x + square_size;
                let end_y = start_y + square_size;

                for x in start_x..end_x {
                    for y in start_y..end_y {
                        image.put_pixel(x, y, color);
                    } 
                }

                if let Some(piece) = self.state[r_index][f_index] {
                    let mut dir = match env::current_exe() {
                        Ok(path) => path,
                        Err(e) => panic!("Failed to get program directory: {}", e)
                    };


                    let color_flag = if piece.color == Color::White { 'w' } else { 'b' };
                    let file_name = format!("{}_{:?}.png", color_flag, piece.piece_kind).to_lowercase();

                    dir.pop();
                    dir.push("textures");
                    dir.push(file_name);

                    let piece_texture = image::open(dir)
                        .expect("Failed to load piece texture")
                        .into_rgba8();

                    imageops::overlay(&mut image, &piece_texture, start_x as i64, start_y as i64);
                }
            }
        }
        image
    }

    pub fn get_game_status(&self) -> GameStatus {
        let legal_moves = self.get_legal_moves();

        if legal_moves.is_empty() {
            if self.halfmove_clock >= 100 { return GameStatus::Draw; }

            match self.active_turn {
                Color::White if self.is_square_attacked(self.white_king_position.unwrap()) => GameStatus::BlackWin,
                Color::Black if self.is_square_attacked(self.black_king_position.unwrap()) => GameStatus::WhiteWin,
                _ => GameStatus::Draw
            }

        } else { GameStatus::Ongoing }
    }

    pub fn castle_queenside(&mut self) {
        match self.active_turn {
            Color::White => {
                let king = self.state[WHITE_KING_POS.0][WHITE_KING_POS.1].take();
                let rook = self.state[WHITE_QUEENSIDE_ROOK_POS.0][WHITE_QUEENSIDE_ROOK_POS.1].take();

                self.state[WHITE_KING_POS.0][WHITE_KING_POS.1 - 2] = king;
                self.state[WHITE_QUEENSIDE_ROOK_POS.0][WHITE_QUEENSIDE_ROOK_POS.1 + 3] = rook;

                self.castle_flags.white_kingside = false;
                self.castle_flags.white_queenside = false;
                self.active_turn = Color::Black;
            },

            Color::Black => {
                let king = self.state[BLACK_KING_POS.0][BLACK_KING_POS.1].take();
                let rook = self.state[BLACK_QUEENSIDE_ROOK_POS.0][BLACK_QUEENSIDE_ROOK_POS.1].take();

                self.state[BLACK_KING_POS.0][BLACK_KING_POS.1 - 2] = king;
                self.state[BLACK_QUEENSIDE_ROOK_POS.0][BLACK_QUEENSIDE_ROOK_POS.1 + 3] = rook;

                self.castle_flags.black_kingside = false;
                self.castle_flags.black_queenside = false;
                self.active_turn = Color::White;
            }
        }
    }

    pub fn castle_kingside(&mut self) {
        match self.active_turn {
            Color::White => {
                let king = self.state[WHITE_KING_POS.0][WHITE_KING_POS.1].take();
                let rook = self.state[WHITE_KINGSIDE_ROOK_POS.0][WHITE_KINGSIDE_ROOK_POS.1].take();

                self.state[WHITE_KING_POS.0][WHITE_KING_POS.1 + 2] = king;
                self.state[WHITE_KINGSIDE_ROOK_POS.0][WHITE_KINGSIDE_ROOK_POS.1 - 2] = rook;

                self.castle_flags.white_kingside = false;
                self.castle_flags.white_queenside = false;
                self.active_turn = Color::Black;
            },

            Color::Black => {
                let king = self.state[BLACK_KING_POS.0][BLACK_KING_POS.1].take();
                let rook = self.state[BLACK_KINGSIDE_ROOK_POS.0][BLACK_KINGSIDE_ROOK_POS.1].take();

                self.state[BLACK_KING_POS.0][BLACK_KING_POS.1 + 2] = king;
                self.state[BLACK_KINGSIDE_ROOK_POS.0][BLACK_KINGSIDE_ROOK_POS.1 - 2] = rook;

                self.castle_flags.black_kingside = false;
                self.castle_flags.black_queenside = false;
                self.active_turn = Color::White;
            }
        }
    }

    pub fn get_legal_moves(&self) -> Vec<Move> {
        let mut legal_moves = Vec::new();

        for r_index in 0..8 {
            for f_index in 0..8 {
                match self.active_turn {
                    Color::White => {
                        if self.castle_flags.white_kingside && self.is_legal_move(Move::CastleKingside) {
                            legal_moves.push(Move::CastleKingside);
                        }

                        if self.castle_flags.white_queenside && self.is_legal_move(Move::CastleQueenside) {
                            legal_moves.push(Move::CastleQueenside);
                        }
                    },

                    Color::Black => {
                        if self.castle_flags.black_kingside && self.is_legal_move(Move::CastleKingside) {
                            legal_moves.push(Move::CastleKingside);
                        }

                        if self.castle_flags.black_queenside && self.is_legal_move(Move::CastleQueenside) {
                            legal_moves.push(Move::CastleQueenside);
                        }
                    }
                }

                if let Some(origin_piece) = self.state[r_index][f_index] {
                    if origin_piece.color == self.active_turn {
                        let piece_direction_offsets = origin_piece.get_direction_offsets();

                            match origin_piece.piece_kind {
                                PieceKind::Queen | PieceKind::Rook | PieceKind::Bishop => {
                                    for direction in piece_direction_offsets.iter() {
                                        for target_square in self.get_piece_path((r_index, f_index), *direction).iter() {
                                            let m = Move::PieceMove {
                                                origin_square: (r_index, f_index),
                                                target_square: (target_square.0, target_square.1),
                                                origin_piece,
                                                target_piece: self.state[target_square.0][target_square.1]
                                            };

                                            if self.is_legal_move(m) {
                                                legal_moves.push(m);
                                            }
                                        }
                                    }
                                },

                                _ => {
                                    for direction in piece_direction_offsets.iter() {
                                        let target_square = ((r_index as isize + direction.0) as usize, (f_index as isize + direction.1) as usize);

                                        if target_square.0 < BOARD_SIZE && target_square.1 < BOARD_SIZE {
                                            let m = Move::PieceMove {
                                                origin_square: (r_index, f_index),
                                                target_square,
                                                origin_piece,
                                                target_piece: self.state[target_square.0][target_square.1]
                                            };

                                            if self.is_legal_move(m) {
                                                legal_moves.push(m);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        legal_moves
    }

    pub fn move_piece(&mut self, m: Move) -> Result<(), MoveError> {
        if !self.is_legal_move(m) {
            return Err(MoveError::IllegalMoveError);
        }

        if self.active_turn == Color::Black {
            self.fullmove_number += 1;
            self.halfmove_clock += 1;
        }

        match m {
            Move::CastleKingside => self.castle_kingside(),
            Move::CastleQueenside => self.castle_queenside(),

            Move::PieceMove { origin_square, target_square, origin_piece, ..} => {
                let (origin_rank, origin_file) = origin_square;
                let (target_rank, target_file) = target_square;

                if origin_piece.piece_kind == PieceKind::Pawn {
                    let rank_offset = target_rank.abs_diff(origin_rank);

                    if self.state[target_rank][target_file].is_some() || rank_offset < 2 { self.en_passant_target_square = None; }

                    if rank_offset == 2 {
                        self.en_passant_target_square = match self.active_turn {
                            Color::White => Some((target_rank + 1, target_file)),
                            Color::Black => Some((target_rank - 1, target_file))
                        }
                    }

                    self.halfmove_clock = 0;
                } else{ self.en_passant_target_square = None }

                if self.state[target_rank][target_file].is_some() { self.halfmove_clock = 0; }

                let piece = self.state[origin_rank][origin_file].take();

                self.state[target_rank][target_file] = piece;

                if origin_piece.piece_kind == PieceKind::Pawn {
                    if target_rank == 7 {
                        self.state[target_rank][target_file] = Some(Piece::from_char('q'));
                    } else if target_rank == 0 {
                        self.state[target_rank][target_file] = Some(Piece::from_char('Q'));
                    }
                }

                if origin_piece.piece_kind == PieceKind::King {
                    match self.active_turn {
                        Color::White => {
                            self.castle_flags.white_kingside = false;
                            self.castle_flags.white_queenside = false;

                            self.white_king_position = Some(target_square);
                        },

                        Color::Black => {
                            self.castle_flags.black_kingside = false;
                            self.castle_flags.black_queenside = false;

                            self.black_king_position = Some(target_square);
                        }
                    }
                }
                
                if origin_piece.piece_kind == PieceKind::Rook {
                    match origin_square {
                        WHITE_KINGSIDE_ROOK_POS => self.castle_flags.white_kingside = false,
                        WHITE_QUEENSIDE_ROOK_POS => self.castle_flags.white_queenside = false,

                        BLACK_KINGSIDE_ROOK_POS => self.castle_flags.black_kingside = false,
                        BLACK_QUEENSIDE_ROOK_POS => self.castle_flags.black_queenside = false,
                        _ => ()
                    }
                }

                self.active_turn = match self.active_turn {
                    Color::White => Color::Black,
                    Color::Black => Color::White
                };
            }
        }

        self.history.push(m);

        Ok(())
    }

    pub fn is_square_attacked(&self, position: Position) -> bool {
        let (px, py) = position;

        for (r_index, rank) in self.state.iter().enumerate() {
            for (f_index, piece) in rank.iter().enumerate() {
                    if let Some(p) = piece {
                        if p.color == self.active_turn { continue; }
                        match p.piece_kind {
                            PieceKind::Pawn => {
                                if r_index > 0 && f_index > 0 {
                                    let r_offset = r_index as i32 - px as i32;
                                    let f_offset = f_index as i32 - py as i32;

                                    if (p.color == Color::White && px > r_index) ||
                                       (p.color == Color::Black && px < r_index) {
                                           return false;
                                    }

                                    if (r_offset.abs() == 1) && (f_offset.abs() == 1) { return true; }
                               }
                            },

                            PieceKind::Knight => {
                                if r_index > 0 && f_index > 0 {
                                    let r_offset = r_index as i32 - px as i32;
                                    let f_offset = f_index as i32 - py as i32;

                                    if ((r_offset.abs() == 2) && (f_offset.abs() == 1)) ||
                                        ((r_offset.abs() == 1) && (f_offset.abs() == 2)) {
                                            return true;
                                        }
                                    }
                            },

                            PieceKind::Bishop => {
                                if self.is_diagonal_clear((r_index, f_index), position) {
                                    return true;
                                }
                            },

                            PieceKind::Rook => {
                                if self.is_straight_clear((r_index, f_index), position) {
                                    return true;
                                }
                            },

                            PieceKind::Queen => {
                                if self.is_diagonal_clear((r_index, f_index), position) ||
                                    self.is_straight_clear((r_index, f_index), position) {
                                        return true;
                                }
                            },

                            PieceKind::King => {
                                let r_offset = (r_index as i32 - px  as i32).abs();
                                let f_offset = (f_index as i32 - py as i32).abs();

                                if (r_offset <= 1) && (f_offset <= 1) {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        false
    }

    pub fn is_legal_move(&self, m: Move) -> bool {
        let has_moved_king = self.history.iter().any(|&m| {
            match m {
                Move::PieceMove { origin_piece, .. } => !(origin_piece.piece_kind == PieceKind::King && origin_piece.color == self.active_turn),
                _ => false
            }
        });

        match m {
            Move::CastleKingside => {
                if !has_moved_king { return false; }

                match self.active_turn {
                    Color::White => {
                        if !self.castle_flags.white_kingside || !self.is_straight_clear(WHITE_KING_POS, WHITE_KINGSIDE_ROOK_POS) {
                            return false;
                        }
                    },

                    Color::Black => {
                        if !self.castle_flags.black_kingside || !self.is_straight_clear(BLACK_KING_POS, BLACK_KINGSIDE_ROOK_POS) {
                            return false;
                        }
                    },
                }
            },

            Move::CastleQueenside => {
                if !has_moved_king { return false; }

                match self.active_turn {
                    Color::White => {
                        if !self.castle_flags.white_queenside || !self.is_straight_clear(WHITE_KING_POS, WHITE_QUEENSIDE_ROOK_POS) {
                            return false;
                        }
                    },

                    Color::Black => {
                        if !self.castle_flags.black_queenside || !self.is_straight_clear(BLACK_KING_POS, BLACK_QUEENSIDE_ROOK_POS) {
                            return false;
                        }
                    }
                }
            },

            Move::PieceMove { origin_square, target_square, ..} => {
                let (origin_rank, origin_file) = origin_square;
                let (target_rank, target_file) = target_square;

                match self.state[origin_rank][origin_file] {
                    Some(piece) if piece.color != self.active_turn => { return false; },
                    None => { return false; }
                    _ => ()
                };

                if let Some(piece) = self.state[target_rank][target_file] {
                    if piece.color == self.active_turn || piece.piece_kind == PieceKind::King {
                        return false;
                    }
                }

                let piece = match self.state[origin_rank][origin_file] {
                    Some(piece) => piece,
                    _ => { return false; }
                };

                let piece_direction_offsets = piece.get_direction_offsets();

                match self.active_turn {
                    Color::White => {
                        let king_pos = self.white_king_position.unwrap();
                        let mut board_copy = self.clone();

                        let piece = match board_copy.state[origin_rank][origin_file].take() {
                            Some(piece) => piece,
                            _ => { return false; }
                        };

                        board_copy.state[target_rank][target_file] = Some(piece);

                        if piece.piece_kind == PieceKind::King {
                            if board_copy.is_square_attacked((target_rank, target_file)) {
                                return false;
                            } else { return true; }
                        }

                        if board_copy.is_square_attacked(king_pos) { return false; }
                   },

                    Color::Black => {
                        let king_pos = self.black_king_position.unwrap();
                        let mut board_copy = self.clone();

                        let piece = match board_copy.state[origin_rank][origin_file].take() {
                            Some(piece) => piece,
                            _ => { return false; }
                        };

                        if piece.piece_kind == PieceKind::King {
                            if board_copy.is_square_attacked((target_rank, target_file)) {
                                return false;
                            } else { return true; }
                        }

                        board_copy.state[target_rank][target_file] = Some(piece);

                        if board_copy.is_square_attacked(king_pos) { return false; }
                    }
                }

                match piece.piece_kind {
                    PieceKind::Pawn => {
                        let rank_offset = target_rank.abs_diff(origin_rank);
                        let file_offset = target_file.abs_diff(origin_file);

                        if (piece.color == Color::White && target_rank > origin_rank) ||
                        (piece.color == Color::Black && target_rank < origin_rank) {
                            return false;
                        }

                        if rank_offset == 1 && file_offset == 1 {
                            if let Some(pos) = self.en_passant_target_square {
                                if (target_rank, target_file) == pos {
                                    return true;
                                }
                            }

                            return self.state[target_rank][target_file].is_some();
                        }

                        if self.state[target_rank][target_file].is_some() { return false; }

                        if rank_offset == 2 {
                            if origin_rank == 6 || origin_rank == 1 {
                                return match piece.color {
                                    Color::White if self.state[target_rank + 1][target_file].is_some() => false,
                                    Color::Black if self.state[target_rank - 1][target_file].is_some() => false,

                                    _ => true
                                };
                            } else { return false; }
                        }
                    },

                    PieceKind::Knight => {
                        let rank_offset = if target_rank > origin_rank { target_rank - origin_rank } else { origin_rank - target_rank } as isize;
                        let file_offset = if target_file > origin_file { target_file - origin_file } else { origin_file - target_file } as isize;

                        if !piece_direction_offsets.iter().any(|&d| d == (rank_offset, file_offset)) {
                            return false;
                        }
                    },

                    PieceKind::King => {
                        let rank_offset = target_rank.abs_diff(origin_rank) as isize;
                        let file_offset = target_rank.abs_diff(origin_file) as isize;

                        if !piece_direction_offsets.iter().any(|&d| d == (rank_offset, file_offset)) { return false; }
                    },

                    PieceKind::Bishop => {
                        if !self.is_diagonal_clear(origin_square, target_square) { return false; }
                    },

                    PieceKind::Rook => {
                        if !self.is_straight_clear(origin_square, target_square) { return false; }
                    },

                    PieceKind::Queen => {
                        if !self.is_diagonal_clear(origin_square, target_square) &&
                            !self.is_straight_clear(origin_square, target_square) {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }

    fn get_piece_path(&self, start: Position, direction: (isize, isize)) -> Vec<Position> {
        let mut positions = Vec::new();

        let mut r_index = (start.0 as isize + direction.0) as usize;
        let mut f_index = (start.1 as isize + direction.1) as usize;

        while (r_index < BOARD_SIZE) && (f_index < BOARD_SIZE) {
            if let Some(piece) = self.state[r_index][f_index] {
                if piece.color != self.active_turn {
                    positions.push((r_index, f_index));
                    return positions;
                }
            }

            positions.push((r_index, f_index));

            r_index = (r_index as isize + direction.0) as usize;
            f_index = (f_index as isize + direction.1) as usize;
        }
        positions
    }

    fn is_diagonal_clear(&self, start: Position, end: Position) -> bool {
        let (start_rank, start_file) = start;
        let (end_rank, end_file) = end;

        let r_offset = (end_rank as i32 - start_rank as i32).abs();
        let f_offset = (end_file as i32 - start_file as i32).abs();

        if r_offset != f_offset { return false; }

        let r_step = if end_rank > start_rank { 1 } else { -1 };
        let f_step = if end_file > start_file { 1 } else { -1 };

        let mut r_index = (start_rank as i32 + r_step) as usize;
        let mut f_index = (start_file as i32 + f_step) as usize;

        while (r_index != end_rank && f_index != end_file) && !(r_index > BOARD_SIZE || f_index > BOARD_SIZE) {
            if self.state[r_index][f_index].is_some() { return false; }

            r_index = (r_index as i32 + r_step) as usize;
            f_index = (f_index as i32 + f_step) as usize;
        }

        true
    }

    fn is_straight_clear(&self, start: Position, end: Position) -> bool {
        let (start_rank, start_file) = start;
        let (end_rank, end_file) = end;

        if start_rank == end_rank {
            let f_step = if end_file > start_file { 1 } else { -1 };
            let mut f_index = (start_file as i32 + f_step) as usize;

            while f_index != end_file && f_index < BOARD_SIZE {
                if self.state[start_rank][f_index].is_some() { return false; }

                f_index = (f_index as i32 + f_step) as usize;
            }
        } else if start_file == end_file {
            let r_step = if end_rank > start_rank { 1 } else { -1 };
            let mut r_index = (start_rank as i32 + r_step) as usize;

            while r_index != end_rank && r_index < BOARD_SIZE {
                if self.state[r_index][start_file].is_some() { return false; }

                r_index = (r_index as i32 + r_step) as usize;
            }
        } else { return false; }

        true
    }

    pub fn from_fen(fen: &str) -> Result<Self, ParseFenError> {
        let mut fen_fields = Vec::with_capacity(6);

        if fen == "start" {
            for field in START_FEN.split_whitespace() {
                fen_fields.push(field);
            }
        }

        for field in fen.split_whitespace() {
            fen_fields.push(field);
        }

        let squares = parse_ranks(fen_fields[0]);
        let active_turn = parse_active_turn(fen_fields[1])?;
        let mut castle_flags = CastleFlags::parse_castle_flags(fen_fields[2]);
        let en_passant_target_square = parse_en_passant_target_square(fen_fields[3])?;
        let halfmove_clock  = fen_fields[4].parse::<u8>().unwrap_or(0);
        let fullmove_number = fen_fields[5].parse::<u16>().unwrap_or(0);
        let history: Vec<Move> = Vec::new();

        let mut white_king_position: Option<Position> = None;
        let mut black_king_position: Option<Position> = None;
        let mut white_kingside_rook_position: Option<Position> = None;
        let mut white_queenside_rook_position: Option<Position> = None;
        let mut black_kingside_rook_position: Option<Position> = None;
        let mut black_queenside_rook_position: Option<Position> = None;

        for (r_index, rank) in squares.iter().enumerate() {
            for (f_index, piece) in rank.iter().enumerate() {
                if let Some(p) = piece {
                    if p.piece_kind == PieceKind::King {
                        if p.color == Color::White {
                            white_king_position = Some((r_index, f_index));
                        } else if p.color == Color::Black {
                            black_king_position = Some((r_index, f_index));
                        }
                    }

                    if p.piece_kind == PieceKind::Rook {
                        match p.color {
                            Color::White => match (r_index, f_index) {
                                WHITE_KINGSIDE_ROOK_POS => white_kingside_rook_position = Some(WHITE_KINGSIDE_ROOK_POS),
                                WHITE_QUEENSIDE_ROOK_POS => white_queenside_rook_position = Some(WHITE_QUEENSIDE_ROOK_POS),

                                _ => {
                                    castle_flags.white_kingside = false;
                                    castle_flags.white_queenside = false;
                                }
                            }

                            Color::Black => match (r_index, f_index) {
                                BLACK_KINGSIDE_ROOK_POS => black_kingside_rook_position = Some(BLACK_KINGSIDE_ROOK_POS),
                                BLACK_QUEENSIDE_ROOK_POS => black_queenside_rook_position = Some(BLACK_QUEENSIDE_ROOK_POS),

                                _ => {
                                    castle_flags.black_kingside = false;
                                    castle_flags.black_queenside = false;
                                }
                            }
                        }
                    }
                }
            }
        }

        if white_king_position.is_none() || black_king_position.is_none() {
            panic!("{}", ParseFenError::MissingKing)
        }

        Ok(Self {
            state: squares,
            active_turn,
            castle_flags,
            en_passant_target_square,
            halfmove_clock,
            fullmove_number,
            history,
            white_king_position,
            black_king_position,
            white_kingside_rook_position,
            white_queenside_rook_position,
            black_kingside_rook_position,
            black_queenside_rook_position
        })
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        let active_turn = match self.active_turn {
            Color::White => 'w',
            Color::Black => 'b',
        };

        for rank in self.state {
            let mut empty_squares = 0;

            for square in rank {
                match square {
                    Some(piece) => {
                        if empty_squares > 0 {
                            fen.push_str(&empty_squares.to_string());
                            empty_squares = 0;
                        }
                        fen.push(piece.to_char());
                    },
                    None => empty_squares += 1,
                }
            }

            if empty_squares > 0 {
                fen.push_str(&empty_squares.to_string());
            }
            fen.push('/');
        }

        fen.pop();
        fen.push(' ');

        fen.push(active_turn);
        fen.push(' ');

        if self.castle_flags.white_kingside { fen.push('K'); }
        if self.castle_flags.white_queenside { fen.push('Q') }
        if self.castle_flags.black_kingside { fen.push('k'); }
        if self.castle_flags.black_queenside { fen.push('q'); }

        if !self.castle_flags.white_kingside && !self.castle_flags.white_queenside && !self.castle_flags.black_kingside && !self.castle_flags.black_queenside {
            fen.push('-');
        }

        fen.push(' ');

        if let Some((rank, file)) = self.en_passant_target_square {
            fen.push((file as u8 + 97) as char);
            fen.push((8 - rank).to_string().chars().next().unwrap())
        } else {
            fen.push('-');
        }

        fen.push(' ');

        fen.push_str(&self.halfmove_clock.to_string());
        fen.push(' ');
        fen.push_str(&self.fullmove_number.to_string());

        fen
    }
}

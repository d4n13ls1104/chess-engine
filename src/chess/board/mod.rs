use image::{imageops, Rgba, RgbaImage};
use std::env;

use self::chess_move::Move;
use self::chess_move::MoveError;

pub use self::fen::*;
pub use self::piece::*;

pub mod chess_move;
pub mod fen;
pub mod piece;

const BOARD_SIZE: usize = 8;
const IMAGE_SIZE: u32 = 512;

const WHITE_KING_ORIGIN: Position = (7, 4);
const BLACK_KING_ORIGIN: Position = (0, 4);

const WHITE_KINGSIDE_ROOK_ORIGIN: Position = (7, 7);
const WHITE_QUEENSIDE_ROOK_ORIGIN: Position = (7, 0);
const BLACK_KINGSIDE_ROOK_ORIGIN: Position = (0, 7);
const BLACK_QUEENSIDE_ROOK_ORIGIN: Position = (0, 0);

const BOARD_FOREGROUND_COLOR: Rgba<u8> = Rgba([181, 136, 99, u8::MAX]);
const BOARD_BACKGROUND_COLOR: Rgba<u8> = Rgba([240, 217, 181, u8::MAX]);

pub type Position = (usize, usize);

#[derive(PartialEq)]
pub enum GameStatus {
    WhiteWin,
    BlackWin,
    Draw,
    Ongoing,
}

#[derive(Clone)]
pub struct Board {
    pub state: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE],
    pub active_turn: Color,
    pub castle_flags: CastleFlags,
    pub en_passant_square: Option<Position>,
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
                } else {
                    BOARD_FOREGROUND_COLOR
                };

                let square_size = IMAGE_SIZE / BOARD_SIZE as u32;

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
                        Err(e) => panic!("Failed to get program directory: {}", e),
                    };

                    let color_flag = if piece.color == Color::White {
                        'w'
                    } else {
                        'b'
                    };
                    let file_name =
                        format!("{}_{:?}.png", color_flag, piece.piece_kind).to_lowercase();

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

        let opponent_attacks = match self.active_turn {
            Color::White => self.get_attacks_for_side(Color::Black),
            Color::Black => self.get_attacks_for_side(Color::White),
        };

        if legal_moves.is_empty() {
            if self.halfmove_clock >= 100 {
                return GameStatus::Draw;
            }

            match self.active_turn {
                Color::White => {
                    let king_pos = self.white_king_position.unwrap();

                    if opponent_attacks.iter().any(|&pos| pos == king_pos) {
                        return GameStatus::BlackWin;
                    }
                    GameStatus::Draw
                }

                Color::Black => {
                    let king_pos = self.black_king_position.unwrap();

                    if opponent_attacks.iter().any(|&pos| pos == king_pos) {
                        return GameStatus::WhiteWin;
                    }
                    GameStatus::Draw
                }

                _ => GameStatus::Draw,
            }
        } else {
            GameStatus::Ongoing
        }
    }

    fn get_attacks_for_side(&self, side: Color) -> Vec<Position> {
        let mut positions = Vec::new();

        for r_index in 0..8 {
            for f_index in 0..8 {
                if let Some(piece) = self.state[r_index][f_index] {
                    if piece.color == side {
                        let mut piece_attacks =
                            piece.get_attack_positions((r_index, f_index), &self);

                        positions.append(&mut piece_attacks);
                    }
                }
            }
        }

        positions
    }

    pub fn move_piece(&mut self, m: Move) -> Result<(), MoveError> {
        if !self.is_legal_move(m) {
            println!("Illegal move: {:?}", m);
            println!();
            return Err(MoveError::IllegalMoveError);
        }

        if self.active_turn == Color::Black {
            self.fullmove_number += 1;
            self.halfmove_clock += 1;
        }

        match m {
            Move::CastleKingside => self.castle_kingside(),
            Move::CastleQueenside => self.castle_queenside(),

            Move::PieceMove {
                origin_square,
                target_square,
                origin_piece,
                ..
            } => {
                let (origin_rank, origin_file) = origin_square;
                let (target_rank, target_file) = target_square;

                if origin_piece.piece_kind == PieceKind::Pawn {
                    let is_double_push = target_rank.abs_diff(origin_rank) == 2;

                    if is_double_push {
                        self.en_passant_square = match self.active_turn {
                            Color::White => Some((target_rank + 1, target_file)),
                            Color::Black => Some((target_rank - 1, target_file)),
                        }
                    }

                    self.halfmove_clock = 0;
                } else {
                    self.en_passant_square = None;
                }

                if self.state[target_rank][target_file].is_some() {
                    self.halfmove_clock = 0;
                }

                let piece = self.state[origin_rank][origin_file].take();

                self.state[target_rank][target_file] = piece;

                if origin_piece.piece_kind == PieceKind::Pawn {
                    match target_rank {
                        7 => self.state[target_rank][target_file] = Some(Piece::from_char('q')),
                        0 => self.state[target_rank][target_file] = Some(Piece::from_char('Q')),
                        _ => (),
                    }
                }

                if origin_piece.piece_kind == PieceKind::King {
                    match self.active_turn {
                        Color::White => {
                            self.castle_flags.white_kingside = false;
                            self.castle_flags.white_queenside = false;

                            self.white_king_position = Some(target_square);
                        }

                        Color::Black => {
                            self.castle_flags.black_kingside = false;
                            self.castle_flags.black_queenside = false;

                            self.black_king_position = Some(target_square);
                        }
                    }
                }

                if origin_piece.piece_kind == PieceKind::Rook {
                    match origin_square {
                        WHITE_KINGSIDE_ROOK_ORIGIN if origin_piece.color == Color::White => {
                            self.castle_flags.white_kingside = false
                        }
                        WHITE_QUEENSIDE_ROOK_ORIGIN if origin_piece.color == Color::White => {
                            self.castle_flags.white_queenside = false
                        }

                        BLACK_KINGSIDE_ROOK_ORIGIN if origin_piece.color == Color::Black => {
                            self.castle_flags.black_kingside = false
                        }
                        BLACK_QUEENSIDE_ROOK_ORIGIN if origin_piece.color == Color::Black => {
                            self.castle_flags.black_queenside = false
                        }
                        _ => (),
                    }
                }

                self.active_turn = match self.active_turn {
                    Color::White => Color::Black,
                    Color::Black => Color::White,
                };
            }
        }

        self.history.push(m);

        Ok(())
    }

    pub fn get_legal_moves(&self) -> Vec<Move> {
        let mut legal_moves = Vec::new();

        if self.is_legal_move(Move::CastleKingside) {
            legal_moves.push(Move::CastleKingside);
        }

        if self.is_legal_move(Move::CastleQueenside) {
            legal_moves.push(Move::CastleQueenside);
        }

        for r_index in 0..8 {
            for f_index in 0..8 {
                if let Some(piece) = self.state[r_index][f_index] {
                    if piece.color == self.active_turn {
                        let piece_attacks = piece.get_attack_positions((r_index, f_index), &self);

                        for pos in piece_attacks {
                            if let Some(target_piece) = self.state[pos.0][pos.1] {
                                if target_piece.color != self.active_turn {
                                    let attack_move = Move::PieceMove {
                                        origin_square: (r_index, f_index),
                                        target_square: pos,
                                        origin_piece: piece,
                                        target_piece: Some(target_piece),
                                    };

                                    if self.is_legal_move(attack_move) {
                                        legal_moves.push(attack_move);
                                    }
                                }
                            } else {
                                if piece.piece_kind != PieceKind::Pawn {
                                    let m = Move::PieceMove {
                                        origin_square: (r_index, f_index),
                                        target_square: pos,
                                        origin_piece: piece,
                                        target_piece: None,
                                    };

                                    if self.is_legal_move(m) {
                                        legal_moves.push(m);
                                    }
                                }
                            }
                        }

                        if piece.piece_kind == PieceKind::Pawn {
                            match piece.color {
                                Color::White => {
                                    if r_index == 6 {
                                        let path_is_clear = self.state[r_index - 1][f_index]
                                            .is_none()
                                            && self.state[r_index - 2][f_index].is_none();

                                        if path_is_clear {
                                            let double_pawn_push = Move::PieceMove {
                                                origin_square: (r_index, f_index),
                                                target_square: (r_index - 2, f_index),
                                                origin_piece: piece,
                                                target_piece: None,
                                            };

                                            if self.is_legal_move(double_pawn_push) {
                                                legal_moves.push(double_pawn_push);
                                            }
                                        }
                                    }

                                    if self.state[r_index - 1][f_index].is_none() {
                                        let single_pawn_push = Move::PieceMove {
                                            origin_square: (r_index, f_index),
                                            target_square: (r_index - 1, f_index),
                                            origin_piece: piece,
                                            target_piece: None,
                                        };

                                        if self.is_legal_move(single_pawn_push) {
                                            legal_moves.push(single_pawn_push);
                                        }
                                    }
                                }

                                Color::Black => {
                                    if r_index == 1 {
                                        let path_is_clear = self.state[r_index + 1][f_index]
                                            .is_none()
                                            && self.state[r_index + 2][f_index].is_none();

                                        if path_is_clear {
                                            let double_pawn_push = Move::PieceMove {
                                                origin_square: (r_index, f_index),
                                                target_square: (r_index + 2, f_index),
                                                origin_piece: piece,
                                                target_piece: None,
                                            };

                                            if self.is_legal_move(double_pawn_push) {
                                                legal_moves.push(double_pawn_push);
                                            }
                                        }
                                    }

                                    if self.state[r_index + 1][f_index].is_none() {
                                        let single_pawn_push = Move::PieceMove {
                                            origin_square: (r_index, f_index),
                                            target_square: (r_index + 1, f_index),
                                            origin_piece: piece,
                                            target_piece: None,
                                        };

                                        if self.is_legal_move(single_pawn_push) {
                                            legal_moves.push(single_pawn_push);
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

    pub fn is_legal_move(&self, m: Move) -> bool {
        let has_moved_king = self.history.iter().any(|&m| match m {
            Move::PieceMove { origin_piece, .. } => {
                (origin_piece.piece_kind == PieceKind::King
                    && origin_piece.color == self.active_turn)
            }
            _ => false,
        });

        match m {
            Move::CastleKingside => {
                if has_moved_king {
                    return false;
                }

                if self.active_turn == Color::White {
                    if !self.castle_flags.white_kingside {
                        return false;
                    }

                    let sq_1 = self.state[WHITE_KING_ORIGIN.0][WHITE_KING_ORIGIN.1 + 1];
                    let sq_2 = self.state[WHITE_KING_ORIGIN.0][WHITE_KING_ORIGIN.1 + 2];

                    let path_is_clear = sq_1.is_none() && sq_2.is_none();

                    if !path_is_clear {
                        return false;
                    }
                } else {
                    if !self.castle_flags.black_kingside {
                        return false;
                    }

                    let sq_1 = self.state[BLACK_KING_ORIGIN.0][BLACK_KING_ORIGIN.1 + 1];
                    let sq_2 = self.state[BLACK_KING_ORIGIN.0][BLACK_KING_ORIGIN.1 + 2];

                    let path_is_clear = sq_1.is_none() && sq_2.is_none();

                    if !path_is_clear {
                        return false;
                    }
                }
            }

            Move::CastleQueenside => {
                if has_moved_king {
                    return false;
                }

                if self.active_turn == Color::White {
                    if !self.castle_flags.white_queenside {
                        return false;
                    }

                    let sq_1 = self.state[WHITE_KING_ORIGIN.0][WHITE_KING_ORIGIN.1 - 1];
                    let sq_2 = self.state[WHITE_KING_ORIGIN.0][WHITE_KING_ORIGIN.1 - 2];
                    let sq_3 = self.state[WHITE_KING_ORIGIN.0][WHITE_KING_ORIGIN.1 - 3];

                    let path_is_clear = sq_1.is_none() && sq_2.is_none() && sq_3.is_none();

                    if !path_is_clear {
                        return false;
                    }
                } else {
                    if !self.castle_flags.black_queenside {
                        return false;
                    }

                    let sq_1 = self.state[BLACK_KING_ORIGIN.0][BLACK_KING_ORIGIN.1 - 1];
                    let sq_2 = self.state[BLACK_KING_ORIGIN.0][BLACK_KING_ORIGIN.1 - 2];
                    let sq_3 = self.state[BLACK_KING_ORIGIN.0][BLACK_KING_ORIGIN.1 - 3];

                    let path_is_clear = sq_1.is_none() && sq_2.is_none() && sq_3.is_none();

                    if !path_is_clear {
                        return false;
                    }
                }
            }

            Move::PieceMove {
                origin_square,
                target_square,
                ..
            } => {
                let (origin_rank, origin_file) = origin_square;
                let (target_rank, target_file) = target_square;

                if let Some(piece) = self.state[origin_rank][origin_file] {
                    if piece.color != self.active_turn {
                        return false;
                    }
                } else {
                    return false;
                }

                if let Some(piece) = self.state[target_rank][target_file] {
                    if piece.color == self.active_turn || piece.piece_kind == PieceKind::King {
                        return false;
                    }
                }

                let opponent_color = match self.active_turn {
                    Color::White => Color::Black,
                    Color::Black => Color::White,
                };

                match self.active_turn {
                    Color::White => {
                        let mut board_copy = self.clone();
                        let king_pos = self.white_king_position.unwrap();

                        let piece = board_copy.state[origin_rank][origin_file].take();

                        board_copy.state[target_rank][target_file] = piece;

                        let opponent_attacks = board_copy.get_attacks_for_side(opponent_color);

                        if piece.unwrap().piece_kind == PieceKind::King {
                            let king_target_position_attacked =
                                opponent_attacks.iter().any(|&pos| pos == target_square);

                            if king_target_position_attacked {
                                return false;
                            }
                        } else {
                            let king_position_attacked =
                                opponent_attacks.iter().any(|&pos| pos == king_pos);

                            if king_position_attacked {
                                return false;
                            }
                        }
                    }

                    Color::Black => {
                        let mut board_copy = self.clone();
                        let king_pos = self.black_king_position.unwrap();

                        let piece = board_copy.state[origin_rank][origin_file].take();

                        board_copy.state[target_rank][target_file] = piece;

                        let opponent_attacks = board_copy.get_attacks_for_side(opponent_color);

                        if piece.unwrap().piece_kind == PieceKind::King {
                            let king_target_position_attacked =
                                opponent_attacks.iter().any(|&pos| pos == target_square);

                            if king_target_position_attacked {
                                return false;
                            }
                        } else {
                            let king_position_attacked =
                                opponent_attacks.iter().any(|&pos| pos == king_pos);

                            if king_position_attacked {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        true
    }

    pub fn castle_kingside(&mut self) {
        match self.active_turn {
            Color::White => {
                let king = self.state[WHITE_KING_ORIGIN.0][WHITE_KING_ORIGIN.1].take();
                let rook =
                    self.state[WHITE_KINGSIDE_ROOK_ORIGIN.0][WHITE_KINGSIDE_ROOK_ORIGIN.1].take();

                self.state[WHITE_KING_ORIGIN.0][WHITE_KING_ORIGIN.1 + 2] = king;
                self.state[WHITE_KINGSIDE_ROOK_ORIGIN.0][WHITE_KINGSIDE_ROOK_ORIGIN.1 - 2] = rook;

                self.castle_flags.white_kingside = false;
                self.castle_flags.white_queenside = false;
                self.active_turn = Color::Black;
            }
            Color::Black => {
                let king = self.state[BLACK_KING_ORIGIN.0][BLACK_KING_ORIGIN.1].take();
                let rook =
                    self.state[BLACK_KINGSIDE_ROOK_ORIGIN.0][BLACK_KINGSIDE_ROOK_ORIGIN.1].take();

                self.state[BLACK_KING_ORIGIN.0][BLACK_KING_ORIGIN.1 + 2] = king;
                self.state[BLACK_KINGSIDE_ROOK_ORIGIN.0][BLACK_KINGSIDE_ROOK_ORIGIN.1 - 2] = rook;

                self.castle_flags.black_kingside = false;
                self.castle_flags.black_queenside = false;
                self.active_turn = Color::White;
            }
        }
    }

    pub fn castle_queenside(&mut self) {
        match self.active_turn {
            Color::White => {
                let king = self.state[WHITE_KING_ORIGIN.0][WHITE_KING_ORIGIN.1].take();
                let rook =
                    self.state[WHITE_QUEENSIDE_ROOK_ORIGIN.0][WHITE_QUEENSIDE_ROOK_ORIGIN.1].take();

                self.state[WHITE_KING_ORIGIN.0][WHITE_KING_ORIGIN.1 - 2] = king;
                self.state[WHITE_QUEENSIDE_ROOK_ORIGIN.0][WHITE_QUEENSIDE_ROOK_ORIGIN.1 + 3] = rook;

                self.castle_flags.white_kingside = false;
                self.castle_flags.white_queenside = false;
                self.active_turn = Color::Black;
            }
            Color::Black => {
                let king = self.state[BLACK_KING_ORIGIN.0][BLACK_KING_ORIGIN.1].take();
                let rook =
                    self.state[BLACK_QUEENSIDE_ROOK_ORIGIN.0][BLACK_QUEENSIDE_ROOK_ORIGIN.1].take();

                self.state[BLACK_KING_ORIGIN.0][BLACK_KING_ORIGIN.1 - 2] = king;
                self.state[BLACK_QUEENSIDE_ROOK_ORIGIN.0][BLACK_QUEENSIDE_ROOK_ORIGIN.1 + 3] = rook;

                self.castle_flags.black_kingside = false;
                self.castle_flags.black_queenside = false;
                self.active_turn = Color::White;
            }
        }
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

        let ranks = parse_ranks(fen_fields[0]);
        let active_turn = parse_active_turn(fen_fields[1])?;
        let mut castle_flags = CastleFlags::parse_castle_flags(fen_fields[2]);
        let en_passant_square = parse_en_passant_target_square(fen_fields[3])?;
        let halfmove_clock = fen_fields[4].parse::<u8>().unwrap_or(0);
        let fullmove_number = fen_fields[5].parse::<u16>().unwrap_or(0);
        let history: Vec<Move> = Vec::new();

        let mut white_king_position: Option<Position> = None;
        let mut black_king_position: Option<Position> = None;
        let mut white_kingside_rook_position: Option<Position> = None;
        let mut white_queenside_rook_position: Option<Position> = None;
        let mut black_kingside_rook_position: Option<Position> = None;
        let mut black_queenside_rook_position: Option<Position> = None;

        for (r_index, rank) in ranks.iter().enumerate() {
            for (f_index, square) in rank.iter().enumerate() {
                if let Some(piece) = square {
                    if piece.piece_kind == PieceKind::King {
                        match piece.color {
                            Color::White => white_king_position = Some((r_index, f_index)),
                            Color::Black => black_king_position = Some((r_index, f_index)),
                        }
                    }

                    if piece.piece_kind == PieceKind::Rook {
                        match piece.color {
                            Color::White => match (r_index, f_index) {
                                WHITE_KINGSIDE_ROOK_ORIGIN => {
                                    white_kingside_rook_position = Some(WHITE_KINGSIDE_ROOK_ORIGIN)
                                }

                                WHITE_QUEENSIDE_ROOK_ORIGIN => {
                                    white_queenside_rook_position =
                                        Some(WHITE_QUEENSIDE_ROOK_ORIGIN)
                                }

                                _ => (),
                            },

                            Color::Black => match (r_index, f_index) {
                                BLACK_KINGSIDE_ROOK_ORIGIN => {
                                    black_kingside_rook_position = Some(BLACK_KINGSIDE_ROOK_ORIGIN)
                                }

                                BLACK_QUEENSIDE_ROOK_ORIGIN => {
                                    black_queenside_rook_position =
                                        Some(BLACK_QUEENSIDE_ROOK_ORIGIN)
                                }

                                _ => (),
                            },
                        }
                    }
                }
            }
        }

        if white_king_position.is_none() {
            castle_flags.white_kingside = false;
            castle_flags.white_queenside = false;
        }

        if black_king_position.is_none() {
            castle_flags.black_kingside = false;
            castle_flags.black_queenside = false;
        }

        if white_kingside_rook_position.is_none() {
            castle_flags.white_kingside = false;
        }

        if white_queenside_rook_position.is_none() {
            castle_flags.white_queenside = false;
        }

        if black_kingside_rook_position.is_none() {
            castle_flags.black_kingside = false;
        }

        if black_queenside_rook_position.is_none() {
            castle_flags.black_queenside = false;
        }

        if white_king_position.is_none() || black_king_position.is_none() {
            panic!("{}", ParseFenError::MissingKing)
        }

        Ok(Self {
            state: ranks,
            active_turn,
            castle_flags,
            en_passant_square,
            halfmove_clock,
            fullmove_number,
            history,
            white_king_position,
            black_king_position,
            white_kingside_rook_position,
            white_queenside_rook_position,
            black_kingside_rook_position,
            black_queenside_rook_position,
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

            for piece in rank {
                match piece {
                    Some(p) => {
                        if empty_squares > 0 {
                            fen.push_str(&empty_squares.to_string());
                            empty_squares = 0;
                        }

                        fen.push(p.to_char());
                    }
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

        if self.castle_flags.white_kingside {
            fen.push('K');
        }
        if self.castle_flags.white_queenside {
            fen.push('Q');
        }
        if self.castle_flags.black_kingside {
            fen.push('k');
        }
        if self.castle_flags.black_queenside {
            fen.push('q');
        }

        if !self.castle_flags.white_kingside
            && !self.castle_flags.white_queenside
            && !self.castle_flags.black_kingside
            && !self.castle_flags.black_queenside
        {
            fen.push('-');
        }

        fen.push(' ');

        if let Some((rank, file)) = self.en_passant_square {
            fen.push((file as u8 + 97) as char);
            fen.push((8 - rank).to_string().chars().next().unwrap());
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

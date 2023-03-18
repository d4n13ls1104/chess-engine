use super::{
    Position,
    piece::{Piece, PieceKind},
    Board,
    PieceKind
};

#[derive(Debug, Copy, Clone)]
pub enum Move {
    CastleKingside,
    CastleQueenside,

    PieceMove {
        origin_square: Position,
        target_square: Position,
        origin_piece: Piece,
        target_piece: Option<Piece>
    }
}

impl Move { 
    pub fn from_lan(board: &Board, lan: &str) -> Result<Self, MoveError> {
        if lan.len() != 4 {
            return Err(MoveError::MalformedLANString(lan.to_string()));
        }

        match lan.to_uppercase().as_str() {
            "O-O" => { return Ok(Self::CastleKingside); },
            "O-O-O" => { return Ok(Self::CastleQueenside); }
            _ => ()
        }

        let (origin_square, target_square) = lan.split_at(2);
        
        let origin_rank = 8 - origin_square.chars().nth(1).unwrap() as usize - 48;
        let origin_file = origin_square.chars().next().unwrap() as usize - 97;

        let target_rank = 8 - target_square.chars().nth(1).unwrap() as usize - 48;
        let target_file = target_square.chars().next().unwrap() as usize - 97;

        let origin_piece = board.state[origin_rank][origin_file].unwrap();
        let target_piece = board.state[target_rank][target_file];

        Ok(Self::PieceMove {
            origin_square: (origin_rank, origin_file),
            target_square: (target_rank, target_file),
            origin_piece,
            target_piece
        })
    }

    pub fn to_str(self) -> String {
        match self {
            Self::CastleKingside => String::from("O-O"),
            Self::CastleQueenside => String::from("O-O-O"),

            Self::PieceMove { origin_square, target_square, origin_piece, target_piece } => {
                let mut str = String::new();
                let piece_char = origin_piece.to_char().to_uppercase().to_string();

                let origin_file = (origin_square.1 as u8 + 97) as char;

                let target_rank = (8 - target_square.0)
                    .to_string()
                    .chars()
                    .next()
                    .unwrap();

                let target_file = (target_square.1 as u8 + 97) as char;

                let rank_offset = target_square.0.abs_diff(origin_square.0);
                let file_offset = target_square.1.abs_diff(origin_square.1);

                if origin_piece.piece_kind == PieceKind::Pawn {
                    if rank_offset == 1 && file_offset == 1 {
                        str.push(origin_file);
                        str.push('x');
                        str.push(target_file);
                        str.push(target_rank);

                        return str;
                    }

                    str.push(target_file);
                    str.push(target_rank);

                    return str;
                }

                if target_piece.is_some() {
                    str.push(piece_char);
                    str.push('x');
                    str.push(target_file);
                    str.push(target_rank);

                    return str;
                }

                str.push(piece_char);
                str.push(target_file);
                str.push(target_rank);

                str
            }
        }
    }
}

#[derive(Debug)]
pub enum MoveError {
    MalformedLANString(String),
    IllegalMoveError,
}

impl std::fmt::Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::MalformedLANString(lan) => {
                write!(f, "Invalid LAN String: '{lan}'")
            },

            Self::IllegalMoveError => {
                write!(f, "Illegal move.")
            } 
       }
    }
}

impl std::error::Error for MoveError {}

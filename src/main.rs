mod chess;
use chess::board::Board;

fn main() {
    let mut board = match Board::from_fen("start") {
        Ok(b) => b,
        Err(err) => panic!("Error initializing board: {err}")
    };

    let m = board.get_legal_moves()[0];
    board.move_piece(m).expect("Illegal move");

    println!("{}", board.to_fen());
}

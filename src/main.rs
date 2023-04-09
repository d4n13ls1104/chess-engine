#[allow(dead_code)]
#[allow(unused)]
mod chess;

use chess::board::Board;
use rand::{thread_rng, Rng};
use tungstenite::{connect, Message};
use url::Url;

fn main() {
    let mut board = match Board::from_fen("start") {
        Ok(b) => b,
        Err(err) => panic!("Error initializing board: {err}"),
    };

    let (mut socket, response) = connect(Url::parse("ws://localhost:8080").unwrap())
        .expect("Failed to connect to WS server.");

    println!("Connected to ws server.");
    println!("STATUS: {}", response.status());

    let mut rng = thread_rng();

    for _ in 0..1000 {
        socket.write_message(Message::Text(board.to_fen())).unwrap();

        let legal_moves = board.get_legal_moves();
        let random_index = rng.gen_range(0..legal_moves.len());

        println!("Legal Moves: {}", legal_moves.len());

        board
            .move_piece(legal_moves[random_index])
            .expect("Failed to move piece");

        std::thread::sleep(std::time::Duration::from_millis(1500));
    }
}

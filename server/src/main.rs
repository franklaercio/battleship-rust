use std::io::{Read, Result, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;

use crate::board::Board;

mod board;

fn handle_client(mut client1: TcpStream, mut client2: TcpStream, player_board: Board, enemy_board: Board) -> Result<()> {
    writeln!(client1, "Paired with another player!")?;
    writeln!(client2, "Paired with another player!")?;

    let player_board_state = player_board.render();
    let enemy_board_state = enemy_board.render();
    let response = format!("UPDATE_BOARD;{}||{}\n", player_board_state, enemy_board_state);

    client1.write(response.as_bytes()).unwrap();
    client2.write(response.as_bytes()).unwrap();

    Ok(())
}

fn new_client(mut stream: TcpStream, player_board: Board, enemy_board: Board) -> Result<()> {
    static GUARD: Mutex<Option<TcpStream>> = Mutex::new(None);

    writeln!(stream, "Awaiting another player...")?;
    println!("Client connected: {:?}", stream.peer_addr());

    let another = {
        let mut guard = GUARD.lock().unwrap();
        if let Some(another) = guard.take() {
            another
        } else {
            *guard = Some(stream);
            return Ok(());
        }
    };

    // let mut buffer = [0; 512];
    // stream.read(&mut buffer).unwrap();
    //
    // let request = String::from_utf8_lossy(&buffer[..]);
    // let cleaned_request = request.trim();
    //
    // if cleaned_request.starts_with("PLACE_SHIP") {
    //     todo!()
    // } else if cleaned_request.starts_with("PLACE_BOMB") {
    //     todo!()
    // }

    handle_client(stream, another, player_board, enemy_board)?;

    Ok(())
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    println!("Server listening on {}", listener.local_addr().unwrap());

    let player_board = Board::new();
    let enemy_board = Board::new();

    for stream in listener.incoming() {
        if let Ok(s) = stream {
            std::thread::spawn(move || {
                let _ = new_client(s, player_board, enemy_board);
            });
        }
    }
    Ok(())
}

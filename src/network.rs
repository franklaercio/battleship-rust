use std::io::{BufReader, Result, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::thread;

use log::{info, warn};
use crate::game::{determine_winner, TOTAL_ROUNDS};

use crate::player::Player;

static WAITING: Mutex<Option<TcpStream>> = Mutex::new(None);

pub fn handle_client(mut stream: TcpStream) -> Result<()> {
    let client_ip = stream.peer_addr()?.to_string();
    info!("New client connected: {}", client_ip);
    write!(stream, "Waiting for another player...\r\n")?;
    stream.flush()?;

    let opponent = {
        let mut guard = WAITING.lock().unwrap();
        if let Some(opponent) = guard.take() {
            let opponent_ip = opponent.peer_addr()?.to_string();
            info!("Found opponent for {}: {}", client_ip, opponent_ip);
            opponent
        } else {
            info!("No opponent found for {}, waiting...", client_ip);
            *guard = Some(stream);
            return Ok(());
        }
    };

    let opponent_ip = opponent.peer_addr()?.to_string();
    let mut player1 = Player::new(BufReader::new(stream.try_clone()?), stream.try_clone()?, client_ip.clone());
    let mut player2 = Player::new(BufReader::new(opponent.try_clone()?), opponent.try_clone()?, opponent_ip.clone());

    info!("Player 1 ({}) setting up board", player1.ip);
    player1.place_ships()?;

    info!("Player 2 ({}) setting up board", player2.ip);
    player2.place_ships()?;

    for i in 0..TOTAL_ROUNDS {
        info!("Playing round {}", i + 1);
        if i % 2 == 0 {
            info!("Player 1 ({}) turn", player1.ip);
            player1.take_turn(&player2.board)?;
        } else {
            info!("Player 2 ({}) turn", player2.ip);
            player2.take_turn(&player1.board)?;
        }
    }

    determine_winner(&mut player1, &mut player2)
}

pub fn start_server(address: &str) -> Result<()> {
    let listener = TcpListener::bind(address)?;
    info!("Server started at {}", address);

    for stream in listener.incoming() {
        if let Ok(s) = stream {
            info!("New connection established from {}", s.peer_addr()?);
            thread::spawn(|| {
                if let Err(e) = handle_client(s) {
                    warn!("Error handling client: {:?}", e);
                }
            });
        } else {
            warn!("Failed to accept connection");
        }
    }
    Ok(())
}

use std::io::{Result, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;

fn handle_client(mut client1: TcpStream, mut client2: TcpStream) -> Result<()> {
    writeln!(client1, "Paired with another player!")?;
    writeln!(client2, "Paired with another player!")?;

    Ok(())
}

fn new_client(mut this: TcpStream) -> Result<()> {
    static GUARD: Mutex<Option<TcpStream>> = Mutex::new(None);

    writeln!(this, "Awaiting another player...")?;
    println!("Client connected: {:?}", this.peer_addr());

    let another = {
        let mut guard = GUARD.lock().unwrap();
        if let Some(another) = guard.take() {
            another
        } else {
            *guard = Some(this);
            return Ok(());
        }
    };

    handle_client(this, another)?;

    Ok(())
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    println!("Server listening on {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        if let Ok(s) = stream {
            std::thread::spawn(|| {
                let _ = new_client(s);
            });
        }
    }
    Ok(())
}

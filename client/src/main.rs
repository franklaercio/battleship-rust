use std::io::{self, BufRead};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    println!("Connected to the server.");

    let mut reader = io::BufReader::new(stream.try_clone()?);
    let mut buffer = String::new();

    while reader.read_line(&mut buffer)? > 0 {
        println!("Server: {}", buffer.trim());
        buffer.clear();
    }

    Ok(())
}


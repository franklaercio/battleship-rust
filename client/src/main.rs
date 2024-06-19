use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    println!("Connected to the server.");

    // Primeiro, o jogador posiciona seus navios
    // place_ships(&mut stream);

    let stream_clone = stream.try_clone()?;
    std::thread::spawn(move || {
        let mut reader = BufReader::new(stream_clone);
        let mut buffer = String::new();

        while reader.read_line(&mut buffer).unwrap() > 0 {
            let response = std::str::from_utf8(buffer.as_bytes()).unwrap();
            if response.starts_with("UPDATE_BOARD") {
                let board_data = response.replace("UPDATE_BOARD;", "");
                let cleaned_board_data = board_data;
                display_board(&cleaned_board_data);
            } else if response.starts_with("CHECK_END_GAME") {
                // Handle end game check here
            }

            buffer.clear();
        }
    });

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input = line?;
        if input == "PLACE_BOMB" {
            print!("Enter the coordinates to place the bomb: ");

            io::stdout().flush()?;
            let mut bomb_coords = String::new();
            io::stdin().read_line(&mut bomb_coords)?;
            let bombs = format!("PLACE_BOMB;{}", bomb_coords);
            stream.write(bombs.as_bytes()).unwrap();
            continue;
        }
        stream.write_all(input.as_bytes())?;
        stream.write_all(b"\n")?;
    }

    Ok(())
}

fn display_board(boards_data: &String) {
    let boards: Vec<&str> = boards_data.split("||").collect();
    if boards.len() == 2 {
        println!("\n        Y O U                   E N E M Y     \n");
        let your_board: Vec<&str> = boards[0].split(';').collect();
        let enemy_board: Vec<&str> = boards[1].split(';').collect();
        for (your_row, enemy_row) in your_board.iter().zip(enemy_board.iter()) {
            println!("{}    {}", your_row, enemy_row);
        }
    }
}

// fn place_ships(_stream: &mut TcpStream) {
//     todo!()
// }
use std::io::{BufRead, BufReader, Result, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

const BOARD_SIZE: usize = 5;
const TOTAL_MOVES: usize = 5;

fn parse_position(input: &str) -> Option<(usize, usize)> {
    let parts: Vec<&str> = input.trim().split(',').collect();
    if parts.len() == 2 {
        if let (Ok(x), Ok(y)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
            if x < BOARD_SIZE && y < BOARD_SIZE {
                return Some((x, y));
            }
        }
    }
    None
}

fn read_position(r: &mut impl BufRead, w: &mut impl Write) -> Result<(usize, usize)> {
    let mut line = String::new();
    loop {
        line.clear();
        r.read_line(&mut line)?;
        if let Some(pos) = parse_position(&line) {
            return Ok(pos);
        }
        write!(w, "Invalid position, try again (format: x,y): ")?;
        w.flush()?;
    }
}

fn clear_screen(out: &mut impl Write) -> Result<()> {
    write!(out, "\x1B[2J\x1B[H")?;
    out.flush()
}

fn display_title(out: &mut impl Write, title: &str) {
    for ch in title.chars() {
        write!(out, "{}", ch).unwrap();
        thread::sleep(Duration::from_millis(50));
        std::io::stdout().flush().unwrap();
    }
    writeln!(out, "\r").unwrap();
}

fn display_board(out: &mut impl Write, board: &[[bool; BOARD_SIZE]; BOARD_SIZE], hits: &[(usize, usize)]) -> Result<()> {
    clear_screen(out)?;
    display_title(out, ">>>>>>>>>>>>>>>>>>>>>>");
    display_title(out, "B A T T L E S H I P");
    display_title(out, ">>>>>>>>>>>>>>>>>>>>>>");
    writeln!(out, "\nEnemy battlefield\r\n")?;
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            if hits.contains(&(i, j)) && board[i][j] {
                write!(out, " X ")?;
            } else if hits.contains(&(i, j)) {
                write!(out, " O ")?;
            } else {
                write!(out, " ~ ")?;
            }
        }
        writeln!(out, "\r\n")?;
    }
    out.flush()
}

fn place_ships(r: &mut impl BufRead, w: &mut impl Write, board: &mut [[bool; BOARD_SIZE]; BOARD_SIZE]) -> Result<()> {
    write!(w, "Place 3 ships on the board (format: x,y):\r\n")?;
    w.flush()?;
    for _ in 0..3 {
        let (x, y) = read_position(r, w)?;
        board[x][y] = true;
    }
    Ok(())
}

fn player_turn(r: &mut impl BufRead, w: &mut impl Write, board: &[[bool; BOARD_SIZE]; BOARD_SIZE], hits: &mut Vec<(usize, usize)>) -> Result<()> {
    write!(w, "It's your turn, choose where to attack (format: x,y): ")?;
    w.flush()?;
    let (x, y) = read_position(r, w)?;
    hits.push((x, y));
    if board[x][y] {
        display_board(w, board, hits)?;
        write!(w, "You Hit your enemy, await your opponent play!")?;
    } else {
        display_board(w, board, hits)?;
        write!(w, "You Miss your enemy, await your opponent play!")?;
    }
    w.flush()
}

fn handle_client(mut stream: TcpStream) -> Result<()> {
    static WAITING: Mutex<Option<TcpStream>> = Mutex::new(None);

    write!(stream, "Waiting for another player...\r\n")?;
    stream.flush()?;
    let opponent = {
        let mut guard = WAITING.lock().unwrap();
        if let Some(opponent) = guard.take() {
            opponent
        } else {
            *guard = Some(stream);
            return Ok(());
        }
    };

    let mut player1_board = [[false; BOARD_SIZE]; BOARD_SIZE];
    let mut player2_board = [[false; BOARD_SIZE]; BOARD_SIZE];
    let mut player1_hits = Vec::new();
    let mut player2_hits = Vec::new();

    let (mut player1_reader, mut player1_writer) = (BufReader::new(stream.try_clone()?), stream.try_clone()?);
    let (mut player2_reader, mut player2_writer) = (BufReader::new(opponent.try_clone()?), opponent.try_clone()?);

    write!(player1_writer, "Player 1, place your ships.\r\n")?;
    player1_writer.flush()?;
    place_ships(&mut player1_reader, &mut player1_writer, &mut player1_board)?;

    write!(player2_writer, "Player 2, place your ships.\r\n")?;
    player2_writer.flush()?;
    place_ships(&mut player2_reader, &mut player2_writer, &mut player2_board)?;

    for i in 0..TOTAL_MOVES {
        if i % 2 == 0 {
            player1_writer.flush()?;
            display_board(&mut player1_writer, &player2_board, &player1_hits)?;
            player_turn(&mut player1_reader, &mut player1_writer, &player2_board, &mut player1_hits)?;
        } else {
            player2_writer.flush()?;
            display_board(&mut player2_writer, &player1_board, &player2_hits)?;
            player_turn(&mut player2_reader, &mut player2_writer, &player1_board, &mut player2_hits)?;
        }
    }

    let player1_score = player1_hits.iter().filter(|&&(x, y)| player2_board[x][y]).count();
    let player2_score = player2_hits.iter().filter(|&&(x, y)| player1_board[x][y]).count();

    if player1_score > player2_score {
        write!(player1_writer, "\r\nYou won!\r\n")?;
        write!(player2_writer, "\r\nYou lost!\r\n")?;
    } else if player2_score > player1_score {
        write!(player1_writer, "\r\nYou lost!\r\n")?;
        write!(player2_writer, "\r\nYou won!\r\n")?;
    } else {
        write!(player1_writer, "\r\nDraw!\r\n")?;
        write!(player2_writer, "\r\nDraw!\r\n")?;
    }

    player1_writer.flush()?;
    player2_writer.flush()?;

    Ok(())
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:5560")?;
    for stream in listener.incoming() {
        if let Ok(s) = stream {
            thread::spawn(|| {
                let _ = handle_client(s);
            });
        }
    }
    Ok(())
}

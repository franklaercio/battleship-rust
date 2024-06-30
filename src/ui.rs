use std::io::{Result, Write};
use std::thread;
use std::time::Duration;
use crate::game::BOARD_SIZE;

pub fn clear_screen(out: &mut impl Write) -> Result<()> {
    write!(out, "\x1B[2J\x1B[H")?;
    out.flush()
}

pub fn display_title(out: &mut impl Write, title: &str) {
    for ch in title.chars() {
        write!(out, "{}", ch).unwrap();
        thread::sleep(Duration::from_millis(50));
        std::io::stdout().flush().unwrap();
    }
    writeln!(out, "\r").unwrap();
}

pub fn display_board(out: &mut impl Write, board: &[[bool; 5]], hits: &[(usize, usize)]) -> Result<()> {
    clear_screen(out)?;
    display_title(out, ">>>>>>>>>>>>>>>>>>>>>>\r\nB A T T L E S H I P\r\n<<<<<<<<<<<<<<<<<<<<<<\r\n");

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

pub fn display_placed_ships_board(out: &mut impl Write, board: &[[bool; 5]], hits: &[(usize, usize)]) -> Result<()> {
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            if board[i][j] {
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

pub fn display_attacked_ships_board(out: &mut impl Write, board: &[[bool; 5]], hits: &[(usize, usize)]) -> Result<()> {
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
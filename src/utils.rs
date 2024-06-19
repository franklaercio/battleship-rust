use std::io::{BufRead, Result, Write};
use crate::game::BOARD_SIZE;

pub fn parse_position(input: &str) -> Option<(usize, usize)> {
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

pub fn read_position(r: &mut impl BufRead, w: &mut impl Write) -> Result<(usize, usize)> {
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

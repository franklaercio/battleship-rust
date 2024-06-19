use std::io::{BufRead, Result, Write};

use crate::player::Player;
use crate::utils::read_position;

pub const BOARD_SIZE: usize = 5;
pub const TOTAL_ROUNDS: usize = 5;

pub fn place_ships(r: &mut impl BufRead, w: &mut impl Write, board: &mut [[bool; 5]]) -> Result<()> {
    write!(w, "\nIt's your time to place 3 ships.\r\n\n")?;
    w.flush()?;
    for _ in 0..3 {
        write!(w, "Ship at (format: x,y): ")?;
        w.flush()?;
        let (x, y) = read_position(r, w)?;
        board[x][y] = true;
        write!(w, "Ship placed at ({}, {})\r\n\n", x, y)?;
        w.flush()?;
    }

    write!(w, "All ships placed, await your opponent play!")?;
    w.flush()?;
    Ok(())
}

pub fn player_turn(r: &mut impl BufRead, w: &mut impl Write, board: &[[bool; 5]], hits: &mut Vec<(usize, usize)>) -> Result<()> {
    todo!("Player must be selected a position to attack the opponent board");
}

pub fn determine_winner<R1: BufRead, W1: Write, R2: BufRead, W2: Write>(
    player1: &mut Player<R1, W1>,
    player2: &mut Player<R2, W2>,
) -> Result<()> {
    todo!("Determine the winner based on the hits of each player")
}
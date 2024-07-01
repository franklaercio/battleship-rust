use std::io::{BufRead, Result, Write};

use log::info;
use crate::player::Player;
use crate::utils::read_position;

use crate::ui::{display_attacked_ships_board, display_board, display_placed_ships_board};

pub const BOARD_SIZE: usize = 5;
pub const TOTAL_ROUNDS: usize = 49;
pub const TOTAL_SHIPS: usize = 3;

pub fn place_ships(r: &mut impl BufRead, w: &mut impl Write, board: &mut [[bool; BOARD_SIZE]], hits: &[(usize, usize)]) -> Result<()> {
    write!(w, "\nIt's your time to place 3 ships.\r\n\n")?;
    w.flush()?;
    display_placed_ships_board(w, board, hits)?;
    
    for _ in 0..TOTAL_SHIPS {
        loop {
            write!(w, "Ship at (format: x,y): ")?;
            w.flush()?;
            let (x, y) = read_position(r, w)?;
            
            if !board[x][y] {
                board[x][y] = true;
                write!(w, "Ship placed at ({}, {})\r\n\n", x, y)?;
                w.flush()?;
                break;
            }

            write!(w, "You alredy placed at ({}, {}), try again!\r\n\n", x, y)?;
            w.flush()?;
        }

        display_placed_ships_board(w, board, hits)?;
    }

    write!(w, "All ships placed, await your opponent play!")?;
    w.flush()?;
    Ok(())
}

pub fn player_turn(r: &mut impl BufRead, w: &mut impl Write, board: &mut [[bool; 5]], hits: &mut Vec<(usize, usize)>) -> Result<()> {
    loop {
        write!(w, "Attack ship at (format: x,y): ")?;
        w.flush()?;
        let (x, y) = read_position(r, w)?;
       
        if !hits.contains(&(x, y)) {
            hits.push((x,y));
          
            write!(w, "\nYou attacked at ({}, {})\r\n\n", x, y)?;
            w.flush()?;
            
            break;
        } 

        write!(w, "\nYou already attacked at ({}, {}), try again!\r\n\n", x, y)?;
        w.flush()?;
    }
    
    display_attacked_ships_board(w, board, hits)?;
    write!(w, "\nWating the other opponent's attack...\r\n\n")?;
    w.flush()?;
    
    Ok(())
}

pub fn determine_winner<R1: BufRead, W1: Write, R2: BufRead, W2: Write>(
    player1: &mut Player<R1, W1>,
    player2: &mut Player<R2, W2>,
) -> Result<()> {
    let score1 = player1.calculate_score(&player2.board);
    let score2 = player2.calculate_score(&player1.board);
  
    let mut message1 = format!("\nYou won, {} points\n", score1);
    let mut message2 = format!("\nYou lost, {} points\n", score2);

    if score2 > score1 {
        message2 = format!("\nYou won, {} points\n", score2);
        message1 = format!("\nYou lost, {} points\n", score1);
    } else if score1 == score2 {
        let tied_msg = format!("\nGame tied {} - {} points\n", score1, score2);
        message1 = tied_msg.clone();
        message2 = tied_msg.clone();
    }
  
    write!(player1.writer, "{}", message1)?;
    player1.writer.flush()?;

    write!(player2.writer, "{}", message2)?;
    player2.writer.flush()?;

    Ok(())
}
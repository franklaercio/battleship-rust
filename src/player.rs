use std::io::{BufRead, Result, Write};

use crate::game::{BOARD_SIZE, TOTAL_SHIPS, place_ships, player_turn};
use crate::ui::display_board;

pub struct Player<R: BufRead, W: Write> {
    pub reader: R,
    pub writer: W,
    pub board: [[bool; BOARD_SIZE]; BOARD_SIZE],
    pub hits: Vec<(usize, usize)>,
    pub ip: String,
    pub ships_total: usize,
}

impl<R: BufRead, W: Write> Player<R, W> {
    pub fn new(reader: R, writer: W, ip: String) -> Self {
        Self {
            reader,
            writer,
            board: [[false; BOARD_SIZE]; BOARD_SIZE],
            hits: Vec::new(),
            ip,
            ships_total: TOTAL_SHIPS,
        }
    }

    pub fn place_ships(&mut self) -> Result<()> {
        write!(self.writer, "Place your ships.\r\n")?;
        self.writer.flush()?;
        place_ships(&mut self.reader, &mut self.writer, &mut self.board, &self.hits)
    }

    pub fn take_turn(&mut self, opponent_board: &[[bool; BOARD_SIZE]]) -> Result<()> {
        display_board(&mut self.writer, opponent_board, &self.hits)?;
        player_turn(&mut self.reader, &mut self.writer, opponent_board, &mut self.hits)
    }

    pub fn calculate_score(&self, opponent_board: &[[bool; BOARD_SIZE]]) -> usize {
        self.hits.iter().filter(|&&(x, y)| opponent_board[x][y]).count()
    }
}

use std::io::Write;
use std::net::TcpStream;

#[derive(Clone, Copy)]
pub struct Board {
    grid: [[char; 10]; 10],
}

impl Board {
    pub fn new() -> Self {
        Self {
            grid: [['■'; 10]; 10],
        }
    }

    pub fn update(&mut self, x: usize, y: usize) {
        self.grid[x][y] = 'X';
    }

    pub fn render(&self) -> String {
        let mut board_str = String::from("  A B C D E F G H I J;");

        for (i, row) in self.grid.iter().enumerate() {
            let row_str: String = row.iter().collect::<String>().replace("", " ").trim().to_string();
            board_str.push_str(&format!("{} {};", i, row_str));
        }

        board_str
    }

    pub fn display(&self, mut client: TcpStream) {
        for row in &self.grid {
            for cell in row {
                writeln!(client, "{}", cell).unwrap();
            }
            writeln!(client, "{}", "\\n").unwrap();
        }
    }
}
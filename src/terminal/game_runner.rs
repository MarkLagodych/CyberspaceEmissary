use crate::game::Game;

use std::io::{self, Write};

use termion::{cursor, color, terminal_size};
use termion::raw::{IntoRawMode, RawTerminal};

pub struct GameRunner {
    game: Game,
}

impl GameRunner {
    pub fn new() -> Self {
        let size = terminal_size().unwrap();
        Self {
            game: Game::new(size.0 as usize, size.1 as usize)
        }
    }

    pub fn run(&mut self) {
        let stdout = io::stdout();
        let mut stdout_locked = stdout.lock();
        let mut stdout_raw = stdout_locked.into_raw_mode().unwrap();

        write!(
            stdout_raw, "{}", termion::clear::All
        );

        let size = terminal_size().unwrap();
        let w = size.0 as usize;
        let h = size.1 as usize;

        for i in 0..h {
            for j in 0..w {
                let c = self.game.color_map[i][j];
                write!(
                    stdout_raw,
                    "{}{}{}",
                    cursor::Goto(j as u16 + 1, i as u16 + 1),
                    color::Fg(color::Rgb(c.r, c.g, c.b)),
                    self.game.symbol_map[i][j]
                );
            }
        }
        stdout_raw.flush().unwrap();
    }
}
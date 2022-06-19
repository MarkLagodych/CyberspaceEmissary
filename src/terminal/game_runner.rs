use crate::game::*;
use crate::special_key_codes::*;

use std::io::{self, Write};
use std::{thread, time};


use termion::input::TermRead;
use termion::{cursor, color, terminal_size};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::input::Keys;
use termion::event::Key;

pub struct GameRunner {
    game: Game,
}

type RawStdout<'a> = RawTerminal<io::StdoutLock<'a>>;

impl GameRunner {
    pub fn new() -> Self {
        let (width, height) = Self::get_size();
        Self {
            game: Game::new(width, height),
        }
    }

    fn get_size() -> (usize, usize) {
        let size = terminal_size().unwrap();
        (size.0 as usize, size.1 as usize)
    }

    fn clear(stdout: &mut RawStdout) {
        write!(stdout, "{}", termion::clear::All);
    }

    fn cleanup(stdout: &mut RawStdout) {
        Self::clear(stdout);
        Self::goto(stdout, Position::origin());
        write!(stdout, "{}{}", color::Fg(color::Reset), color::Bg(color::Reset));
        Self::update(stdout);
    }

    fn update(stdout: &mut RawStdout) {
        stdout.flush().unwrap();
    }

    fn goto(stdout: &mut RawStdout, pos: Position) {
        write!(
            stdout,
            "{}",
            cursor::Goto(1 + pos.x as u16, 1 + pos.y as u16)
        );
    }

    fn draw_character(stdout: &mut RawStdout, c: &Character) {
        let rend = &c.states[c.state];
        
        let mut pos = c.position;

        for line in rend.content.split("\n") {
            Self::goto(stdout, pos);
            
            write!(
                stdout,
                "{}{}",
                color::Fg(color::Rgb(rend.color.r, rend.color.g, rend.color.b)),
                line
            );

            pos.y += 1;
        }
    }

    pub fn run(&mut self) {
        let _stdout = io::stdout();
        let _stdout_locked = _stdout.lock();
        let mut stdout = _stdout_locked.into_raw_mode().unwrap();

        let mut keys = termion::async_stdin().keys();

        self.game.characters.push(Character { 
            states: vec![
                Renderable {
                    color: Color::new(255, 200, 0),
                    content: String::from("Waka\r\nWaka\r\n EE ")
                }
            ],
            state: 0,
            position: Position::new(5, 1)
        });

        loop {

            let (width, height) = Self::get_size();
            self.game.set_size(width, height);

            Self::clear(&mut stdout);

            let key = keys.next();
            if let Some(key) = key {
                match key.unwrap() {
                    Key::Char(c) => self.game.process_key(c, false),
                    Key::Ctrl(c) => self.game.process_key(c, true),
                    Key::Backspace => self.game.process_key(KEY_BACKSPACE, false),
                    _ => {}
                }
            }

            if self.game.stopped {
                break;
            }

            for character in &self.game.characters {
                Self::draw_character(&mut stdout, character);
            }
           
            Self::goto(&mut stdout, self.game.cursor_position);
            Self::update(&mut stdout);

            thread::sleep(time::Duration::from_millis(16));
        }

        Self::cleanup(&mut stdout);

    }
}
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
        Self {
            game: Game::new(Self::get_size()),
        }
    }

    fn get_size() -> Size {
        let size = terminal_size().unwrap();
        Size::new(size.0.into(), size.1.into())
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
        // io::stdout().flush();
        stdout.flush().unwrap();
    }

    fn goto(stdout: &mut RawStdout, pos: Position) {
        write!(
            stdout,
            "{}",
            cursor::Goto(1 + pos.x as u16, 1 + pos.y as u16)
        );
    }

    fn hide_cursor(stdout: &mut RawStdout) {
        write!(stdout, "{}", cursor::Hide);
    }

    fn show_cursor(stdout: &mut RawStdout) {
        write!(stdout, "{}", cursor::Show);
    }

    fn draw_character(&self, stdout: &mut RawStdout, ch: &Character) {
        for rend in &ch.renderables {
            let mut pos = ch.position + rend.offset;
            
            if !rect_is_fully_inside(&self.game.size, &pos, &rend.size) {
                continue;
            }

            for line in rend.content.split("\n") {
                Self::goto(stdout, pos);
                
                write!(
                    stdout,
                    "{}{}",
                    color::Fg(color::Rgb(rend.color.r, rend.color.g, rend.color.b)),
                    line
                );

                pos.y += 1;
                // Self::update(stdout);
            }

        }
    }

    pub fn run(&mut self) {
        let _stdout = io::stdout();
        let _stdout_locked = _stdout.lock();
        let mut stdout = _stdout_locked.into_raw_mode().unwrap();

        let mut keys = termion::async_stdin().keys();

        loop {

            self.game.set_size(Self::get_size());

            Self::clear(&mut stdout);

            let key = keys.next();
            if let Some(key) = key {
                match key.unwrap() {
                    Key::Char(c) => self.game.process_key(c, false),
                    Key::Ctrl(c) => self.game.process_key(c, true),
                    Key::Backspace => self.game.process_key(KEY_BACKSPACE, false),
                    Key::Left => self.game.process_key('[', false),
                    Key::Right => self.game.process_key(']', false),
                    Key::Up | Key::Down => self.game.process_key('\'', false),
                    _ => {}
                }
            }

            if self.game.stopped {
                break;
            }

            Self::hide_cursor(&mut stdout);

            if !self.game.min_size.fits_in(&self.game.size) {
                Self::goto(&mut stdout, Position::new(0, 0));
                print!("Minimum size: {}x{}", self.game.min_size.width, self.game.min_size.height);
                Self::goto(&mut stdout, Position::new(0, 1));
                print!("Current size: {}x{}", self.game.size.width, self.game.size.height);
                Self::goto(&mut stdout, Position::new(0, 2));
                print!("Resize your terminal, please");
                Self::update(&mut stdout);
            } else {
                let room = &self.game.rooms[self.game.current_room];
                for entity_id in &room.entities {
                    let character = (&self.game.entities)[*entity_id].get_character();
                    self.draw_character(&mut stdout, character);
                }
                // Self::update(&mut stdout);
            }
           
            Self::show_cursor(&mut stdout);

            Self::goto(&mut stdout, self.game.cursor_position);
            Self::update(&mut stdout);

            thread::sleep(time::Duration::from_millis(30));
        }

        Self::cleanup(&mut stdout);

    }
}
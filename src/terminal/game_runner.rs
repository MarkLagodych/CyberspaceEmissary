use crate::game::*;
use crate::special_key_codes::*;

use std :: {
    io::{self, Write},
    thread,
    time::Duration
};


use crossterm :: {
    terminal,
    cursor,
    QueueableCommand,
    queue,
    execute,
    event,
    style,
};



pub struct GameRunner {
    game: Box<Game>,
}

impl GameRunner {
    pub fn new() -> Self {
        Self {
            game: Box::new(
                Game::new(Self::static_get_size())
            ),
        }
    }

    fn static_get_size() -> Size {
        let size = terminal::size().unwrap();
        Size::new(size.0.into(), size.1.into())
    }

    fn get_size(&self) -> Size {
        Self::static_get_size()
    }

    fn clear(&self) {
        queue!(io::stdout(), terminal::Clear(terminal::ClearType::Purge));
    }

    fn cleanup(&self) {
        execute!(
            io::stdout(),
            cursor::MoveTo(0, 0),
            terminal::Clear(terminal::ClearType::All),
            terminal::Clear(terminal::ClearType::Purge)
        );
    }

    fn flush(&self) {
        io::stdout().flush();
    }

    fn update_cursor(&self) {
        queue!(
            io::stdout(),
            cursor::MoveTo(
                self.game.cursor_position.x as u16,
                self.game.cursor_position.y as u16
            )
        );
    }

    fn draw(&self) {
        let mut x = 0usize;
        let mut y = 0usize;

        queue!(io::stdout(), cursor::Hide);
        
        for line in &self.game.symbol_buffer {
            queue!(io::stdout(), cursor::MoveTo(0, y as u16));

            x = 0;

            for symbol in line {

                if *symbol != ' ' {
                    let color = &self.game.color_buffer[y][x];

                    queue!(
                        io::stdout(),
                        style::SetForegroundColor(
                            style::Color::Rgb { r: color.r, g: color.g, b: color.b }
                        )
                    );
                }

                write!(io::stdout(), "{}", symbol);

                x += 1;
            }

            y += 1;
        }

        queue!(io::stdout(), cursor::Show);

    }

    pub fn run(&mut self) {
        terminal::enable_raw_mode();

        loop {

            if event::poll(Duration::from_millis(30)).unwrap() {
                match event::read().unwrap() {
                    event::Event::Key(key_event) => {
                        match key_event.code {
                            event::KeyCode::Char(ch) => {
                                if key_event.modifiers.contains(event::KeyModifiers::CONTROL) {
                                    self.game.process_key(ch, true);
                                } else {
                                    self.game.process_key(ch, false);
                                }
                            }

                            event::KeyCode::Left => {
                                self.game.process_key('[', false);
                            }

                            event::KeyCode::Right => {
                                self.game.process_key(']', false);
                            }

                            event::KeyCode::Up
                            | event::KeyCode::Down => {
                                self.game.process_key('\'', false);
                            }

                            event::KeyCode::Enter => {
                                self.game.process_key(KEY_ENTER, false);
                            }

                            event::KeyCode::Backspace => {
                                self.game.process_key(KEY_BACKSPACE, false);
                            }

                            _ => {}
                        }
                    }

                    _ => {}
                }
            }

            if self.game.stopped {
                break;
            }

            self.game.set_size(self.get_size());
            self.game.render();
            self.clear();
            self.draw();
            self.update_cursor();
            self.flush();

            // thread::sleep(Duration::from_millis(64));
        }

        self.cleanup();
        self.flush();

        terminal::disable_raw_mode();
        // self.flush();
    }
}
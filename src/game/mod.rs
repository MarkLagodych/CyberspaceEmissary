mod utils;
pub use utils::*;

use crate::special_key_codes::*;

pub struct Game {
    pub characters: Vec<Character>,
    pub stopped: bool,

    pub cursor_position: Position,

    width: usize,
    height: usize,

    console: String,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        let mut new_self = Self {
            characters: vec![],
            stopped: false,
            
            cursor_position: Position::new(0, height-1),
            
            width,
            height,

            console: "".into()
        };

        new_self.characters.push(
            Character {
                states: vec![
                    Renderable {
                        color: Color::white(),
                        content: "".into()
                    }
                ],
                state: 0,
                position: Position::new(0, height-1)
            }
        );

        new_self
    }

    pub fn set_size(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }

    pub fn process_key(&mut self, key: char, ctrl: bool) {
        if ctrl && key == 'q' {
            self.stopped = true;
            return;
        }

        match key {
            '\n' => {
                self.console.clear();
                self.cursor_position.x = 0;
            }

            KEY_BACKSPACE => {
                if (self.console.len() > 0) {
                    self.console.pop();
                    self.cursor_position.x -= 1;
                }
            }

            _ => {
                self.console.push(key);
                self.cursor_position.x += 1;
            }
        }

        self.characters[0].states[0].content = self.console.clone();
    }
}
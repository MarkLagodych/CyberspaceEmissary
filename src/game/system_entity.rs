use super::*;

const SYSENT_REND_CONSOLE: usize = 0;

pub struct SystemEntity {
    view_size: Size,
    console: String,
    character: Character,
}

impl SystemEntity {
    pub fn new() -> Self {
        Self {
            view_size: Size::new(1, 1),
            console: "".into(),
            character: Character {
                renderables: vec![
                    Renderable::new()
                ],
                position: Position::origin()
            }
        }
    }

    pub fn set_size(&mut self, size: Size) {
        self.view_size = size;

        self.character.renderables[SYSENT_REND_CONSOLE].offset.y = size.height - 1;
    }

    fn console_update(&mut self) {
        self.character.renderables[SYSENT_REND_CONSOLE].content = self.console.clone();
    }

    pub fn console_add_char(&mut self, ch: char) {
        self.console.push(ch);
        self.console_update();
    }

    pub fn console_backspace(&mut self) {
        if (self.console.len() > 0) {
            self.console.pop();
            self.console_update();
        }
    }

    pub fn console_len(&self) -> usize {
        self.console.len()
    }

    pub fn console_finish(&mut self) -> String {
        let s = self.console.clone();
        self.console.clear();
        self.console_update();
        s
    }
}

impl Entity for SystemEntity {
    fn get_character(&self) -> &Character {
        &self.character
    }

    fn get_system_entity_mut(&mut self) -> Option<&mut SystemEntity> {
        Some(self)
    }
}
use super::*;

const SYSENT_REND_CONSOLE: usize = 0;
const SYSENT_REND_ROOM_TOP: usize = 1;
const SYSENT_REND_ROOM_LEFT: usize = 2;
const SYSENT_REND_ROOM_RIGHT: usize = 3;
const SYSENT_REND_ROOM_BOTTOM: usize = 4;
const SYSENT_REND_HERO: usize = 5;

const HERO: &'static str =
r" 0
/#\
/ \";

const ROOM_WIDTH: i32 = 90;
const ROOM_HEIGHT: i32 = 25;

const ROOM_BORDER_HORZ: &'static str = 
"##########################################################################################";
const ROOM_BORDER_VERT: &'static str = 
"#\n#\n#\n#\n#\n#\n#\n#\n#\n#\n#\n#\n#\n#\n#\n#\n#\n#\n#\n#\n#\n#\n#\n";

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
                    // Console
                    Renderable::new(),
                    // Room
                    Renderable {
                        color: Color::new(0, 95, 127),
                        content: ROOM_BORDER_HORZ.into(),
                        offset: Position::origin(),
                        size: Size::new(ROOM_BORDER_HORZ.len() as i32, 1)
                    },
                    
                    Renderable {
                        color: Color::new(0, 95, 127),
                        content: ROOM_BORDER_VERT.into(),
                        offset: Position::new(0, 1),
                        size: Size::new(1, ROOM_BORDER_VERT.len() as i32)
                    },
                    Renderable {
                        color: Color::new(0, 95, 127),
                        content: ROOM_BORDER_VERT.into(),
                        offset: Position::new(ROOM_WIDTH-1, 1),
                        size: Size::new(1, ROOM_BORDER_VERT.len() as i32)
                    },
                    Renderable {
                        color: Color::new(0, 95, 127),
                        content: ROOM_BORDER_HORZ.into(),
                        offset: Position::new(0, ROOM_HEIGHT - 1),
                        size: Size::new(ROOM_BORDER_HORZ.len() as i32, 1)
                    },
                    // Hero
                    Renderable {
                        color: Color::new(255, 0, 255),
                        content: HERO.into(),
                        offset: Position::new(1, ROOM_HEIGHT-1-3),
                        size: Size::new(3, 3)
                    }
                ],
                position: Position::origin()
            }
        }
    }

    pub fn get_cursor_pos(&self) -> Position {
        self.character.renderables[SYSENT_REND_CONSOLE].offset
        + Position::new(self.console.len() as i32, 0)
    }

    pub fn get_min_size(&self) -> Size {
        Size::new(ROOM_WIDTH, ROOM_HEIGHT + 2)
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

    pub fn move_hero(&mut self, dir: i32) {
        let rend = &mut self.character.renderables[SYSENT_REND_HERO];
        if (rend.offset.x + dir) < 1
        || (rend.offset.x + dir + rend.size.width) > ROOM_WIDTH-1
        {
            return;
        }
        else {
            rend.offset.x += dir;
        }
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
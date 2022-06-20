mod atoms;
pub use atoms::*;

mod system_entity;
use system_entity::*;

use crate::special_key_codes::*;

pub struct Game {
    pub size: Size,
    pub min_size: Size,
    
    pub stopped: bool,
    pub cursor_position: Position,
    
    pub rooms: Vec<Room>,
    pub current_room: usize,
    pub entities: Vec<Box<dyn Entity>>,

    system_entity: ID,
}

impl Game {
    pub fn new(size: Size) -> Self {
        let mut new_self = Self {
            rooms: vec![Room::new()],
            current_room: 0,
            entities: vec![],
            stopped: false,
            
            cursor_position: Position::origin(),
            
            size,
            min_size: Size::new(80, 25),

            system_entity: 0,
        };

        let sysent = SystemEntity::new();

        new_self.cursor_position = Position::new(0, size.height-1);
        new_self.system_entity = new_self.new_entity(Box::new(sysent));

        new_self.rooms[0].entities.push(new_self.system_entity);


        new_self
    }

    fn get_system_entity_mut(&mut self) -> &mut SystemEntity {
        self.entities[self.system_entity].get_system_entity_mut().unwrap()
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = size;
        self.get_system_entity_mut().set_size(size);
        self.cursor_position.x = self.get_system_entity_mut().console_len() as i32;
    }

    pub fn process_key(&mut self, key: char, ctrl: bool) {
        let sysent = self.get_system_entity_mut();
        
        if ctrl {
            match key {
                'q' => {
                    self.stopped = true;
                    return;
                }

                _ => {}
            }
        }

        match key {
            '\n' => {
                sysent.console_finish();
            }

            KEY_BACKSPACE => {
                sysent.console_backspace();
            }

            '0'..='9' => {

            }

            _ => {
                sysent.console_add_char(key);
            }
        }

        self.cursor_position.x = sysent.console_len() as i32;

    }

    fn new_entity(&mut self, ent: Box<dyn Entity>) -> ID {
        self.entities.push(ent);
        self.entities.len() - 1
    }

    fn get_character(&self, entity_id: ID) -> &Character {
        self.entities[entity_id].get_character()
    }
}
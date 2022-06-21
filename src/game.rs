mod atoms;

pub use atoms::*;

mod system_entity;
use system_entity::*;

pub mod ascii_art;
use ascii_art::*;

use crate::special_key_codes::*;

pub struct Game {
    pub size: Size,
    pub min_size: Size,
    
    pub stopped: bool,
    pub cursor_position: Position,
    
    pub rooms: Vec<Room>,
    pub current_room: usize,
    pub entities: Vec<Box<dyn Entity>>,

    pub symbol_buffer: Vec<Vec<char>>,
    pub color_buffer: Vec<Vec<Color>>,

    system_entity: ID,
}

impl Game {
    pub fn new(size: Size) -> Self {
        let mut new_self = Self {
            rooms: vec![],
            current_room: 0,
            entities: vec![],
            stopped: false,
            
            cursor_position: Position::origin(),
            
            size: Size::new(0, 0),
            min_size: Size::new(80, 25),

            symbol_buffer: vec![],
            color_buffer: vec![],

            system_entity: 0,
        };

        new_self.resize_buffers(size);

        let mut sysent = SystemEntity::new();
        sysent.set_size(size);
        new_self.min_size = sysent.get_min_size();
        new_self.cursor_position = sysent.get_cursor_pos();
        new_self.system_entity = new_self.new_entity(Box::new(sysent));

        new_self.construct_rooms();

        new_self
    }

    fn get_system_entity_mut(&mut self) -> &mut SystemEntity {
        self.entities[self.system_entity].get_system_entity_mut().unwrap()
    }

    pub fn set_size(&mut self, size: Size) {
        if self.size != size {
           self.resize_buffers(size);
        }

        self.size = size;
        self.get_system_entity_mut().set_size(size);
        self.cursor_position = self.get_system_entity_mut().get_cursor_pos();
    }

    fn resize_buffers(&mut self, size: Size) {
        self.symbol_buffer = vec![vec![' '; size.width as usize]; size.height as usize];
        self.color_buffer = vec![vec![Color::white(); size.width as usize]; size.height as usize];
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
            KEY_ENTER => {
                sysent.console_finish();
            }

            KEY_BACKSPACE => {
                sysent.console_backspace();
            }

            '1'..='9' => {

            }

            '[' => sysent.move_hero(-1),
            ']' => sysent.move_hero(1),
            '\'' => {}

            'a'..='z' | 'A'..='Z' | ' ' => {
                sysent.console_add_char(key);
            }

            _ => {}
        }

        self.cursor_position = sysent.get_cursor_pos();

    }

    fn new_entity(&mut self, ent: Box<dyn Entity>) -> ID {
        self.entities.push(ent);
        self.entities.len() - 1
    }

    fn get_character(&self, entity_id: ID) -> &Figure {
        self.entities[entity_id].get_figure()
    }

    /// Returns: index for the new room
    fn new_room(&mut self) -> usize {
        self.rooms.push(Room::new());
        let index = self.rooms.len() - 1;
        self.rooms[index].entities.push(self.system_entity);
        index
    }

    fn construct_rooms(&mut self) {
        let start_room = self.new_room();

        let tutorial_id = self.new_entity(
            Box::new(StaticEntity::new(
                TUTORIAL.into(),
                Color::new(200, 70, 0),
                Position::new(10, 3)
            ))
        );

        self.rooms[start_room].entities.push(tutorial_id);
    }

    pub fn render(&mut self) {

        for row in &mut self.symbol_buffer {
            for symbol in row {
                *symbol = ' ';
            }
        }

        for entity_id in &self.rooms[self.current_room].entities {
            let figure = &self.entities[*entity_id].get_figure();
            
            for sprite in &figure.sprites {
                let mut pos = figure.position + sprite.offset;
                let initial_x = pos.x;
                
                for line in sprite.content.split('\n') {
                    pos.x = initial_x;

                    for symbol in line.chars() {

                        if !pos.is_in_view(&self.size) {
                            break; // Discard the rest of the line
                        }

                        let row = pos.y as usize;
                        let col = pos.x as usize;

                        if symbol != ' ' {
                            self.symbol_buffer[row][col] = symbol;
                            self.color_buffer[row][col] = sprite.color.clone();
                        }

                        pos.x += 1;
                    }

                    pos.y += 1;
                }
            }
        }

    }
}
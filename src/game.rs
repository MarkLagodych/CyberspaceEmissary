pub mod ascii_art;

mod atoms;
mod entities;
mod game_objects;

pub use atoms::*;

use entities::*;
use ascii_art::*;
use game_objects::*;

use crate::special_key_codes::*;



pub struct Game {
    pub size: Size,
    pub min_size: Size,
    
    pub stopped: bool,
    pub cursor_position: Position,
    
    rooms: Vec<Room>,
    current_room: RoomID,
    entities: Vec<Box<dyn Entity>>,

    pub symbol_buffer: Vec<Vec<char>>,
    pub color_buffer: Vec<Vec<Color>>,

    view_position: Position,

    console: SpellConsole,
    console_id: EntityID,

    debug_id: EntityID,

    tutorial_id: EntityID,
    floor_id: EntityID,

    hero_id: EntityID,
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
            min_size: Size::new(WORLD_MIN_WIDTH, WORLD_HEIGHT),

            symbol_buffer: vec![],
            color_buffer: vec![],

            view_position: Position::origin(),

            debug_id: 0,

            console: SpellConsole::new(),
            console_id: 0,

            tutorial_id: 0,
            floor_id: 0,

            hero_id: 0,
        };

        new_self.construct_entities();
        new_self.construct_rooms();
        new_self.resize_buffers(size);

        new_self
    }

    pub fn set_size(&mut self, size: Size) {
        if self.size != size {
           self.resize_buffers(size);
        }

        self.size = size;
        self.manage_console();
    }

    fn manage_console(&mut self) {
        let fig = self.entities[self.console_id].get_figure_mut();
        fig.sprites[0].content = self.console.get_spell();
        fig.position = Position::new(0, self.size.height-1);
        
        self.cursor_position = fig.position + Position::new(self.console.get_len() as i32, 0);
    }

    fn resize_buffers(&mut self, size: Size) {
        self.symbol_buffer = vec![vec![' '; size.width as usize]; size.height as usize];
        self.color_buffer = vec![vec![Color::white(); size.width as usize]; size.height as usize];
    }

    pub fn process_key(&mut self, key: char, ctrl: bool) {        
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
                self.console.finish_spell();
            }

            KEY_BACKSPACE => {
                self.console.backspace();
            }

            '1'..='9' => {

            }

            '[' | ']' => {

                let delta = if key == ']' {1} else {-1};

                let hero_pos_abs = self.entities[self.hero_id].get_figure().position;
                let hero_pos = hero_pos_abs.relative_to(self.view_position);

                let hero_size = self.entities[self.hero_id].get_size();

                let right = hero_pos.x + delta + hero_size.width;
                let right_abs = hero_pos_abs.x + delta + hero_size.width;
                if right >= self.size.width - WORLD_RIGHT_MARGIN {
                    if right_abs < self.rooms[self.current_room].size.width - WORLD_RIGHT_MARGIN {
                        self.view_position += Position::new(1, 0);
                    }
                }

                if right_abs < self.rooms[self.current_room].size.width - WORLD_RIGHT_MARGIN {
                    Hero::move_entity(&mut self.entities[self.hero_id], delta);
                }

                if delta < 0 && hero_pos.x + delta >= 0 {
                    Hero::move_entity(&mut self.entities[self.hero_id], delta);
                }
            }

            '\'' => {}

            'a'..='z' | 'A'..='Z' | ' ' => {
                self.console.add_char(key);
            }

            _ => {}
        }

        self.manage_console();

    }

    fn new_entity(&mut self, ent: Box<dyn Entity>) -> EntityID {
        self.entities.push(ent);
        self.entities.len() - 1
    }

    /// Returns: index for the new room
    fn new_room(&mut self, room_size: Size) -> RoomID {
        self.rooms.push(Room::new(room_size));
        let index = self.rooms.len() - 1;
        index
    }

    fn construct_entities(&mut self) {

        self.debug_id = self.new_entity(Box::new(StaticEntity::new(
            DEBUG.into(), Color::cyan(), Position::new(WORLD_MIN_WIDTH*2, WORLD_HEIGHT-2)
        )));

        self.console_id = self.new_entity(Box::new(StaticEntity::new(
            "".into(), Color::white(), Position::origin()
        )));

        self.tutorial_id = self.new_entity(
            Box::new(StaticEntity::new(
                TUTORIAL.into(),
                Color::new(200, 70, 0),
                Position::new(10, 3)
            ))
        );

        self.floor_id = self.new_entity(Box::new(
            StaticEntity::new(
                String::from_iter(FLOOR),
                Color::white(),
                Position::new(0, WORLD_HEIGHT-1)
            )
        ));

        self.hero_id = self.new_entity(Hero::new_entity());
    }

    fn construct_rooms(&mut self) {
        let start_room_id = self.new_room(Size::new(WORLD_MIN_WIDTH*2, WORLD_HEIGHT));

        self.rooms[start_room_id].entities.push(self.debug_id);
        self.rooms[start_room_id].entities.push(self.tutorial_id);
        self.rooms[start_room_id].entities.push(self.console_id);
        self.rooms[start_room_id].entities.push(self.floor_id);
        self.rooms[start_room_id].entities.push(self.hero_id);

        self.current_room = start_room_id;
    }

    fn is_not_in_view_yet(&self, pos: Position) -> bool {
        pos.x < 0 || pos.y < 0
    }

    fn is_not_in_view_already(&self, pos: Position) -> bool {
        pos.x >= self.size.width || pos.y >= self.size.height
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

                if !sprite.active {
                    continue;
                }

                let mut pos = (figure.position + sprite.offset).relative_to(self.view_position);
                let initial_x = pos.x;
                
                for line in sprite.content.split('\n') {
                    pos.x = initial_x;

                    for symbol in line.chars() {
                        pos.x += 1;

                        if self.is_not_in_view_yet(pos) {
                            continue; // Discard only this character
                        }

                        if self.is_not_in_view_already(pos) {
                            break; // Discard the rest of the line
                        }

                        if symbol == ' ' {
                            continue; // Invisible
                        }

                        let row = pos.y as usize;
                        let col = pos.x as usize;

                        self.symbol_buffer[row][col] = symbol;
                        self.color_buffer[row][col] = sprite.color.clone();
                    }

                    pos.y += 1;
                }
            }
        }

    }
}
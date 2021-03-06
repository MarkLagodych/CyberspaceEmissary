// CyberspaceEmissary, a terminal game
// Copyright 2022 Mark Lagodych
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

pub mod ascii_art;

mod atoms;
mod entities;
mod game_objects;

pub use atoms::*;

use entities::*;
use ascii_art::*;
use game_objects::*;
use web_sys::console;

use crate::special_key_codes::*;



pub struct Game {
    pub size: Size,
    
    pub stopped: bool,
    pub cursor_position: Position,
    
    rooms: Vec<Room>,
    current_room: RoomID,
    entities: Vec<Box<dyn Entity>>,

    pub symbol_buffer: Vec<Vec<char>>,
    pub color_buffer: Vec<Vec<Color>>,

    view_position: Position,
    is_recording_spell: bool,

    console: SpellConsole,
    console_id: EntityID,

    debug_id: EntityID,

    sign_ids: Vec<EntityID>,
    floor_id: EntityID,

    hero_id: EntityID,
    hero_controller: HeroController,

    sword_id: EntityID,

    spike_ids: Vec<EntityID>,

    hostile_ids: Vec<EntityID>,

    platform_id: EntityID,
    boss_id: EntityID,
}

impl Game {
    pub fn new(size: Size) -> Self {
        let mut new_self = Self {
            rooms: vec![],
            current_room: 0,
            entities: vec![],
            stopped: false,
            
            cursor_position: Position::origin(),
            
            size: size,
            is_recording_spell: false,

            symbol_buffer: vec![],
            color_buffer: vec![],

            view_position: Position::origin(),

            debug_id: 0,

            console: SpellConsole::new(),
            console_id: 0,

            sign_ids: vec![],
            floor_id: 0,

            hero_id: 0,
            hero_controller: HeroController::new(),
            sword_id: 0,

            spike_ids: vec![],

            hostile_ids: vec![],

            platform_id: 0,

            boss_id: 0,
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
        fig.position = Position::new(self.view_position.x, Y_BOTTOM+3);
        
        self.cursor_position =
            fig.position.relative_to(self.view_position)
            + Position::new(self.console.get_len() as i32, 0);
    }

    fn resize_buffers(&mut self, size: Size) {
        self.symbol_buffer = vec![vec![' '; size.width as usize]; size.height as usize];
        self.color_buffer = vec![vec![Color::white(); size.width as usize]; size.height as usize];
    }

    pub fn is_expecting_text(&self) -> bool {
        self.is_recording_spell
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

        if key == ' ' {
            self.is_recording_spell = true;
        }

        if self.is_recording_spell {
            match key {
                KEY_ENTER => {
                    self.is_recording_spell = false;
                    let spell = self.console.finish_spell();

                    if spell == "android" {
                        self.entities[self.boss_id].get_figure_mut().visible = false;
                    }
                }

                KEY_BACKSPACE => {
                    self.console.backspace();
                }

                'a'..='z' | 'A'..='Z' => {
                    self.console.add_char(key);
                }

                _ => {}
            }

            self.manage_console();

            return;
        }

        match key {
            'a' | 'd' => {

                let delta = if key == 'd' {1} else {-1};

                let hero_pos_abs = self.entities[self.hero_id].get_figure().position;
                let hero_pos = hero_pos_abs.relative_to(self.view_position);

                let hero_size = self.entities[self.hero_id].get_size();
                let room_size = self.rooms[self.current_room].size;

                if hero_pos.x + delta >= 0
                && hero_pos_abs.x + hero_size.width + delta < room_size.width - WORLD_RIGHT_MARGIN {
                    self.entities[self.hero_id].get_figure_mut().position.x += delta;
                    self.entities[self.sword_id].get_figure_mut().position.x += delta;
                    
                    if self.collides_with_damage(self.hero_id) {
                        self.entities[self.hero_id].get_figure_mut().position = Position::origin();
                        self.view_position = Position::origin();
                        self.hero_controller.jump_potential = 0;
                        return;
                    }

                    if self.collides(self.hero_id) {
                        self.entities[self.hero_id].get_figure_mut().position.x -= delta;
                        self.entities[self.sword_id].get_figure_mut().position.x -= delta;
                    } else {
                        self.hero_controller.move_entity(&mut self.entities[self.hero_id], delta);
                    }
                }

                if hero_pos.x + delta + hero_size.width > self.size.width - WORLD_RIGHT_MARGIN {
                    self.view_position += Position::new(1, 0);
                }
            }

            'w' => {
                self.hero_controller.jump_entity(&mut self.entities[self.hero_id]);
            }

            's' => {
                self.hero_controller.crouch_entity(&mut self.entities[self.hero_id]);
            }

            'v' => {
                let mut fig = self.entities[self.sword_id].get_figure_mut();
                fig.visible = true;
            }

            _ => {}
        }


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
            DEBUG.into(), Color::cyan(), Position::new(WORLD_MIN_WIDTH*3+1, Y_BOTTOM)
        )));

        self.console_id = self.new_entity(Box::new(StaticEntity::new(
            "".into(), Color::white(), Position::origin()
        )));

        self.sign_ids = vec![
            self.new_entity(Box::new(StaticEntity::new(
                TUTORIAL.into(),
                Color::cyan(),
                Position::new(10, 3)
            ))),
            self.new_entity(Box::new(StaticEntity::new(
                GO_RIGHT_SIGN.into(),
                Color::cyan(),
                Position::new(78, 15)
            ))),
            self.new_entity(Box::new(StaticEntity::new(
                BOSS_SIGN.into(),
                Color::cyan(),
                Position::new(140, 3)
            ))),
            self.new_entity(Box::new(StaticEntity::new(
                END_SIGN.into(),
                Color::cyan(),
                Position::new(215, 15)
            ))),
        ];

        self.floor_id = self.new_entity(Box::new(
            StaticEntity::new(
                String::from_iter(FLOOR),
                Color::white(),
                Position::new(0, Y_BOTTOM+1)
            )
        ));

        self.hero_id = self.new_entity(self.hero_controller.new_entity());

        self.sword_id = self.new_entity(SwordController::new_entity());

        self.spike_ids = vec![
            self.new_entity(Box::new(HostileStaticEntity::new(
                SPIKE_UP.into(), Position::new(20, Y_BOTTOM), Color::red()
            ))),
            self.new_entity(Box::new(HostileStaticEntity::new(
                SPIKE_UP.into(), Position::new(21, Y_BOTTOM), Color::red()
            ))),
            self.new_entity(Box::new(HostileStaticEntity::new(
                SPIKE_DOWN.into(), Position::new(40, Y_BOTTOM-2), Color::red()
            ))),
            self.new_entity(Box::new(HostileStaticEntity::new(
                SPIKE_DOWN.into(), Position::new(40, Y_BOTTOM-4), Color::red()
            ))),
            self.new_entity(Box::new(HostileStaticEntity::new(
                SPIKE_DOWN.into(), Position::new(40, Y_BOTTOM-6), Color::red()
            ))),
            self.new_entity(Box::new(HostileStaticEntity::new(
                SPIKE_DOWN.into(), Position::new(40, Y_BOTTOM-8), Color::red()
            ))),
            self.new_entity(Box::new(HostileStaticEntity::new(
                SPIKE_UP.into(), Position::new(72, Y_BOTTOM-5), Color::red()
            ))),
            self.new_entity(Box::new(HostileStaticEntity::new(
                SPIKE_UP.into(), Position::new(72, Y_BOTTOM-7), Color::red()
            ))),
            self.new_entity(Box::new(HostileStaticEntity::new(
                SPIKE_DOWN.into(), Position::new(105, Y_BOTTOM-4), Color::red()
            ))),
            self.new_entity(Box::new(HostileStaticEntity::new(
                SPIKE_DOWN.into(), Position::new(105, Y_BOTTOM-2), Color::red()
            ))),
            self.new_entity(Box::new(HostileStaticEntity::new(
                SPIKE_DOWN.into(), Position::new(105, Y_BOTTOM), Color::red()
            ))),
        ];

        self.hostile_ids = vec![
            self.new_entity(Box::new(HostileStaticDefeatableEntity::new(
                ENEMY.into(), Position::new(70, Y_BOTTOM-2), Color::red()
            ))),
            self.new_entity(Box::new(HostileStaticDefeatableEntity::new(
                ENEMY.into(), Position::new(120, Y_BOTTOM-7), Color::red()
            ))),
        ];

        self.platform_id = self.new_entity(Box::new(StaticEntity::new(
            PLATFORM.into(), Color::new(100, 100, 100), Position::new(100, Y_BOTTOM-5)
        )));

        self.boss_id = self.new_entity(Box::new(HostileStaticEntity::new(
            BOSS.into(), Position::new(170, 13), Color::blue()
        )));

    }

    fn construct_rooms(&mut self) {
        let start_room_id = self.new_room(Size::new(WORLD_MIN_WIDTH*3, WORLD_HEIGHT));

        self.rooms[start_room_id].entities.push(self.debug_id);
        self.rooms[start_room_id].entities.push(self.console_id);
        self.rooms[start_room_id].entities.push(self.floor_id);
        self.rooms[start_room_id].entities.push(self.hero_id);
        self.rooms[start_room_id].entities.push(self.sword_id);
        self.rooms[start_room_id].entities.push(self.platform_id);

        for id in &self.sign_ids {
            self.rooms[start_room_id].entities.push(*id);
        }

        for id in &self.spike_ids {
            self.rooms[start_room_id].entities.push(*id);
        }

        for id in &self.hostile_ids {
            self.rooms[start_room_id].entities.push(*id);
        }

        self.rooms[start_room_id].entities.push(self.boss_id);

        self.current_room = start_room_id;
    }

    fn is_not_in_view_yet(&self, pos: Position) -> bool {
        pos.x < 0 || pos.y < 0
    }

    fn is_not_in_view_already(&self, pos: Position) -> bool {
        pos.x >= self.size.width || pos.y >= self.size.height
    }

    pub fn render(&mut self) {

        self.update_before_render();

        for row in &mut self.symbol_buffer {
            for symbol in row {
                *symbol = ' ';
            }
        }

        for entity_id in &self.rooms[self.current_room].entities {
            let figure = &self.entities[*entity_id].get_figure();

            if !figure.visible {
                continue;
            }
            
            for sprite in &figure.sprites {

                if !sprite.active {
                    continue;
                }

                let mut pos = (figure.position + sprite.offset).relative_to(self.view_position);
                let initial_x = pos.x;
                
                for line in sprite.content.split('\n') {

                    for (symbol, x) in line.chars().zip(initial_x .. initial_x + line.len() as i32) {
                        pos.x = x;

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

        self.update_after_render();

    }

    fn update_before_render(&mut self) {
        for ent in &mut self.entities {
            ent.animate();
        }

        if self.hero_controller.jump_potential > 0 {
            self.entities[self.hero_id].get_figure_mut().position.y -= 1;

            self.hero_controller.jump_potential -= 1;

            if self.collides_with_damage(self.hero_id) {
                self.entities[self.hero_id].get_figure_mut().position = Position::origin();
                self.hero_controller.jump_potential = 0;
                self.view_position = Position::origin();
            }

            else if self.collides(self.hero_id) {
                self.hero_controller.jump_potential = 0;
                self.entities[self.hero_id].get_figure_mut().position.y += 1;
                self.entities[self.hero_id].set_state(HERO_STATE_NORMAL);
            }

        } else {

            self.entities[self.hero_id].get_figure_mut().position.y += 1;

            if self.collides_with_damage(self.hero_id) {
                self.entities[self.hero_id].get_figure_mut().position = Position::origin();
                self.hero_controller.jump_potential = 0;
                self.view_position = Position::origin();
            }
            else if self.collides(self.hero_id) {
                self.entities[self.hero_id].get_figure_mut().position.y -= 1;

                if self.entities[self.hero_id].get_state() != HERO_STATE_CROUCHING {
                    self.entities[self.hero_id].set_state(HERO_STATE_NORMAL);
                }
            } else {
                self.entities[self.hero_id].set_state(HERO_STATE_FALLING);
            }
        }

        let sword_pos =
            self.entities[self.hero_id].get_figure().position
            + Position::new(3, 0);
        let mut fig = self.entities[self.sword_id].get_figure_mut();
        fig.position = sword_pos;

        self.defeat_with_sword();

    }

    fn update_after_render(&mut self) {
        let mut fig = self.entities[self.sword_id].get_figure_mut();
        fig.visible = false;
    }

    fn collides(&self, entity_id: EntityID) -> bool {
        for ent_id in &self.rooms[self.current_room].entities {
            if *ent_id != entity_id {
                if !self.entities[*ent_id].get_figure().visible { continue; }
                if collides(self.entities[entity_id].get_figure(), self.entities[*ent_id].get_figure()) {
                    return true;
                }
            }
        }

        false
    }

    fn collides_with_damage(&self, entity_id: EntityID) -> bool {
        for ent_id in &self.rooms[self.current_room].entities {
            if *ent_id != entity_id {
                if !self.entities[*ent_id].get_figure().visible { continue; }
                if self.entities[*ent_id].get_damage() == 0 { continue; }
                if collides(self.entities[entity_id].get_figure(), self.entities[*ent_id].get_figure()) {
                    return true;
                }
            }
        }

        false
    }

    fn defeat_with_sword(&mut self) {
        if !self.entities[self.sword_id].get_figure().visible {return;}

        for ent_id in &self.rooms[self.current_room].entities {
            if *ent_id != self.sword_id && *ent_id != self.hero_id {
                if !self.entities[*ent_id].get_figure().visible { continue; }
                if !self.entities[*ent_id].is_defeatable() { continue; }
        
                if collides(self.entities[self.sword_id].get_figure(), self.entities[*ent_id].get_figure()) {
                    self.entities[*ent_id].get_figure_mut().visible = false;
                }
            }
        }
    }

}
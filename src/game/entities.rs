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

use super::*;

use std::collections::HashMap;

pub struct StaticEntity {
    pub figure: Figure,
}

impl StaticEntity {
    pub fn new(content: String, color: Color, position: Position) -> Self {
        let size = Sprite::get_content_size(&content);

        Self {
            figure: Figure {
                position,
                sprites: vec![
                    Sprite {
                        color,
                        content,
                        offset: Position::origin(),
                        size,
                        active: true
                    }
                ],
            }
        }
    }    
}


impl Entity for StaticEntity {
    fn get_figure(&self) -> &Figure {
        &self.figure
    }

    fn get_figure_mut(&mut self) -> &mut Figure {
        &mut self.figure
    }

    fn get_size(&self) -> Size {
        self.figure.sprites[0].size
    }
}


struct AnimationPoint {
    pub duration: u32,
    pub enabled_sprites: Vec<SpriteID>
}

impl AnimationPoint {
    pub fn new(duration: u32, sprite_ids: Vec<SpriteID>) -> Self {
        Self { duration, enabled_sprites: sprite_ids }
    }
}

pub const ANIMATE_FOREVER: u32 = 0;


pub struct AnimatableEntity {
    pub figure: Figure,

    /// E.g. running/crouching/flying
    current_state: StateID,
    
    /// E.g. Running 1 / Running 2 / Running 3
    current_animation_point: usize,

    animations: HashMap<StateID, Vec<AnimationPoint>>,

    animation_call_counter: u32,
}

impl AnimatableEntity {
    pub fn new(position: Position) -> Self {
        Self {
            figure: Figure { 
                sprites: vec![],
                position,
            },
            current_state: 0,
            current_animation_point: 0,
            animations: HashMap::new(),
            animation_call_counter: 0,
        } 
    }

    pub fn add_sprite(&mut self, mut sprite: Sprite) -> SpriteID {
        sprite.active = false;
        self.figure.sprites.push(sprite);
        self.figure.sprites.len() - 1
    }

    /// Arguments:
    /// duration - number of animate() calls that the given point should be retained.
    /// If duration is ANIMATE_FOREVER (0), indicates that it is the final animation point.
    pub fn add_animation_point(&mut self, state_id: StateID, sprite_ids: Vec<SpriteID>, duration: u32) {
        if !self.animations.contains_key(&state_id) {
            self.animations.insert(state_id, vec![]);
        }
        self.animations
            .get_mut(&state_id)
            .unwrap()
            .push(
                AnimationPoint::new(duration, sprite_ids)
            );
    }

    pub fn get_state(&mut self) -> StateID {
        self.current_state
    }

    fn set_current_animation_point_active(&mut self, active: bool) {
        let animation_point =
            &self.animations
            .get_mut(&self.current_state)
            .unwrap()
            [self.current_animation_point];
        
        for sprite_id in &animation_point.enabled_sprites {
            self.figure.sprites[*sprite_id].active = active;
        }
    }

    fn get_current_animation_point_duration(&self) -> u32 {
        self.animations
            [&self.current_state]
            [self.current_animation_point]
            .duration
    }

    fn next_animation_point(&mut self) {
        self.current_animation_point += 1;
        if self.current_animation_point >= self.animations[&self.current_state].len() {
            self.current_animation_point = 0;
        }
    }

    pub fn animate(&mut self, new_state: Option<StateID>) {
        if self.animations.len() == 0 || self.figure.sprites.len() == 0 {
            return;
        }

        if (new_state == None && self.animation_call_counter > 0) {
            self.animation_call_counter -= 1;
            return;
        }

        let duration = self.get_current_animation_point_duration();
        if new_state == None && duration == ANIMATE_FOREVER {
            return; // Stop animating
        }
        
        self.set_current_animation_point_active(false);

        if new_state == None {
            self.next_animation_point();
        } else if new_state.unwrap() != self.current_state {
            self.current_state = new_state.unwrap();
            self.current_animation_point = 0;
        }

        self.animation_call_counter = self.get_current_animation_point_duration();

        self.set_current_animation_point_active(true);
    }
}


impl Entity for AnimatableEntity {
    fn get_figure(&self) -> &Figure {
        &self.figure
    }

    fn get_figure_mut(&mut self) -> &mut Figure {
        &mut self.figure
    }

    fn set_state(&mut self, state_id: StateID) {
        self.animate(Some(state_id));
    }

    fn get_state(&mut self) -> StateID {
        self.current_state
    }

    fn animate(&mut self) {
        self.animate(None);
    }

    /// HACK We get only the size of the first (0th) sprite in current state ;)
    fn get_size(&self) -> Size {
        self.figure.sprites[
            self.animations[&self.current_state][self.current_animation_point].enabled_sprites[0]
        ].size
    }
}
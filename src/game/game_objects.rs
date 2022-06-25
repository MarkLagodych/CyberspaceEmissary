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
use super::entities::*;

pub struct SpellConsole {
    spell: String
}

impl SpellConsole {
    pub fn new() -> Self {
        Self { spell: "".into() }
    }

    pub fn add_char(&mut self, ch: char) {
        self.spell.push(ch);
    }

    pub fn backspace(&mut self) {
        if self.spell.len() > 0 {
            self.spell.pop();
        }
    }

    pub fn get_len(&self) -> usize {
        self.spell.len()
    }

    pub fn get_spell(&self) -> String {
        self.spell.clone()
    }

    pub fn finish_spell(&mut self) -> String {
        let s = self.spell.clone();
        self.spell.clear();
        s
    }
}


pub struct HeroController {
    pub direction_right: bool,
    pub health: u32,
    pub jump_potential: i32,
}

pub const HERO_STATE_NORMAL: usize = 0;
pub const HERO_STATE_CROUCHING: usize = 1;
pub const HERO_STATE_JUMPING_RIGHT: usize = 2;
pub const HERO_STATE_JUMPING_LEFT: usize = 3;
pub const HERO_STATE_FALLING: usize = 4;

impl HeroController {
    pub fn new() -> Self {
        Self { health: 10, direction_right: true, jump_potential: 0 }
    }

    pub fn new_entity(&self) -> Box<dyn Entity> {
        let hero_size = Sprite::get_content_size(HERO);
        let hero_crouching2_size = Sprite::get_content_size(HERO_CROUCHING_2);
        
        let mut ent = Box::new(
            AnimatableEntity::new(Position::origin())
        );

        let staying = ent.add_sprite(Sprite {
            color: Color::magenta(),
            content: HERO.into(),
            offset: Position::origin(), 
            size: hero_size,
            active: true
        });

        let crouching_1 = ent.add_sprite(Sprite {
            color: Color::magenta(),
            content: HERO_CROUCHING_1.into(),
            offset: Position::origin(), 
            size: hero_size,
            active: true
        });

        let crouching_2 = ent.add_sprite(Sprite {
            color: Color::magenta(),
            content: HERO_CROUCHING_2.into(),
            offset: Position::new(0, 1), 
            size: hero_crouching2_size,
            active: true
        });

        let jumping_left = ent.add_sprite(Sprite {
            color: Color::magenta(),
            content: HERO_JUMPING_LEFT.into(),
            offset: Position::origin(), 
            size: hero_size,
            active: true
        });

        let jumping_right = ent.add_sprite(Sprite {
            color: Color::magenta(),
            content: HERO_JUMPING_RIGHT.into(),
            offset: Position::origin(), 
            size: hero_size,
            active: true
        });

        let falling = ent.add_sprite(Sprite {
            color: Color::magenta(),
            content: HERO_FALL.into(),
            offset: Position::origin(), 
            size: hero_size,
            active: true
        });

        ent.add_animation_point(HERO_STATE_NORMAL, vec![staying], ANIMATE_FOREVER);
        
        ent.add_animation_point(HERO_STATE_CROUCHING, vec![crouching_1], 4);
        ent.add_animation_point(HERO_STATE_CROUCHING, vec![crouching_2], ANIMATE_FOREVER);

        ent.add_animation_point(HERO_STATE_JUMPING_LEFT, vec![jumping_left], ANIMATE_FOREVER);
        ent.add_animation_point(HERO_STATE_JUMPING_RIGHT, vec![jumping_right], ANIMATE_FOREVER);

        ent.add_animation_point(HERO_STATE_FALLING, vec![falling], ANIMATE_FOREVER);

        ent.set_state(HERO_STATE_NORMAL);
        
        ent
    }

    pub fn move_entity(&mut self, _self: &mut Box<dyn Entity>, dir: i32) {
        self.direction_right = dir > 0;

        if vec![HERO_STATE_JUMPING_LEFT, HERO_STATE_JUMPING_RIGHT].contains(&_self.get_state()) {
            if self.direction_right {
                _self.set_state(HERO_STATE_JUMPING_RIGHT);
            } else {
                _self.set_state(HERO_STATE_JUMPING_LEFT);
            }
        }
    }

    pub fn jump_entity(&mut self, _self: &mut Box<dyn Entity>) {
        if _self.get_state() == HERO_STATE_CROUCHING {
            _self.set_state(HERO_STATE_NORMAL);
        } else {
            if self.jump_potential == 0
            && _self.get_state() != HERO_STATE_FALLING
            && _self.get_state() != HERO_STATE_JUMPING_LEFT
            && _self.get_state() != HERO_STATE_JUMPING_RIGHT {
                self.jump_potential = HERO_JUMPING_HEIGHT;
            }

            if self.direction_right {
                _self.set_state(HERO_STATE_JUMPING_RIGHT);
            } else {
                _self.set_state(HERO_STATE_JUMPING_LEFT);
            }
        }
    }

    pub fn crouch_entity(&self, _self: &mut Box<dyn Entity>) {
        _self.set_state(HERO_STATE_CROUCHING);
    }
}


pub const SWORD_STATE_NORMAL: usize = 0;

pub struct SwordController {

}

impl SwordController {
    pub fn new_entity() -> Box<dyn Entity> {
        let size = Size::new(3, 3);
        let mut ent = Box::new(
            AnimatableEntity::new(Position::new(0, Y_BOTTOM - size.height))
        );

        ent.figure.visible = false;

        let common = Sprite {
            color: Color::yellow(),
            content: "".into(),
            offset: Position::origin(),
            size,
            active: false
        };

        let mut sprite_ids = vec![
            ent.add_sprite(Sprite { content: SWORD_1.into(), ..common }),
            ent.add_sprite(Sprite { content: SWORD_2.into(), ..common }),
            ent.add_sprite(Sprite { content: SWORD_3.into(), ..common }),
            ent.add_sprite(Sprite { content: SWORD_4.into(), ..common }),
            ent.add_sprite(Sprite { content: SWORD_5.into(), ..common }),
        ];

        for id in &sprite_ids {
            ent.add_animation_point(SWORD_STATE_NORMAL, vec![*id], 2);
        }

        ent.set_state(SWORD_STATE_NORMAL);

        ent
    }
}



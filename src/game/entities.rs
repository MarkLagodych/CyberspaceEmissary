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



pub struct AnimatableEntity {
    pub figure: Figure,

    /// E.g. running/crouching/flying
    current_state: StateID,
    
    /// E.g. Running 1 / Running 2 / Running 3
    current_animation: usize,

    animations: HashMap<StateID, Vec<Vec<SpriteID>>>,
}

impl AnimatableEntity {
    pub fn new(position: Position) -> Self {
        Self {
            figure: Figure { 
                sprites: vec![],
                position,
            },
            current_state: 0,
            current_animation: 0,
            animations: HashMap::new()
        } 
    }

    pub fn add_sprite(&mut self, mut sprite: Sprite) -> SpriteID {
        sprite.active = false;
        self.figure.sprites.push(sprite);
        self.figure.sprites.len() - 1
    }

    pub fn add_animation_point(&mut self, state_id: StateID, sprite_ids: Vec<SpriteID>) {
        if !self.animations.contains_key(&state_id) {
            self.animations.insert(state_id, vec![]);
        }
        self.animations.get_mut(&state_id).unwrap().push(sprite_ids);
    }

    pub fn set_state(&mut self, state_id: StateID) {
        self.current_state = state_id;
        self.current_animation = 0;
        self.set_current_animation_active(true);
    }

    pub fn get_state(&mut self) -> StateID {
        self.current_state
    }

    fn set_current_animation_active(&mut self, active: bool) {
        let mut sprite_ids =
            &mut self.animations
            .get_mut(&self.current_state)
            .unwrap()
            [self.current_animation];
        
        for sprite_id in sprite_ids {
            self.figure.sprites[*sprite_id].active = active;
        }
    }

    pub fn animate(&mut self) {
        if self.animations.len() == 0 {
            return;
        }

        if self.figure.sprites.len() == 0 {
            return;
        }

        self.set_current_animation_active(false);

        self.current_animation += 1;
        if self.current_animation >= self.animations[&self.current_state].len() {
            self.current_animation = 0;
        }

        self.set_current_animation_active(true);
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
        self.set_state(state_id);
    }

    fn get_state(&mut self) -> StateID {
        self.current_state
    }

    fn animate(&mut self) {
        self.animate();
    }

    fn get_size(&self) -> Size {
        // HACK We get only the size of the first (0th) sprite in current state ;)
        self.figure.sprites[
            self.animations[&self.current_state][self.current_animation][0]
        ].size
    }
}
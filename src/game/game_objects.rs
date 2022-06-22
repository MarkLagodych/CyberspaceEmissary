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


pub struct Hero {

}

pub const HERO_STATE_NORMAL: usize = 0;

impl Hero {
    pub fn new_entity() -> Box<dyn Entity> {
        let mut ent = AnimatableEntity::new(Position::origin());

        let hero_size = Sprite::get_content_size(HERO);

        let staying_id = ent.add_sprite(Sprite {
            color: Color::magenta(),
            content: HERO.into(),
            offset: Position::new(0, WORLD_HEIGHT-1 - hero_size.height), 
            size: hero_size,
            active: true
        });

        ent.add_animation_point(HERO_STATE_NORMAL, vec![staying_id]);

        ent.set_state(HERO_STATE_NORMAL);
        
        Box::new(ent)
    }

    pub fn move_entity(ent: &mut Box<dyn Entity>, dir: i32) {
        ent.get_figure_mut().position += Position::new(dir, 0);

        // if ent.get_state() != HERO_STATE_MOVING_RIGHT {
        //     ent.set_state(HERO_STATE_MOVING_RIGHT);
        // }
        
        ent.animate();
    }
}
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
        let hero_size = Sprite::get_content_size(HERO);
        
        let mut ent = Box::new(
            AnimatableEntity::new(Position::new(0, WORLD_HEIGHT-1 - hero_size.height))
        );

        let staying_id = ent.add_sprite(Sprite {
            color: Color::magenta(),
            content: HERO.into(),
            offset: Position::origin(), 
            size: hero_size,
            active: true
        });

        ent.add_animation_point(HERO_STATE_NORMAL, vec![staying_id], 1);

        ent.set_state(HERO_STATE_NORMAL);
        
        ent
    }

    pub fn move_entity(ent: &mut Box<dyn Entity>, dir: i32) {
        ent.get_figure_mut().position += Position::new(dir, 0);

        // if ent.get_state() != HERO_STATE_MOVING_RIGHT {
        //     ent.set_state(HERO_STATE_MOVING_RIGHT);
        // }
        
        ent.animate();
    }
}


pub const SWORD_STATE_NORMAL: usize = 0;

pub struct Sword {

}

impl Sword {
    pub fn new_entity() -> Box<dyn Entity> {
        let size = Size::new(3, 3);
        let mut ent = Box::new(
            AnimatableEntity::new(Position::new(0, WORLD_HEIGHT-1 - size.height - 2))
        );

        let common = Sprite {
            color: Color::yellow(),
            content: "".into(),
            offset: Position::origin(),
            size,
            active: true
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
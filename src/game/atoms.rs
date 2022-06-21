use super::system_entity::SystemEntity;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn black() -> Self {
        Self::new(0, 0, 0)
    }

    pub fn white() -> Self {
        Self::new(255, 255, 255)
    }
}


pub struct Sprite {
    pub color: Color,
    pub content: String,
    pub offset: Position,
    pub size: Size,
}

impl Sprite {
    pub fn new() -> Self {
        Self {
            color: Color::white(),
            content: "".into(),
            offset: Position::origin(), 
            size: Size::new(0, 0)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn origin() -> Self {
        Self::new(0, 0)
    }

    pub fn is_in_view(&self, view_size: &Size) -> bool {
        self.x >= 0
        && self.y >= 0
        && self.x < view_size.width
        && self.y < view_size.height
    }
}

use std::ops;
impl ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

impl Size {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }

    pub fn fits_in(&self, other: &Self) -> bool {
        self.width <= other.width && self.height <= other.height
    }
}

impl PartialEq for Size {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }

    fn ne(&self, other: &Self) -> bool {
        self.width != other.width || self.height != other.height
    }
}


/// Graphical Object
pub struct Figure {
    pub sprites: Vec<Sprite>,
    pub position: Position,
}

impl Figure {
    pub fn new() -> Self {
        Self { sprites: vec![], position: Position::origin() }
    }
}


pub trait Entity {
    fn get_figure(&self) -> &Figure;

    fn get_system_entity_mut(&mut self) -> Option<&mut SystemEntity> {
        None
    }

    /// Returns: (destination room number, destination X coordinate)
    fn teleport(&self) -> Option<(usize, i32)> {
        None
    }
}


pub type ID = usize;

pub struct Room {
    pub entities: Vec<ID>,
}

impl Room {
    pub fn new() -> Self {
        Self {
            entities: vec![]
        }
    }
}


pub struct StaticEntity {
    gobject: Figure,
}

impl StaticEntity {
    pub fn new(content: String, color: Color, position: Position) -> Self {
        let mut size = Size::new(0, 0);

        for line in content.split('\n') {
            size.height += 1;
            size.width = size.width.max(line.len() as i32);
        }

        Self {
            gobject: Figure {
                position,
                sprites: vec![
                    Sprite {
                        color,
                        content,
                        offset: Position::origin(),
                        size
                    }
                ],
            }
        }
    }    
}


impl Entity for StaticEntity {
    fn get_figure(&self) -> &Figure {
        &self.gobject
    }
}
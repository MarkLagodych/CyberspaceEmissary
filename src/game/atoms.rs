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

    pub fn red() -> Self {
        Self::new(255, 0, 0)
    }
    
    pub fn green() -> Self {
        Self::new(0, 255, 0)
    }

    pub fn blue() -> Self {
        Self::new(0, 0, 255)
    }

    pub fn magenta() -> Self {
        Self::new(255, 0, 255)
    }

    pub fn yellow() -> Self {
        Self::new(255, 255, 0)
    }

    pub fn cyan() -> Self {
        Self::new(0, 255, 255)
    }
}


pub struct Sprite {
    pub color: Color,
    pub content: String,
    pub offset: Position,
    pub size: Size,
    pub active: bool,
}

impl Sprite {
    pub fn new() -> Self {
        Self {
            color: Color::white(),
            content: "".into(),
            offset: Position::origin(), 
            size: Size::new(0, 0),
            active: true,
        }
    }

    pub fn get_content_size(content: &str) -> Size {
        let mut size = Size::new(0, 0);
    
        for line in content.split('\n') {
            size.height += 1;
            size.width = size.width.max(line.len() as i32);
        }
    
        size
    }

    pub fn get_sprite_size(sprite: &Self) -> Size {
        Self::get_content_size(sprite.content.as_str())
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

    pub fn relative_to(self, other: Self) -> Self {
        self - other
    }
}

use std::ops;

use super::ascii_art::{WORLD_HEIGHT, WORLD_MIN_WIDTH};
impl ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl ops::Sub<Position> for Position {
    type Output = Position;

    fn sub(self, other: Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl ops::AddAssign<Position> for Position {
    fn add_assign(&mut self, rhs: Position) {
        self.x += rhs.x;
        self.y += rhs.y;
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
    fn get_figure_mut(&mut self) -> &mut Figure;
    fn animate(&mut self) { }
    fn set_state(&mut self, state_id: StateID) { }
    fn get_state(&mut self) -> StateID { 0 }
    fn get_size(&self) -> Size;
}


pub struct Room {
    pub entities: Vec<EntityID>,
    pub size: Size,
}

impl Room {
    pub fn new(size: Size) -> Self {
        Self {
            entities: vec![],
            size,
        }
    }
}

/// Index in an array
pub type EntityID = usize;

/// Index in an array
pub type RoomID = usize;

/// Index in an array
pub type SpriteID = usize;

/// Index in an array
pub type StateID = usize;
use super::system_entity::SystemEntity;

#[derive(Clone, Copy)]
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


pub struct Renderable {
    pub color: Color,
    pub content: String,
    pub offset: Position,
    pub size: Size,
}

impl Renderable {
    pub fn new() -> Self {
        Self {
            color: Color::white(),
            content: "".into(),
            offset: Position::origin(), 
            size: Size::new(0, 0)
        }
    }
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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


/// Checks whether rectangle is fully inside the view
pub fn rect_is_fully_inside(view_size: &Size, rect_pos: &Position, rect_size: &Size) -> bool {
    if rect_pos.x < 0
    || rect_pos.y < 0
    || rect_pos.x + rect_size.width >= view_size.width
    || rect_pos.y + rect_size.height >= view_size.height
    {
        false
    }
    else
    {
        true
    }
}


pub struct Character {
    pub renderables: Vec<Renderable>,
    pub position: Position,
}

impl Character {
    pub fn new() -> Self {
        Self { renderables: vec![], position: Position::origin() }
    }
}


pub trait Entity {
    fn get_character(&self) -> &Character;

    fn get_system_entity_mut(&mut self) -> Option<&mut SystemEntity> {
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



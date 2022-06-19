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

pub struct Game {
    pub symbol_map: Vec<Vec<char>>,
    pub color_map: Vec<Vec<Color>>,

    width: usize,
    height: usize,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        let mut symbol_map = vec![vec!['*'; width]; height];
        let mut color_map = vec![vec![Color::black(); width]; height];

        for i in 0..height {
            for j in 0..width {
                color_map[i][j] = Color::new(i as u8, j as u8, 127);
            }
        }

        Self { symbol_map, color_map, width, height }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.symbol_map = vec![vec!['*'; width]; height];
        self.color_map =  vec![vec![Color::black(); width]; height];
    }
}
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


#![allow(non_snake_case)]

use crate::game::*;
use crate::special_key_codes::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

#[wasm_bindgen]
pub struct GameRunner {
    ctx: web_sys::CanvasRenderingContext2d,
    canvas_size: Size,
    game: Box<Game>,
}


#[wasm_bindgen]
impl GameRunner {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Self {
            ctx: context,
            canvas_size: Size::new(900, 600),
            game: Box::new(Game::new(Size::new(100, 30)))
        }
    }

    pub fn has_stopped(&self) -> bool {
        self.game.stopped
    }

    pub fn update(&mut self) {
        self.game.render();
        self.draw();        
    }

    pub fn handle_key(&mut self, key: char) {
        self.game.process_key(key, false);
    } 

    fn draw(&mut self) {
        self.ctx.clear_rect(0., 0., self.canvas_size.width as f64, self.canvas_size.height as f64);
        self.ctx.set_font("14px monospace");

        let font_metrics = self.ctx.measure_text("W").unwrap();
        let font_width = font_metrics.width();
        let font_height = font_metrics.font_bounding_box_ascent();

        for y in 0..self.game.size.height as usize {
            // console::log_1(&JsValue::from(format!("{}", y)));

            for x in 0..self.game.size.width as usize {
                let color = &self.game.color_buffer[y][x];
                let symbol = self.game.symbol_buffer[y][x];

                self.ctx.set_fill_style(&JsValue::from(
                    format!("#{:02x}{:02x}{:02x}", color.r, color.g, color.b)
                ));

                self.ctx.fill_text(
                    symbol.to_string().as_str(),
                    x as f64 * font_width,
                    y as f64 * font_height
                );
            }
        }
    }
}
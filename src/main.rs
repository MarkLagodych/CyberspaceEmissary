#![allow(unused)]
#![allow(dead_code)]

pub mod game;

mod terminal;
use terminal::game_runner::GameRunner;

fn main() {
    let mut game_runner = GameRunner::new();
    game_runner.run();
}

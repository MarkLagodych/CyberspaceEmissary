#![allow(unused)]
#![allow(dead_code)]
#![cfg(feature="terminal_backend")]
#![deprecated]

pub mod game;
pub mod special_key_codes;

mod terminal;
use terminal::GameRunner;


fn main() {
    let mut game_runner = GameRunner::new();
    game_runner.run();
}

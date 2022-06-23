#![allow(unused)]
#![allow(dead_code)]

#![cfg(feature="wasm_backend")]

pub mod game;
pub mod special_key_codes;

mod wasm;
pub use wasm::GameRunner;
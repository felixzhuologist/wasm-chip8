//! A CHIP-8 interpreter
//! 
//! This library is a focused on simplicity, meant to be a starter project for
//! learning Rust. It is based on Matthew Mikolay's great [Mastering CHIP-8][1]
//! guide and implements all 35 of the original CHIP-8 instructions.
//! 
//! [1]: http://mattmik.com/files/chip8/mastering/chip8.html
extern crate wasm_bindgen;
extern crate js_sys;

pub use self::wasm::CPUWrapper;

pub mod cpu;
pub mod wasm;
pub mod keypad;
pub mod screen;
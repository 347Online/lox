#![feature(derive_from)]
pub mod chunk;
pub mod compiler;
#[cfg(debug_assertions)]
pub mod debug;
#[warn(clippy::pedantic)]
pub mod scanner;
pub mod stack;
pub mod value;
pub mod vm;

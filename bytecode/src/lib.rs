#![feature(derive_from)]
#[warn(clippy::pedantic)]
pub mod chunk;
#[cfg(debug_assertions)]
pub mod debug;
pub mod stack;
pub mod value;
pub mod vm;

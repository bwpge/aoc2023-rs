//! Shared library for Advent of Code 2023.

pub mod cli;
pub mod coordinate;
pub mod fsutils;
pub mod grid;
pub mod macros;
pub mod solutions;

pub use coordinate::{Coordinate, Direction};
pub use grid::Grid;

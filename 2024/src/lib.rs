use std::fs::read_to_string;

pub mod dijkstra;
pub mod geometry;
pub mod grid;
pub mod parser;
pub mod util;

pub fn read(day: u8) -> std::io::Result<String> {
    read_to_string(format!("input/day{}.txt", day))
}

use std::fs::read_to_string;

pub mod math;
pub mod parser;

pub fn read(day: u8) -> std::io::Result<String> {
    read_to_string(format!("input/day{}.txt", day))
}

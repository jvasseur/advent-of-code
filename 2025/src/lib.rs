use std::fs::read_to_string;

pub mod counter;
pub mod dijkstra;
pub mod grid;
pub mod parser;

pub fn read(day: u8) -> std::io::Result<String> {
    read_to_string(format!("input/day{}.txt", day))
}

pub fn gcd(x: i32, y: i32) -> i32 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

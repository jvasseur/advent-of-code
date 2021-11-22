use std::fs;

pub mod day1;
pub mod day2;

pub mod util;

fn main() {
    let input = fs::read_to_string("input/day2.txt").expect("Failed to read input file");
    let parsed_input = day2::parse(&input).expect("Failed to parse input");

    println!("{}", day2::solve_part1(&parsed_input));
    println!("{}", day2::solve_part2(&parsed_input));
}

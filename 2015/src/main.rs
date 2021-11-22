use std::fs;

pub mod day1;

pub mod util;

fn main() {
    let input = fs::read_to_string("input/day1.txt").expect("Failed to read input file");
    let parsed_input = day1::parse(&input).expect("Failed to parse input");

    println!("{}", day1::solve_part1(&parsed_input));
    println!("{}", day1::solve_part2(&parsed_input));
}

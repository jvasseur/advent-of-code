use advent_of_code_2018::{read, Parsable};
use nom::IResult;
use nom::character::complete::i32;
use nom::bytes::complete::tag;
use nom::sequence::terminated;
use nom::multi::many0;
use nom::combinator::map;
use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    changes: Vec<i32>,
}

impl Input {
    fn new(changes: impl Into<Vec<i32>>) -> Self {
        Self {
            changes: changes.into(),
        }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        map(many0(terminated(i32, tag("\n"))), Input::new)(input)
    }
}

fn solve_part1(input: &Input) -> i32 {
    input.changes.iter().sum()
}

fn solve_part2(input: &Input) -> i32 {
    let mut frequency = 0;
    let mut seen = HashSet::new();

    for change in input.changes.iter().cycle() {
        frequency += change;

        if seen.contains(&frequency) {
            return frequency;
        }

        seen.insert(frequency);
    }

    panic!();
}

fn main() {
    let input = read(1);
    let parsed = Input::parse(&input).unwrap();

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "+1
-2
+3
+1
";

    fn parsed_input() -> Input {
        Input::new([1, -2, 3, 1])
    }

    #[test]
    fn test_parser() {
        assert_eq!(Input::parse(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 3);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 2);
    }
}

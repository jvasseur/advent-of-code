use advent_of_code_2018::{read, Parsable};
use nom::bytes::complete::tag;
use nom::sequence::terminated;
use nom::multi::many0;
use nom::combinator::map;
use nom::character::complete::alpha1;
use nom::IResult;
use itertools::Itertools;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    boxes: Vec<Box>,
}

impl Input {
    fn new(boxes: impl Into<Vec<Box>>) -> Self {
        Self {
            boxes: boxes.into(),
        }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        map(many0(terminated(Box::parser, tag("\n"))), Input::new)(input)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Box {
    id: String,
}

impl Box {
    fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
        }
    }
}

impl Parsable for Box {
    fn parser(input: &str) -> IResult<&str, Self> {
        map(alpha1, Box::new)(input)
    }
}

fn solve_part1(input: &Input) -> usize {
    input.boxes.iter().filter(|b| b.id.chars().counts().values().contains(&2)).count()
        * input.boxes.iter().filter(|b| b.id.chars().counts().values().contains(&3)).count()
}

fn solve_part2(input: &Input) -> usize {
    0
}

fn main() {
    let input = read(2);
    let parsed = Input::parse(&input).unwrap();

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab
";

    fn parsed_input() -> Input {
        Input::new([
            Box::new("abcdef"),
            Box::new("bababc"),
            Box::new("abbcde"),
            Box::new("abcccd"),
            Box::new("aabcdd"),
            Box::new("abcdee"),
            Box::new("ababab"),
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(Input::parse(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 12);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 0);
    }
}

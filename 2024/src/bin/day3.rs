use advent_of_code_2024::{parser::*, read};
use nom::{branch::alt, bytes::complete::tag, character::complete::{anychar, u32}, combinator::{map, value}, multi::many1, sequence::tuple, IResult};

#[derive(Clone, PartialEq, Eq, Debug)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

impl Parsable for Instruction {
    fn parser(input: &str) -> IResult<&str, Self> {
        alt((
            map( tuple((tag("mul("), u32, tag(","), u32, tag(")"))), |(_, a, _, b, _)| Instruction::Mul(a, b)),
            value(Instruction::Do, tag("do()")),
            value(Instruction::Dont, tag("don't()")),
        ))(input)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    instructions: Vec<Instruction>,
}

impl Input {
    fn new(instructions: impl Into<Vec<Instruction>>) -> Self {
        Self { instructions: instructions.into() }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, content) = many1(alt((
            map(Instruction::parser, |instruction| Some(instruction)),
            value(None, anychar),
        )))(input)?;

        let instruction = content.into_iter().filter_map(|value| value).collect::<Vec<Instruction>>();

        Ok((input, Input::new(instruction)))
    }
}

fn solve_part1(input: &Input) -> u32 {
    let mut result = 0;

    for instruction in &input.instructions {
        match instruction {
            Instruction::Mul(a, b) => {
                result += a * b;
            },
            Instruction::Do => {},
            Instruction::Dont => {},
        }

    }

    result
}

fn solve_part2(input: &Input) -> u32 {
    let mut result = 0;
    let mut enabled = true;

    for instruction in &input.instructions {
        match instruction {
            Instruction::Mul(a, b) => {
                if enabled {
                    result += a * b;
                }
            },
            Instruction::Do => {
                enabled = true;
            },
            Instruction::Dont => {
                enabled = false;
            },
        }
    }

    result
}

fn main() {
    let input = parse(&read(3).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    fn parsed_input() -> Input {
        Input::new([
            Instruction::Mul(2, 4),
            Instruction::Dont,
            Instruction::Mul(5, 5),
            Instruction::Mul(11, 8),
            Instruction::Do,
            Instruction::Mul(8, 5),
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse::<Input>(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 161);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 48);
    }
}

use advent_of_code_2015::{parser::*, read};
use nom::{branch::alt, character::complete::char, combinator::{map, value}, multi::many1};

#[derive(Clone, Debug, Eq, PartialEq)]
enum Instruction {
    Up,
    Down,
}

impl Parsable for Instruction {
    fn parser<'a>(input: &'a str) -> ParserResult<'a, Self> {
        alt((
            value(Instruction::Up, char('(')),
            value(Instruction::Down, char(')')),
        ))(input)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    instructions: Vec<Instruction>,
}

impl Input {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self { instructions }
    }
}

impl Parsable for Input {
    fn parser<'a>(input: &'a str) -> ParserResult<'a, Self> {
        map(many1(Instruction::parser), Input::new)(input)
    }
}

fn solve_part1(input: &Input) -> i32 {
    input.instructions.iter().fold(0, |floor, instruction: &Instruction| match instruction {
        Instruction::Up => floor + 1,
        Instruction::Down => floor - 1,
    })
}

fn solve_part2(input: &Input) -> u32 {
    let mut index: u32 = 0;
    let mut floor: i32 = 0;

    for instruction in &input.instructions {
        index = index + 1;
        floor = match instruction {
            Instruction::Up => floor + 1,
            Instruction::Down => floor - 1,
        };

        if floor < 0 {
            return index;
        }
    }

    panic!("Here be dragons");
}

fn main() {
    let input = parse(&read(1).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(parse::<Input>("(())"), Ok(Input::new(vec![
            Instruction::Up,
            Instruction::Up,
            Instruction::Down,
            Instruction::Down,
        ])));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&Input::new(vec![
            Instruction::Up,
            Instruction::Up,
            Instruction::Down,
            Instruction::Down,
        ])), 0);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&Input::new(vec![
            Instruction::Up,
            Instruction::Down,
            Instruction::Down,
            Instruction::Up,
        ])), 3);
    }
}

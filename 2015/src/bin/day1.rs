use advent_of_code_2015::{read, parse};
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::value;
use nom::multi::many0;
use nom::IResult;

#[derive(Clone,Debug,Eq,PartialEq)]
enum Instruction {
    Up,
    Down,
}

fn parser(input: &str) -> IResult<&str, Vec<Instruction>> {
    many0(alt((
        value(Instruction::Up, char('(')),
        value(Instruction::Down, char(')')),
    )))(input)
}

fn solve_part1(input: &[Instruction]) -> i32 {
    input.iter().fold(0, |floor, instruction: &Instruction| match instruction {
        Instruction::Up => floor + 1,
        Instruction::Down => floor - 1,
    })
}

fn solve_part2(input: &[Instruction]) -> u32 {
    let mut index: u32 = 0;
    let mut floor: i32 = 0;

    for instruction in input {
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
    let input = read(1);

    let parsed_input = parse(parser, &input);

    println!("{}", solve_part1(&parsed_input));
    println!("{}", solve_part2(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::Instruction;
    use super::parser;
    use super::solve_part1;
    use super::solve_part2;

    #[test]
    fn test_parse() {
        assert_eq!(parser("(())"), Ok(("", vec![
            Instruction::Up,
            Instruction::Up,
            Instruction::Down,
            Instruction::Down,
        ])));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&vec![
            Instruction::Up,
            Instruction::Up,
            Instruction::Down,
            Instruction::Down,
        ]), 0);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&vec![
            Instruction::Up,
            Instruction::Down,
            Instruction::Down,
            Instruction::Up,
        ]), 3);
    }
}

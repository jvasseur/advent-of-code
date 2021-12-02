use advent_of_code_2021::{read, parse_lines};
use nom::IResult;
use nom::branch::alt;
use nom::combinator::map;
use nom::combinator::value;
use nom::character::complete::char;
use nom::character::complete::u32;
use nom::bytes::complete::tag;
use nom::sequence::separated_pair;

#[derive(Clone,Debug,Eq,PartialEq)]
enum Direction {
    Forward,
    Up,
    Down,
}

#[derive(Clone,Debug,Eq,PartialEq)]
struct Instruction {
    direction: Direction,
    value: u32,
}

fn direction_parser(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Forward, tag("forward")),
        value(Direction::Up, tag("up")),
        value(Direction::Down, tag("down")),
    ))(input)
}

fn instruction_parser(input: &str) -> IResult<&str, Instruction> {
    map(separated_pair(direction_parser, char(' '), u32), |(direction, value)| Instruction { direction, value })(input)
}

fn solve_part1(input: &[Instruction]) -> u32 {
    let mut depth = 0;
    let mut position = 0;

    for instruction in input {
        match instruction.direction {
            Direction::Forward => position += instruction.value,
            Direction::Up => depth -= instruction.value,
            Direction::Down => depth += instruction.value,
        }
    }

    depth * position
}

fn solve_part2(input: &[Instruction]) -> u32 {
    let mut aim = 0;
    let mut depth = 0;
    let mut position = 0;

    for instruction in input {
        match instruction.direction {
            Direction::Forward => {
                position += instruction.value;
                depth += aim * instruction.value;
            },
            Direction::Up => aim -= instruction.value,
            Direction::Down => aim += instruction.value,
        }
    }

    depth * position
}

fn main() {
    let input = read(2);

    let parsed_input = parse_lines(instruction_parser, &input);

    println!("{}", solve_part1(&parsed_input));
    println!("{}", solve_part2(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::Direction;
    use super::Instruction;
    use super::instruction_parser;
    use super::solve_part1;
    use super::solve_part2;

    #[test]
    fn test_instruction_parser() {
        assert_eq!(instruction_parser("forward 4"), Ok(("", Instruction {
            direction: Direction::Forward,
            value: 4,
        })));
        assert_eq!(instruction_parser("up 2"), Ok(("", Instruction {
            direction: Direction::Up,
            value: 2,
        })));
        assert_eq!(instruction_parser("down 43"), Ok(("", Instruction {
            direction: Direction::Down,
            value: 43,
        })));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&vec![
            Instruction { direction: Direction::Forward, value: 5 },
            Instruction { direction: Direction::Down, value: 5 },
            Instruction { direction: Direction::Forward, value: 8 },
            Instruction { direction: Direction::Up, value: 3 },
            Instruction { direction: Direction::Down, value: 8 },
            Instruction { direction: Direction::Forward, value: 2 },
        ]), 150);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&vec![
            Instruction { direction: Direction::Forward, value: 5 },
            Instruction { direction: Direction::Down, value: 5 },
            Instruction { direction: Direction::Forward, value: 8 },
            Instruction { direction: Direction::Up, value: 3 },
            Instruction { direction: Direction::Down, value: 8 },
            Instruction { direction: Direction::Forward, value: 2 },
        ]), 900);
    }
}

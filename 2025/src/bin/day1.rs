use advent_of_code_2025::{parser::*, read};
use derive_more::IntoIterator;
use nom::{IResult, branch::alt, bytes::complete::tag, combinator::{map, value}};

#[derive(Clone, PartialEq, Eq, Debug)]
enum Direction {
    Left,
    Right,
}

impl Parsable for Direction {
    fn parser(input: &str) -> IResult<&str, Self> {
        alt((
            value(Direction::Left, tag("L")),
            value(Direction::Right, tag("R")),
        ))(input)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Rotation {
    direction: Direction,
    count: u16,
}

impl Rotation {
    fn new(direction: Direction, count: u16) -> Self {
        Self { direction, count }
    }
}

impl Parsable for Rotation {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, direction) = parse(input)?;
        let (input, count) = parse(input)?;

        Ok((input, Rotation::new(direction, count)))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, IntoIterator)]
#[into_iterator(owned, ref, ref_mut)]
struct Input(Vec<Rotation>);

impl Input {
    fn new(rotations: impl Into<Vec<Rotation>>) -> Self {
        Self(rotations.into())
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        map(
            parse_lines,
            Input::new,
        )(input)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Dial {
    value: u16,
}

impl Dial {
    fn new() -> Self {
        Self { value: 50 }
    }

    fn rotate(&mut self, rotation: &Rotation) -> u16 {
        let value = (self.value as i16) + match rotation.direction {
            Direction::Left => -(rotation.count as i16),
            Direction::Right => rotation.count as i16,
        };

        let mut zeros: i16 = 0;

        // The overflow would re-count the fact that we endend on 0 the previous rotation so we have to remove it
        if rotation.direction == Direction::Left && self.value == 0 {
            zeros -= 1;
        }

        self.value = value.rem_euclid(100) as u16;

        zeros += value.div_euclid(100).abs();

        // We ended on 0 without an overflow so we have to add it
        if rotation.direction == Direction::Left && self.value == 0 {
            zeros += 1;
        }

        zeros as u16
    }
}

fn solve_part1(input: &Input) -> u16 {
    let mut password = 0;
    let mut dial = Dial::new();

    for rotation in input {
        dial.rotate(&rotation);

        if dial.value == 0 {
            password += 1;
        }
    }

    password
}

fn solve_part2(input: &Input) -> u16 {
    let mut password: u16 = 0;
    let mut dial = Dial::new();

    for rotation in input {
        password += dial.rotate(&rotation);
    }

    password
}

fn main() {
    let input = from_str(&read(1).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    fn parsed_input() -> Input {
        Input::new([
            Rotation::new(Direction::Left, 68),
            Rotation::new(Direction::Left, 30),
            Rotation::new(Direction::Right, 48),
            Rotation::new(Direction::Left, 5),
            Rotation::new(Direction::Right, 60),
            Rotation::new(Direction::Left, 55),
            Rotation::new(Direction::Left, 1),
            Rotation::new(Direction::Left, 99),
            Rotation::new(Direction::Right, 14),
            Rotation::new(Direction::Left, 82),
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse(INPUT), Ok(("", parsed_input())));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 3);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 6);
    }
}

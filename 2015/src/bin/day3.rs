use advent_of_code_2015::{read, parse};
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::value;
use nom::multi::many0;
use nom::IResult;
use std::collections::HashSet;

#[derive(Clone,Debug,Eq,PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    fn move_in(&self, direction: &Direction) -> Position {
        match direction {
            Direction::North => Position {
                y: self.y + 1,
                x: self.x,
            },
            Direction::South => Position {
                y: self.y - 1,
                x: self.x,
            },
            Direction::East => Position {
                y: self.y,
                x: self.x + 1,
            },
            Direction::West => Position {
                y: self.y,
                x: self.x - 1,
            },
        }
    }
}

fn parser(input: &str) -> IResult<&str, Vec<Direction>> {
    many0(alt((
        value(Direction::North, char('^')),
        value(Direction::South, char('v')),
        value(Direction::East, char('>')),
        value(Direction::West, char('<')),
    )))(input)
}

fn solve_part1(input: &[Direction]) -> usize {
    let mut position = Position {
        x: 0,
        y: 0,
    };

    let mut visited = HashSet::new();

    visited.insert(position);

    for direction in input {
        position = position.move_in(direction);

        visited.insert(position);
    }

    visited.len()
}

fn solve_part2(input: &[Direction]) -> usize {
    let mut santa = Position {
        x: 0,
        y: 0,
    };
    let mut robot_santa = Position {
        x: 0,
        y: 0,
    };

    let mut visited = HashSet::new();

    visited.insert(santa);

    for directions in input.chunks(2) {
        santa = santa.move_in(&directions[0]);
        robot_santa = robot_santa.move_in(&directions[1]);

        visited.insert(santa);
        visited.insert(robot_santa);
    }

    visited.len()
}

fn main() {
    let input = read(3);

    let parsed_input = parse(parser, &input);

    println!("{}", solve_part1(&parsed_input));
    println!("{}", solve_part2(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::Direction;
    use super::parser;
    use super::solve_part1;
    use super::solve_part2;

    #[test]
    fn test_parse() {
        assert_eq!(parser("^>v<"), Ok(("", vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ])));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&vec![
            Direction::East,
        ]), 2);
        assert_eq!(solve_part1(&vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]), 4);
        assert_eq!(solve_part1(&vec![
            Direction::North,
            Direction::South,
            Direction::North,
            Direction::South,
            Direction::North,
            Direction::South,
            Direction::North,
            Direction::South,
            Direction::North,
            Direction::South,
        ]), 2);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&vec![
            Direction::North,
            Direction::South,
        ]), 3);
        assert_eq!(solve_part2(&vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]), 3);
        assert_eq!(solve_part2(&vec![
            Direction::North,
            Direction::South,
            Direction::North,
            Direction::South,
            Direction::North,
            Direction::South,
            Direction::North,
            Direction::South,
            Direction::North,
            Direction::South,
        ]), 11);
    }
}

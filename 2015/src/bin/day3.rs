use advent_of_code_2015::{parser::*, read};
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::{map, value};
use nom::multi::many0;
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Parsable for Direction {
    fn parser<'a>(input: &'a str) -> ParserResult<'a, Self> {
        alt((
            value(Direction::North, char('^')),
            value(Direction::South, char('v')),
            value(Direction::East, char('>')),
            value(Direction::West, char('<')),
        ))(input)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Input {
    directions: Vec<Direction>,
}

impl Input {
    fn new(directions: Vec<Direction>) -> Self {
        Self { directions }
    }
}

impl Parsable for Input {
    fn parser<'a>(input: &'a str) -> ParserResult<'a, Self> {
        map(many0(Direction::parser), Input::new)(input)
    }
}

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
struct Position {
    x: i32,
    y: i32,
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

fn solve_part1(input: &Input) -> usize {
    let mut position = Position {
        x: 0,
        y: 0,
    };

    let mut visited = HashSet::new();

    visited.insert(position);

    for direction in &input.directions {
        position = position.move_in(direction);

        visited.insert(position);
    }

    visited.len()
}

fn solve_part2(input: &Input) -> usize {
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

    for directions in (&input.directions).chunks(2) {
        santa = santa.move_in(&directions[0]);
        robot_santa = robot_santa.move_in(&directions[1]);

        visited.insert(santa);
        visited.insert(robot_santa);
    }

    visited.len()
}

fn main() {
    let input = parse(&read(3).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(parse::<Input>("^>v<"), Ok(Input::new(vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ])));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&Input::new(vec![
            Direction::East,
        ])), 2);
        assert_eq!(solve_part1(&Input::new(vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ])), 4);
        assert_eq!(solve_part1(&Input::new(vec![
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
        ])), 2);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&Input::new(vec![
            Direction::North,
            Direction::South,
        ])), 3);
        assert_eq!(solve_part2(&Input::new(vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ])), 3);
        assert_eq!(solve_part2(&Input::new(vec![
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
        ])), 11);
    }
}

use std::collections::{HashMap, HashSet};

use advent_of_code_2025::{counter::Counter, grid::{Grid, Point}, parser::*, read};
use nom::{IResult, branch::alt, bytes::complete::tag, combinator::{map, value}};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
enum Value {
    #[default]
    Empty,
    Split,
    Start,
}

impl Parsable for Value {
    fn parser(input: &str) -> IResult<&str, Self> {
        alt((
           value(Value::Empty, tag(".")),
           value(Value::Split, tag("^")),
           value(Value::Start, tag("S")),
        ))(input)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Input(Grid<Value>);

impl Input {
    fn new(values: impl Into<Grid<Value>>) -> Self {
        Self(values.into())
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        map(parse_grid, Input::new)(input)
    }
}

fn solve_part1(input: &Input) -> usize {
    let start = input.0.points().find(|point| input.0.get(point) == &Value::Start).unwrap();

    let mut beams = HashSet::from([start]);
    let mut split = 0;

    for _ in 0..input.0.rows() {
        let mut new_beams = HashSet::new();

        for beam in beams {
            let new_position = Point { col: beam.col, row: beam.row + 1 };

            match input.0.get(&new_position) {
                Value::Empty => {
                    new_beams.insert(new_position);
                },
                Value::Split => {
                    new_beams.insert(Point { col: new_position.col - 1, row: new_position.row });
                    new_beams.insert(Point { col: new_position.col + 1, row: new_position.row });

                    split += 1;
                },
                _ => panic!("We are back to the start, this shouldn't append")
            }
        }

        beams = new_beams;
    }

    split
}

fn solve_part2(input: &Input) -> usize {
    let start = input.0.points().find(|point| input.0.get(point) == &Value::Start).unwrap();

    let mut beams = Counter::from([(start, 1)]);

    for _ in 0..input.0.rows() {
        let mut new_beams = Counter::new();

        for (beam, &count) in beams.counts() {
            let new_position = Point { col: beam.col, row: beam.row + 1 };

            match input.0.get(&new_position) {
                Value::Empty => {
                    new_beams.add_count(&new_position, count);
                },
                Value::Split => {
                    new_beams.add_count(&Point { col: new_position.col - 1, row: new_position.row }, count);
                    new_beams.add_count(&Point { col: new_position.col + 1, row: new_position.row }, count);
                },
                _ => panic!("We are back to the start, this shouldn't append")
            }
        }

        beams = new_beams;
    }

    beams.counts().map(|(_, count)| count).sum()
}

fn main() {
    let input = from_str(&read(7).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    fn parsed_input() -> Input {
        Input::new(vec![
            vec![Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Start, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty],
            vec![Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty],
            vec![Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Split, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty],
            vec![Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty],
            vec![Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Split, Value::Empty, Value::Split, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty],
            vec![Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty],
            vec![Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Split, Value::Empty, Value::Split, Value::Empty, Value::Split, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty],
            vec![Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty],
            vec![Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Split, Value::Empty, Value::Split, Value::Empty, Value::Empty, Value::Empty, Value::Split, Value::Empty, Value::Empty, Value::Empty, Value::Empty],
            vec![Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty],
            vec![Value::Empty, Value::Empty, Value::Empty, Value::Split, Value::Empty, Value::Split, Value::Empty, Value::Empty, Value::Empty, Value::Split, Value::Empty, Value::Split, Value::Empty, Value::Empty, Value::Empty],
            vec![Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty],
            vec![Value::Empty, Value::Empty, Value::Split, Value::Empty, Value::Empty, Value::Empty, Value::Split, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Split, Value::Empty, Value::Empty],
            vec![Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty],
            vec![Value::Empty, Value::Split, Value::Empty, Value::Split, Value::Empty, Value::Split, Value::Empty, Value::Split, Value::Empty, Value::Split, Value::Empty, Value::Empty, Value::Empty, Value::Split, Value::Empty],
            vec![Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty],
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse(INPUT), Ok(("", parsed_input())));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 21);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 40);
    }
}

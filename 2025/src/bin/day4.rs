use advent_of_code_2025::{grid::{Direction, Grid, Point}, parser::*, read};
use nom::{IResult, branch::alt, bytes::complete::tag, combinator::{map, value}};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Value {
    Roll,
    Empty,
}

impl Default for Value {
    fn default() -> Self {
        Value::Empty
    }
}

impl Parsable for Value {
    fn parser(input: &str) -> IResult<&str, Self> {
        alt((
            value(Value::Roll, tag("@")),
            value(Value::Empty, tag(".")),
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
        map(
            parse_grid,
            Input::new,
        )(input)
    }
}

fn can_be_moved(grid: &Grid<Value>, point: &Point) -> bool {
    Direction::VALUES.into_iter()
        .filter(|direction| grid.get(&(point + direction * 1)) == &Value::Roll)
        .count() < 4
}

fn solve_part1(input: &Input) -> usize {
    input.0.points()
        .filter(|&point| input.0.get(&point) == &Value::Roll)
        .filter(|&point| can_be_moved(&input.0, &point))
        .count()
}

fn solve_part2(input: &Input) -> usize {
    let mut changed = true;
    let mut grid = input.0.clone();
    let mut removed = 0;

    while changed {
        changed = false;

        for point in grid.points() {
            if grid.get(&point) == &Value::Empty || !can_be_moved(&grid, &point) {
                continue;
            }

            grid.set(&point, Value::Empty);
            changed = true;
            removed += 1;
        }
    }

    removed
}

fn main() {
    let input = from_str(&read(4).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    fn parsed_input() -> Input {
        Input::new(vec![
            vec![Value::Empty, Value::Empty, Value::Roll, Value::Roll, Value::Empty, Value::Roll, Value::Roll, Value::Roll, Value::Roll, Value::Empty],
            vec![Value::Roll, Value::Roll, Value::Roll, Value::Empty, Value::Roll, Value::Empty, Value::Roll, Value::Empty, Value::Roll, Value::Roll],
            vec![Value::Roll, Value::Roll, Value::Roll, Value::Roll, Value::Roll, Value::Empty, Value::Roll, Value::Empty, Value::Roll, Value::Roll],
            vec![Value::Roll, Value::Empty, Value::Roll, Value::Roll, Value::Roll, Value::Roll, Value::Empty, Value::Empty, Value::Roll, Value::Empty],
            vec![Value::Roll, Value::Roll, Value::Empty, Value::Roll, Value::Roll, Value::Roll, Value::Roll, Value::Empty, Value::Roll, Value::Roll],
            vec![Value::Empty, Value::Roll, Value::Roll, Value::Roll, Value::Roll, Value::Roll, Value::Roll, Value::Roll, Value::Empty, Value::Roll],
            vec![Value::Empty, Value::Roll, Value::Empty, Value::Roll, Value::Empty, Value::Roll, Value::Empty, Value::Roll, Value::Roll, Value::Roll],
            vec![Value::Roll, Value::Empty, Value::Roll, Value::Roll, Value::Roll, Value::Empty, Value::Roll, Value::Roll, Value::Roll, Value::Roll],
            vec![Value::Empty, Value::Roll, Value::Roll, Value::Roll, Value::Roll, Value::Roll, Value::Roll, Value::Roll, Value::Roll, Value::Empty],
            vec![Value::Roll, Value::Empty, Value::Roll, Value::Empty, Value::Roll, Value::Roll, Value::Roll, Value::Empty, Value::Roll, Value::Empty],
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse(INPUT), Ok(("", parsed_input())));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 13);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 43);
    }
}

use std::collections::HashSet;

use advent_of_code_2024::{grid::{Direction, Grid, Point}, parser::*, read};
use nom::{bytes::complete::tag, character::complete::none_of, multi::many1, sequence::terminated, IResult};

#[derive(Clone, PartialEq, Eq, Debug)]
enum Element {
    Empty,
    Obstruction,
    Outside,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    grid: Grid<Element>,
    position: Point,
    direction: Direction,
}

impl Input {
    fn new(grid: impl Into<Grid<Element>>, position: impl Into<Point>, direction: impl Into<Direction>) -> Self {
        Self { grid: grid.into(), position: position.into(), direction: direction.into() }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, data) = many1(terminated(many1(none_of("\n")), tag("\n")))(input)?;

        let mut position = None;
        let mut grid = Grid::new_fill(data.len(), data[0].len(), Element::Outside);

        for (row, row_data) in data.iter().enumerate() {
            for (col, char) in row_data.iter().enumerate() {
                let point = Point { row: row as i32, col: col as i32 };

                if char == &'^' {
                    position = Some(point);
                }

                if char == &'#' {
                    grid.set(&point, Element::Obstruction);
                } else {
                    grid.set(&point, Element::Empty);
                }
            }
        }

        Ok((input, Input::new(grid, position.unwrap(), Direction::Up)))
    }
}

fn move_guard(grid: &Grid<Element>, (position, direction): &(Point, Direction)) -> Option<(Point, Direction)> {
    let forward = position + direction * 1;

    match grid.get(&forward) {
        Element::Obstruction => {
            Some((position.to_owned(), match direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                _ => panic!("Invalid direction"),
            }))
        },
        Element::Empty => {
            Some((forward, direction.to_owned()))
        },
        Element::Outside => {
            None
        },
    }

}

fn solve_part1(input: &Input) -> usize {
    let mut visited = Grid::new_fill(input.grid.rows(), input.grid.cols(), false);
    let mut state = (input.position, input.direction);

    loop {
        visited.set(&state.0, true);

        match move_guard(&input.grid, &state) {
            None => break,
            Some(new_state) => state = new_state,
        }
    }

    visited.values().filter(|&&value| value).count()
}

fn solve_part2(input: &Input) -> usize {
    let mut available_obstructions = 0;

    for point in input.grid.points() {
        if point == input.position {
            continue;
        }

        let mut grid = input.grid.clone();
        grid.set(&point, Element::Obstruction);

        let mut visited = HashSet::new();

        let mut state = (input.position, input.direction);

        loop {
            visited.insert(state);

            match move_guard(&grid, &state) {
                None => break,
                Some(new_state) => state = new_state,
            }

            if visited.contains(&state) {
                available_obstructions += 1;

                break;
            }
        }
    }

    available_obstructions
}

fn main() {
    let input = parse(&read(6).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    fn parsed_input() -> Input {
        let mut grid = Grid::new_fill(10, 10, Element::Outside);

        for point in grid.points() {
            grid.set(&point, Element::Empty);
        }

        grid.set(&Point::new(0, 4), Element::Obstruction);
        grid.set(&Point::new(1, 9), Element::Obstruction);
        grid.set(&Point::new(3, 2), Element::Obstruction);
        grid.set(&Point::new(4, 7), Element::Obstruction);
        grid.set(&Point::new(6, 1), Element::Obstruction);
        grid.set(&Point::new(7, 8), Element::Obstruction);
        grid.set(&Point::new(8, 0), Element::Obstruction);
        grid.set(&Point::new(9, 6), Element::Obstruction);

        Input::new(grid, Point { row: 6, col: 4 }, Direction::Up)
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse::<Input>(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 41);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 6);
    }
}

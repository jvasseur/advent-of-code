use std::{collections::HashMap, ops::Rem};

use advent_of_code_2023::{read, Parsable, util::{Grid, Point}};
use nom::{IResult, combinator::value, combinator::map, branch::alt, bytes::complete::tag, multi::many0, sequence::terminated};

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Input {
    grid: Grid<Option<Rock>>,
}

impl Input {
    fn new(grid: impl Into<Grid<Option<Rock>>>) -> Self {
        Self { grid: grid.into() }
    }

    fn tilted_north(&self) -> Self {
        let mut grid = self.grid.clone();

        for col in 0..grid.cols() {
            for row in 1..grid.rows() {
                let point = Point { col, row };

                if grid.get(&point) != &Some(Rock::Rounded) {
                    continue;
                }

                let mut row_up = row;
                while row_up > 0 && grid.get(&Point { col, row: row_up - 1 }) == &None {
                    row_up -= 1;
                }

                if row_up != row {
                    let point_up = Point { col, row: row_up };

                    grid.set(&point, None);
                    grid.set(&point_up, Some(Rock::Rounded));
                }
            }
        }

        Self { grid }
    }

    fn tilted_south(&self) -> Self {
        let mut grid = self.grid.clone();

        for col in 0..grid.cols() {
            for row in (0..grid.rows() - 1).rev() {
                let point = Point { col, row };

                if grid.get(&point) != &Some(Rock::Rounded) {
                    continue;
                }

                let mut row_down = row;
                while row_down < grid.rows() - 1 && grid.get(&Point { col, row: row_down + 1 }) == &None {
                    row_down += 1;
                }

                if row_down != row {
                    let point_down = Point { col, row: row_down };

                    grid.set(&point, None);
                    grid.set(&point_down, Some(Rock::Rounded));
                }
            }
        }

        Self { grid }
    }

    fn tilted_west(&self) -> Self {
        let mut grid = self.grid.clone();

        for row in 0..grid.rows() {
            for col in 1..grid.cols() {
                let point = Point { col, row };

                if grid.get(&point) != &Some(Rock::Rounded) {
                    continue;
                }

                let mut col_left = col;
                while col_left > 0 && grid.get(&Point { col: col_left - 1, row }) == &None {
                    col_left -= 1;
                }

                if col_left != col {
                    let point_left = Point { col: col_left, row };

                    grid.set(&point, None);
                    grid.set(&point_left, Some(Rock::Rounded));
                }
            }
        }

        Self { grid }
    }

    fn tilted_east(&self) -> Self {
        let mut grid = self.grid.clone();

        for row in 0..grid.rows() {
            for col in (0..grid.cols() - 1).rev() {
                let point = Point { col, row };

                if grid.get(&point) != &Some(Rock::Rounded) {
                    continue;
                }

                let mut col_right = col;
                while col_right < grid.cols() - 1 && grid.get(&Point { col: col_right + 1, row }) == &None {
                    col_right += 1;
                }

                if col_right != col {
                    let point_right = Point { col: col_right, row };

                    grid.set(&point, None);
                    grid.set(&point_right, Some(Rock::Rounded));
                }
            }
        }

        Self { grid }
    }

    fn tilted_cycle(&self) -> Self {
        self.tilted_north().tilted_west().tilted_south().tilted_east()
    }

    fn load(&self) -> usize {
        let mut load = 0;

        for col in 0..self.grid.cols() {
            for row in 0..self.grid.rows() {
                if self.grid.get(&Point { col, row }) == &Some(Rock::Rounded) {
                    load += self.grid.rows() - row
                }
            }
        }

        load
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        map(
            many0(
                terminated(
                    many0(
                        alt((
                            value(None, tag(".")),
                            map(Rock::parser, |rock| Some(rock)),
                        )),
                    ),
                    tag("\n"),
                ),
            ),
            Self::new,
        )(input)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Rock {
    Rounded,
    Cube,
}

impl Parsable for Rock {
    fn parser(input: &str) -> IResult<&str, Self> {
        alt((
            value(Self::Rounded, tag("O")),
            value(Self::Cube, tag("#")),
        ))(input)
    }
}

fn solve_part1(input: &Input) -> usize {
    input.tilted_north().load()
}

fn solve_part2(input: &Input) -> usize {
    let cycles = 1000000000;

    let mut map = HashMap::new();
    let mut map_rev = HashMap::new();

    let mut loop_start = None;
    let mut loop_end = None;

    let mut current = input.clone();

    for i in 0..cycles {
        map.insert(current.clone(), i);
        map_rev.insert(i, current.clone());

        let tilted = current.tilted_cycle();

        if let Some(&start) = map.get(&tilted) {
            loop_start = Some(start);
            loop_end = Some(i + 1);

            break;
        }

        current = tilted;
    }

    let loop_start = loop_start.unwrap();
    let loop_end = loop_end.unwrap();
    let loop_size = loop_end - loop_start;

    let remaining_cycles = cycles - loop_start;
    let last_cycles = remaining_cycles.rem(loop_size);

    let last = map_rev.get(&(loop_start + last_cycles)).unwrap();

    last.load()
}

fn main() {
    let input = read(14);
    let parsed = Input::parse(&input).unwrap();

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    fn parsed_input() -> Input {
        Input::new(vec![
            vec![Some(Rock::Rounded), None, None, None, None, Some(Rock::Cube), None, None, None, None],
            vec![Some(Rock::Rounded), None, Some(Rock::Rounded), Some(Rock::Rounded), Some(Rock::Cube), None, None, None, None, Some(Rock::Cube)],
            vec![None, None, None, None, None, Some(Rock::Cube), Some(Rock::Cube), None, None, None],
            vec![Some(Rock::Rounded), Some(Rock::Rounded), None, Some(Rock::Cube), Some(Rock::Rounded), None, None, None, None, Some(Rock::Rounded)],
            vec![None, Some(Rock::Rounded), None, None, None, None, None, Some(Rock::Rounded), Some(Rock::Cube), None],
            vec![Some(Rock::Rounded), None, Some(Rock::Cube), None, None, Some(Rock::Rounded), None, Some(Rock::Cube), None, Some(Rock::Cube)],
            vec![None, None, Some(Rock::Rounded), None, None, Some(Rock::Cube), Some(Rock::Rounded), None, None, Some(Rock::Rounded)],
            vec![None, None, None, None, None, None, None, Some(Rock::Rounded), None, None],
            vec![Some(Rock::Cube), None, None, None, None, Some(Rock::Cube), Some(Rock::Cube), Some(Rock::Cube), None, None],
            vec![Some(Rock::Cube), Some(Rock::Rounded), Some(Rock::Rounded), None, None, Some(Rock::Cube), None, None, None, None],
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(Input::parse(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_tilted_north() {
        assert_eq!(parsed_input().tilted_north(), Input::new(vec![
            vec![Some(Rock::Rounded), Some(Rock::Rounded), Some(Rock::Rounded), Some(Rock::Rounded), None, Some(Rock::Cube), None, Some(Rock::Rounded), None, None],
            vec![Some(Rock::Rounded), Some(Rock::Rounded), None, None, Some(Rock::Cube), None, None, None, None, Some(Rock::Cube)],
            vec![Some(Rock::Rounded), Some(Rock::Rounded), None, None, Some(Rock::Rounded), Some(Rock::Cube), Some(Rock::Cube), None, None, Some(Rock::Rounded)],
            vec![Some(Rock::Rounded), None, None, Some(Rock::Cube), None, Some(Rock::Rounded), Some(Rock::Rounded), None, None, None],
            vec![None, None, None, None, None, None, None, None, Some(Rock::Cube), None],
            vec![None, None, Some(Rock::Cube), None, None, None, None, Some(Rock::Cube), None, Some(Rock::Cube)],
            vec![None, None, Some(Rock::Rounded), None, None, Some(Rock::Cube), None, Some(Rock::Rounded), None, Some(Rock::Rounded)],
            vec![None, None, Some(Rock::Rounded), None, None, None, None, None, None, None],
            vec![Some(Rock::Cube), None, None, None, None, Some(Rock::Cube), Some(Rock::Cube), Some(Rock::Cube), None, None],
            vec![Some(Rock::Cube), None, None, None, None, Some(Rock::Cube), None, None, None, None],
        ]));
    }

    #[test]
    fn test_tilted_cycle_1() {
        assert_eq!(parsed_input().tilted_cycle(), Input::parse(".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....
").unwrap());
    }

    #[test]
    fn test_tilted_cycle_2() {
        assert_eq!(parsed_input().tilted_cycle().tilted_cycle(), Input::parse(".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O
").unwrap());
    }

    #[test]
    fn test_tilted_cycle_3() {
        assert_eq!(parsed_input().tilted_cycle().tilted_cycle().tilted_cycle(), Input::parse(".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O
").unwrap());
    }

    #[test]
    fn test_tilted_find_cycle() {
        assert_eq!(
            parsed_input()
                .tilted_cycle()
                .tilted_cycle()
                .tilted_cycle(),
            parsed_input()
                .tilted_cycle()
                .tilted_cycle()
                .tilted_cycle()
                .tilted_cycle()
                .tilted_cycle()
                .tilted_cycle()
                .tilted_cycle()
                .tilted_cycle()
                .tilted_cycle()
                .tilted_cycle(),
        );
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 136);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 64);
    }
}

use std::collections::HashSet;
use advent_of_code_2023::{read, Parsable};
use advent_of_code_2023::util::Point;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::IResult;
use nom::combinator::value;
use nom::multi::many0;
use nom::sequence::terminated;
use nom::branch::alt;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    lines: Vec<Vec<bool>>,
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, lines) = many0(terminated(many0(alt((
            value(true, tag("#")),
            value(false, tag(".")),
        ))), tag("\n")))(input)?;

        Ok((input, Input { lines }))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Universe {
    galaxies: HashSet<Point>,
}

impl Universe {
    fn expand(&self, scale: usize) -> Universe {
        let rows: HashSet<usize> = self.galaxies.iter().map(|point| point.row).collect();
        let cols: HashSet<usize> = self.galaxies.iter().map(|point| point.col).collect();

        let empty_rows = (*rows.iter().min().unwrap()..*rows.iter().max().unwrap())
            .filter(|row| !rows.contains(row))
            .collect_vec();

        let empty_cols = (*cols.iter().min().unwrap()..*cols.iter().max().unwrap())
            .filter(|col| !cols.contains(col))
            .collect_vec();

        let galaxies = self.galaxies.iter().map(|&Point { row, col }| Point {
            row: row + empty_rows.iter().filter(|&&empty_row| empty_row < row).count() * (scale - 1),
            col: col + empty_cols.iter().filter(|&&empty_col| empty_col < col).count() * (scale - 1),
        }).collect();

        Universe { galaxies }
    }
}

impl From<&Input> for Universe {
    fn from(value: &Input) -> Self {
        let galaxies = value.lines.iter()
            .enumerate()
            .map(|(row, line)| line.iter()
                .enumerate()
                .filter(|(_, is_galaxy)| **is_galaxy)
                .map(move |(col, _)| Point { row, col })
            )
            .flatten()
            .collect();

        Universe { galaxies }
    }
}

fn solve_scale(input: &Input, scale: usize) -> usize {
    let universe = Universe::from(input);

    universe.expand(scale).galaxies.into_iter().combinations(2).map(|galaxies| {
        let a = galaxies[0];
        let b = galaxies[1];

        a.row.abs_diff(b.row) + a.col.abs_diff(b.col)
    }).sum()
}

fn solve_part1(input: &Input) -> usize {
    solve_scale(input, 2)
}

fn solve_part2(input: &Input) -> usize {
    solve_scale(input, 1000000)
}

fn main() {
    let input = read(11);
    let parsed = Input::parse(&input).unwrap();

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    fn parsed_input() -> Input {
        Input {
            lines: vec![
                vec![false, false, false, true, false, false, false, false, false, false],
                vec![false, false, false, false, false, false, false, true, false, false],
                vec![true, false, false, false, false, false, false, false, false, false],
                vec![false, false, false, false, false, false, false, false, false, false],
                vec![false, false, false, false, false, false, true, false, false, false],
                vec![false, true, false, false, false, false, false, false, false, false],
                vec![false, false, false, false, false, false, false, false, false, true],
                vec![false, false, false, false, false, false, false, false, false, false],
                vec![false, false, false, false, false, false, false, true, false, false],
                vec![true, false, false, false, true, false, false, false, false, false],
            ],
        }
    }

    fn example_universe() -> Universe {
        Universe {
            galaxies: HashSet::from([
                Point { row: 0, col: 3 },
                Point { row: 1, col: 7 },
                Point { row: 2, col: 0 },
                Point { row: 4, col: 6 },
                Point { row: 5, col: 1 },
                Point { row: 6, col: 9 },
                Point { row: 8, col: 7 },
                Point { row: 9, col: 0 },
                Point { row: 9, col: 4 },
            ]),
        }
    }

    #[test]
    fn test_parser() {
        assert_eq!(Input::parse(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_universe_from() {
        assert_eq!(Universe::from(&parsed_input()), example_universe());
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 374);
    }

    #[test]
    fn test_solve_10() {
        assert_eq!(solve_scale(&parsed_input(), 10), 1030);
    }

    #[test]
    fn test_solve_100() {
        assert_eq!(solve_scale(&parsed_input(), 100), 8410);
    }
}

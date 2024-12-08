use std::collections::HashMap;

use advent_of_code_2024::{grid::{Grid, Point, Vector}, parser::*, read, util::gcd};
use itertools::Itertools;
use nom::{branch::alt, bytes::complete::tag, character::complete::none_of, combinator::{map, value}, multi::many1, sequence::terminated, IResult};

type Frequency = char;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    grid: Grid<Option<Frequency>>,
}

impl Input {
    fn new(grid: impl Into<Grid<Option<Frequency>>>) -> Self {
        Self { grid: grid.into() }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, data) = many1(terminated(many1(alt((
            value(None, tag(".")),
            map(none_of("\n"), |char| Some(char)),
        ))), tag("\n")))(input)?;

        Ok((input, Input::new(data)))
    }
}

fn get_antenas_by_frequency(input: &Input) -> HashMap<char, Vec<Point>> {
    let mut antenas_by_frequency = HashMap::new();

    for point in input.grid.points() {
        if let Some(frequency) = input.grid.get(&point) {
            if !antenas_by_frequency.contains_key(frequency) {
                antenas_by_frequency.insert(*frequency, Vec::new());
            }

            antenas_by_frequency.get_mut(&frequency).unwrap().push(point);
        }
    }

    antenas_by_frequency
}

fn solve_part1(input: &Input) -> usize {
    let antenas_by_frequency = get_antenas_by_frequency(&input);

    let mut antinodes = Grid::new_fill(input.grid.rows(), input.grid.cols(), false);

    for (_, antenas) in antenas_by_frequency {
        for (antena_a, antena_b) in antenas.iter().tuple_combinations() {
            let diff = antena_b - antena_a;

            let antinode_a = antena_a - diff;
            let antinode_b = antena_b + diff;

            if antinodes.is_in_bounds(&antinode_a) {
                antinodes.set(&antinode_a, true);
            }

            if antinodes.is_in_bounds(&antinode_b) {
                antinodes.set(&antinode_b, true);
            }
        }
    }

    antinodes.values().filter(|&&value| value).count()
}

fn solve_part2(input: &Input) -> usize {
    let antenas_by_frequency = get_antenas_by_frequency(&input);

    let mut antinodes = Grid::new_fill(input.grid.rows(), input.grid.cols(), false);

    for (_, antenas) in antenas_by_frequency {
        for (antena_a, antena_b) in antenas.iter().tuple_combinations() {
            let diff = antena_b - antena_a;
            let vec_gcd = gcd(diff.row, diff.col);
            let vec = Vector { row: diff.row / vec_gcd, col: diff.col / vec_gcd };

            let mut point = antena_a.to_owned();

            while antinodes.is_in_bounds(&point) {
                antinodes.set(&point, true);
                point -= vec;
            }

            let mut point = antena_a.to_owned();

            while antinodes.is_in_bounds(&point) {
                antinodes.set(&point, true);
                point += vec;
            }
        }
    }

    antinodes.values().filter(|&&value| value).count()
}

fn main() {
    let input = parse(&read(8).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    fn parsed_input() -> Input {
        Input::new(vec![
            vec![None, None, None, None, None, None, None, None, None, None, None, None],
            vec![None, None, None, None, None, None, None, None, Some('0'), None, None, None],
            vec![None, None, None, None, None, Some('0'), None, None, None, None, None, None],
            vec![None, None, None, None, None, None, None, Some('0'), None, None, None, None],
            vec![None, None, None, None, Some('0'), None, None, None, None, None, None, None],
            vec![None, None, None, None, None, None, Some('A'), None, None, None, None, None],
            vec![None, None, None, None, None, None, None, None, None, None, None, None],
            vec![None, None, None, None, None, None, None, None, None, None, None, None],
            vec![None, None, None, None, None, None, None, None, Some('A'), None, None, None],
            vec![None, None, None, None, None, None, None, None, None, Some('A'), None, None],
            vec![None, None, None, None, None, None, None, None, None, None, None, None],
            vec![None, None, None, None, None, None, None, None, None, None, None, None],
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse::<Input>(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 14);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 34);
    }
}

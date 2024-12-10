use std::collections::HashSet;

use advent_of_code_2024::{grid::{Direction, Grid, Point}, parser::*, read};
use nom::IResult;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    map: Grid<u8>
}

impl Input {
    fn new(map: impl Into<Grid<u8>>) -> Self {
        Self { map: map.into() }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, map) = grid_parser(input)?;

        Ok((input, Input::new(map)))
    }
}

fn solve_part1(input: &Input) -> usize {
    let mut scores = 0;

    for point in input.map.points() {
        if input.map.get(&point) != &0 {
            continue;
        }

        let mut to_check = vec![point];

        let mut checked = HashSet::new();
        let mut reachable_nines = HashSet::new();

        while let Some(point) = to_check.pop() {
            let point_height = *input.map.get(&point);

            for direction in [
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left,
            ] {
                let point_to_check = point + direction * 1;

                if checked.contains(&point_to_check) {
                    continue;
                }

                if !input.map.is_in_bounds(&point_to_check) {
                    continue;
                }

                let point_to_check_height = *input.map.get(&point_to_check);

                if point_to_check_height != point_height + 1 {
                    continue;
                }

                if point_to_check_height == 9 {
                    reachable_nines.insert(point_to_check);
                } else {
                    to_check.push(point_to_check);
                }
            }

            checked.insert(point);
        }

        scores += reachable_nines.len();
    }

    scores
}

fn get_rating(map: &Grid<u8>, point: &Point) -> usize {
    let mut rating = 0;

    let point_height = *map.get(&point);

    for direction in [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ] {
        let point_to_check = point + direction * 1;

        if !map.is_in_bounds(&point_to_check) {
            continue;
        }

        let point_to_check_height = *map.get(&point_to_check);

        if point_to_check_height != point_height + 1 {
            continue;
        }

        if point_to_check_height == 9 {
            rating += 1;
        } else {
            rating += get_rating(map, &point_to_check);
        }
    }

    rating
}

fn solve_part2(input: &Input) -> usize {
    let mut ratings = 0;

    for point in input.map.points() {
        if input.map.get(&point) != &0 {
            continue;
        }

        ratings += get_rating(&input.map, &point)
    }

    ratings
}

fn main() {
    let input = parse(&read(10).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    fn parsed_input() -> Input {
        Input::new(vec![
            vec![8, 9, 0, 1, 0, 1, 2, 3],
            vec![7, 8, 1, 2, 1, 8, 7, 4],
            vec![8, 7, 4, 3, 0, 9, 6, 5],
            vec![9, 6, 5, 4, 9, 8, 7, 4],
            vec![4, 5, 6, 7, 8, 9, 0, 3],
            vec![3, 2, 0, 1, 9, 0, 1, 2],
            vec![0, 1, 3, 2, 9, 8, 0, 1],
            vec![1, 0, 4, 5, 6, 7, 3, 2],
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse::<Input>(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 36);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 81);
    }
}

use std::{collections::HashSet, hash::Hash};

use advent_of_code_2025::{parser::*, read};
use derive_more::IntoIterator;
use itertools::Itertools;
use nom::{IResult, bytes::complete::tag, combinator::map, sequence::tuple};

type Coordinate = u64;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point(Coordinate, Coordinate, Coordinate);

impl Point {
    fn new(a: Coordinate, b: Coordinate, c: Coordinate) -> Self {
        Self(a, b, c)
    }
}

impl Parsable for Point {
    fn parser(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                parse,
                tag(","),
                parse,
                tag(","),
                parse,
            )),
            |(a, _, b, _, c)| Point::new(a, b, c),
        )(input)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, IntoIterator)]
#[into_iterator(owned, ref, ref_mut)]
struct Input(Vec<Point>);

impl Input {
    fn new(values: impl Into<Vec<Point>>) -> Self {
        Self(values.into())
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

fn euclidian_distance(a: &Point, b: &Point) -> f32 {
    ((a.0 as f32 - b.0 as f32).powi(2) + (a.1 as f32 - b.1 as f32).powi(2) + (a.2 as f32 - b.2 as f32).powi(2)).sqrt()
}

fn solve_part1(input: &Input, connections: usize) -> usize {
    let distances = input.into_iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b, euclidian_distance(a, b)))
        .sorted_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap())
        .collect_vec();

    let mut groups = input.into_iter()
        .map(|a| HashSet::from([a]))
        .collect_vec();

    for i in 0..connections {
        let (a, b, _) = distances[i];

        let (connected, mut rest): (Vec<_>, Vec<_>) = groups.into_iter()
            .partition(|group| group.contains(a) || group.contains(b));

        let mut group = HashSet::new();
        for set in connected {
            group.extend(set);
        }

        rest.push(group);

        groups = rest;
    }

    groups.into_iter()
        .map(|group| group.len())
        .sorted()
        .rev()
        .take(3)
        .product()
}

fn solve_part2(input: &Input) -> Coordinate {
    let distances = input.into_iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b, euclidian_distance(a, b)))
        .sorted_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap())
        .collect_vec();

    let mut groups = input.into_iter()
        .map(|a| HashSet::from([a]))
        .collect_vec();

    for (a, b, _) in distances {
        let (connected, mut rest): (Vec<_>, Vec<_>) = groups.into_iter()
            .partition(|group| group.contains(a) || group.contains(b));

        let mut group = HashSet::new();
        for set in connected {
            group.extend(set);
        }

        rest.push(group);

        groups = rest;

        if groups.len() == 1 {
            return a.0 * b.0;
        }
    }

    panic!("Here be dragons");
}

fn main() {
    let input = from_str(&read(8).unwrap()).unwrap();

    println!("{}", solve_part1(&input, 1_000));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    fn parsed_input() -> Input {
        Input::new([
            Point::new(162, 817, 812),
            Point::new(57, 618, 57),
            Point::new(906, 360, 560),
            Point::new(592, 479, 940),
            Point::new(352, 342, 300),
            Point::new(466, 668, 158),
            Point::new(542, 29, 236),
            Point::new(431, 825, 988),
            Point::new(739, 650, 466),
            Point::new(52, 470, 668),
            Point::new(216, 146, 977),
            Point::new(819, 987, 18),
            Point::new(117, 168, 530),
            Point::new(805, 96, 715),
            Point::new(346, 949, 466),
            Point::new(970, 615, 88),
            Point::new(941, 993, 340),
            Point::new(862, 61, 35),
            Point::new(984, 92, 344),
            Point::new(425, 690, 689),
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse(INPUT), Ok(("", parsed_input())));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input(), 10), 40);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 25272);
    }
}

#![feature(int_abs_diff)]

use advent_of_code_2022::{read, parse};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::IResult;
use nom::multi::many1;
use nom::sequence::terminated;
use std::collections::HashSet;

type Point = (usize, usize);

#[derive(Debug, PartialEq)]
struct Map {
    elevation: Vec<Vec<u8>>,
}

impl Map {
    pub fn height(&self) -> usize {
        self.elevation.len()
    }

    pub fn width(&self) -> usize {
        self.elevation[0].len()
    }

    pub fn get(&self, (i, j): Point) -> u8 {
        self.elevation[i][j]
    }

    pub fn points(&self) -> impl Iterator<Item = Point> {
        (0..self.height()).cartesian_product(0..self.width())
    }

    pub fn neighbors(&self, (i, j): Point) -> Vec<Point> {
        let mut points = Vec::new();

        if i > 0 {
            points.push((i - 1, j));
        }

        if i < self.height() - 1 {
            points.push((i + 1, j));
        }

        if j > 0 {
            points.push((i, j - 1));
        }

        if j < self.width() - 1 {
            points.push((i, j + 1));
        }

        points
    }

    pub fn destinations(&self, point: Point) -> impl Iterator<Item = Point> + '_ {
        let current = self.get(point);

        self.neighbors(point).into_iter().filter(move |neighbor| self.get(*neighbor) <= current + 1)
    }

    pub fn destinations_rev(&self, point: Point) -> impl Iterator<Item = Point> + '_ {
        let current = self.get(point);

        self.neighbors(point).into_iter().filter(move |neighbor| current <= self.get(*neighbor) + 1)
    }
}

#[derive(Debug, PartialEq)]
struct Input {
    map: Map,
    start: Point,
    end: Point,
}

fn parser(input: &str) -> IResult<&str, Input> {
    let (input, chars) = many1(
        terminated(
            alpha1,
            tag("\n"),
        ),
    )(input)?;

    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;

    let mut map = Map {
        elevation: Vec::new(),
    };

    for (i, row) in chars.into_iter().enumerate() {
        let mut elevation_row = Vec::new();

        for (j, c) in row.chars().enumerate() {
            let elevation = match c {
                'S' => {
                    start = Some((i, j));

                    0
                },
                'E' => {
                    end = Some((i, j));

                    25
                },
                c => c as u8 - 97,
            };

            elevation_row.push(elevation);
        }

        map.elevation.push(elevation_row);
    }

    Ok((input, Input {
        map,
        start: start.unwrap(),
        end: end.unwrap(),
    }))
}

fn solve_part1(input: &Input) -> u32 {
    let mut unvisited: HashSet<Point> = input.map.points().collect();
    let mut distances: Vec<Vec<Option<u32>>> = input.map.elevation.iter().map(|row| row.iter().map(|_| None).collect()).collect();

    distances[input.start.0][input.start.1] = Some(0);

    let mut current = input.start;

    loop {
        let new_distance = distances[current.0][current.1].unwrap() + 1;

        for point in input.map.destinations(current) {
            if !unvisited.contains(&point) {
                continue;
            }

            match distances[point.0][point.1] {
                None => {
                    distances[point.0][point.1] = Some(new_distance);
                },
                Some(distance) => {
                    if new_distance < distance {
                        distances[point.0][point.1] = Some(new_distance);
                    }
                },
            }
        }

        unvisited.remove(&current);

        if !unvisited.contains(&input.end) {
            return distances[input.end.0][input.end.1].unwrap();
        }

        current = *unvisited
            .iter()
            .filter_map(|point| match distances[point.0][point.1] {
                None => None,
                Some(distance) => Some((point, distance)),
            })
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(point, _)| point)
            .unwrap();
    }
}

fn solve_part2(input: &Input) -> u32 {
    let mut unvisited: HashSet<Point> = input.map.points().collect();
    let mut distances: Vec<Vec<Option<u32>>> = input.map.elevation.iter().map(|row| row.iter().map(|_| None).collect()).collect();

    distances[input.end.0][input.end.1] = Some(0);

    let mut current = input.end;

    loop {
        let new_distance = distances[current.0][current.1].unwrap() + 1;

        for point in input.map.destinations_rev(current) {
            if !unvisited.contains(&point) {
                continue;
            }

            match distances[point.0][point.1] {
                None => {
                    distances[point.0][point.1] = Some(new_distance);
                },
                Some(distance) => {
                    if new_distance < distance {
                        distances[point.0][point.1] = Some(new_distance);
                    }
                },
            }
        }

        unvisited.remove(&current);

        if let Some(point) = input.map.points().filter(|point| input.map.get(*point) == 0).filter(|point| !unvisited.contains(point)).next() {
            return distances[point.0][point.1].unwrap();
        }

        current = *unvisited
            .iter()
            .filter_map(|point| match distances[point.0][point.1] {
                None => None,
                Some(distance) => Some((point, distance)),
            })
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(point, _)| point)
            .unwrap();
    }
}

fn main() {
    let input = read(12);

    let parsed = parse(parser, &input);

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi\n";

    fn parsed_input() -> Input {
        Input {
            map: Map {
                elevation: vec![
                    vec![0, 0, 1, 16, 15, 14, 13, 12],
                    vec![0, 1, 2, 17, 24, 23, 23, 11],
                    vec![0, 2, 2, 18, 25, 25, 23, 10],
                    vec![0, 2, 2, 19, 20, 21, 22, 9],
                    vec![0, 1, 3, 4, 5, 6, 7, 8],
                ],
            },
            start: (0, 0),
            end: (2, 5),
        }
    }

    #[test]
    fn test_parser() {
        assert_eq!(parser(INPUT), Ok(("", parsed_input())));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 31);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 29);
    }
}

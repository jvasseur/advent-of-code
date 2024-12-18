use std::collections::HashSet;

use advent_of_code_2024::{dijkstra, geometry::{Point, Vector}, parser::*, read};
use nom::{bytes::complete::tag, character::complete::i64, multi::many1, sequence::{separated_pair, terminated}, IResult, Parser};

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    bits: Vec<Point>,
}

impl Input {
    fn new(bits: Vec<Point>) -> Self {
        Self { bits }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, bits) = many1(terminated(separated_pair(i64, tag(","), i64).map(|(x, y)| Point::new(x, y)), tag("\n")))(input)?;

        Ok((input, Input::new(bits)))
    }
}

const DIRECTIONS: [Vector; 4] = [
    Vector::new(1, 0),
    Vector::new(-1, 0),
    Vector::new(0, 1),
    Vector::new(0, -1),
];

fn solve(bits: &[Point], size: (i64, i64)) -> Option<u32> {
    let bits: HashSet<Point> = bits.iter().cloned().collect();
    let end = Point::new(size.0, size.1);

    dijkstra::shortest_path(
        [Point::new(0, 0)],
        |point| {
            let mut edges = Vec::new();

            for direction in DIRECTIONS {
                let next = point + direction;

                if next.x < 0 || next.x > size.0 {
                    continue;
                }

                if next.y < 0 || next.y > size.1 {
                    continue;
                }

                if bits.contains(&next) {
                    continue;
                }

                edges.push(dijkstra::Edge { node: next, cost: 1 });
            }

            edges
        },
        |point| point == &end,
    )
}

fn find_break(bits: &[Point], size: (i64, i64)) -> Point {
    for i in 0..bits.len() {
        if solve(&bits[0..=i], size).is_none() {
            return bits[i];
        }
    }

    panic!("Here be dragons");
}

fn solve_part1(input: &Input) -> u32 {
    solve(&input.bits[0..1024], (70, 70)).unwrap()
}

fn solve_part2(input: &Input) -> Point {
    find_break(&input.bits, (70, 70))
}

fn main() {
    let input = parse(&read(18).unwrap()).unwrap();

    let part1 = solve_part1(&input);
    println!("{}", part1);

    let part2 = solve_part2(&input);
    println!("{},{}", part2.x, part2.y);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

    fn parsed_input() -> Input {
        Input::new(vec![
            Point::new(5,4),
            Point::new(4,2),
            Point::new(4,5),
            Point::new(3,0),
            Point::new(2,1),
            Point::new(6,3),
            Point::new(2,4),
            Point::new(1,5),
            Point::new(0,6),
            Point::new(3,3),
            Point::new(2,6),
            Point::new(5,1),
            Point::new(1,2),
            Point::new(5,5),
            Point::new(2,5),
            Point::new(6,5),
            Point::new(1,4),
            Point::new(0,4),
            Point::new(6,4),
            Point::new(1,1),
            Point::new(6,1),
            Point::new(1,0),
            Point::new(0,5),
            Point::new(1,6),
            Point::new(2,0),
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse::<Input>(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(&parsed_input().bits[0..12], (6, 6)), Some(22));
    }

    #[test]
    fn test_find_break() {
        assert_eq!(find_break(&parsed_input().bits, (6, 6)), Point::new(6, 1));
    }
}

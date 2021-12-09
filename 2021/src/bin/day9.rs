use advent_of_code_2021::{parse_lines, read};
use itertools::Itertools;
use nom::bytes::complete::take;
use nom::character::complete::u8;
use nom::combinator::map_parser;
use nom::multi::many0;
use nom::IResult;
use std::collections::HashMap;
use std::collections::HashSet;

fn line_parser(input: &str) -> IResult<&str, Vec<u8>> {
    many0(map_parser(take(1_u8), u8))(input)
}

type Point = (usize, usize);

struct Map {
    points: Vec<Vec<u8>>,
    height: usize,
    width: usize,
    bassins: HashMap<Point, HashSet<Point>>,
}

impl Map {
    fn new(input: &[Vec<u8>]) -> Self {
        Self {
            points: input.to_owned(),
            height: input.len(),
            width: input[0].len(),
            bassins: HashMap::new(),
        }
    }

    fn get(&self, (i, j): Point) -> u8 {
        self.points[i][j]
    }

    fn neighbours(&self, (i, j): Point) -> Vec<Point> {
        let mut neighbours = Vec::new();

        if i != 0 {
            neighbours.push((i - 1, j))
        }

        if i != self.height - 1 {
            neighbours.push((i + 1, j))
        }

        if j != 0 {
            neighbours.push((i, j - 1))
        }

        if j != self.width - 1 {
            neighbours.push((i, j + 1))
        }

        neighbours
    }

    fn low_points(&self) -> Vec<Point> {
        (0..self.height).cartesian_product(0..self.width).filter(|&point| {
            let local = self.get(point);

            self.neighbours(point).iter().all(|&neighbour| self.get(neighbour) > local)
        }).collect()
    }

    fn compute_bassins(&mut self) {
        for point in self.low_points() {
            self.bassins.insert(point, HashSet::new());

            self.add_to_bassin(point, point);
        }
    }

    fn add_to_bassin(&mut self, bassin: (usize, usize), point: (usize, usize)) {
        let bassin_points = self.bassins.get_mut(&bassin).unwrap();

        if bassin_points.contains(&point) {
            return;
        }

        bassin_points.insert(point);

        for neighbour in self.neighbours(point) {
            if self.get(neighbour) != 9 {
                self.add_to_bassin(bassin, neighbour)
            }
        }
    }
}

fn solve_part1(input: &[Vec<u8>]) -> u32 {
    let map = Map::new(input);

    map.low_points().iter().map(|&low| (map.get(low) as u32) + 1).sum()
}

fn solve_part2(input: &[Vec<u8>]) -> usize {
    let mut map = Map::new(input);

    map.compute_bassins();

    map.bassins
        .into_values()
        .map(|bassin| bassin.len())
        .sorted()
        .rev()
        .take(3)
        .product()
}

fn main() {
    let input = read(9);

    let parsed_input = parse_lines(line_parser, &input);

    println!("{}", solve_part1(&parsed_input));
    println!("{}", solve_part2(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::line_parser;
    use super::solve_part1;
    use super::solve_part2;

    #[test]
    fn test_line_parser() {
        assert_eq!(
            line_parser("2199943210"),
            Ok(("", (vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0])))
        );
    }

    #[test]
    fn test_solve_part_1() {
        let input = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];

        assert_eq!(solve_part1(&input), 15);
    }

    #[test]
    fn test_solve_part_2() {
        let input = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];

        assert_eq!(solve_part2(&input), 1134);
    }
}

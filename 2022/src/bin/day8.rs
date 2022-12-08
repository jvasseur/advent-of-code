use advent_of_code_2022::{read, parse};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::combinator::{map, map_res};
use nom::IResult;
use nom::multi::many1;
use nom::sequence::terminated;

#[derive(Debug, PartialEq)]
struct Grid {
    trees: Vec<Vec<u8>>,
}

type Point = (usize, usize);

impl Grid {
    pub fn height(&self) -> usize {
        self.trees.len()
    }

    pub fn width(&self) -> usize {
        self.trees[0].len()
    }

    pub fn get(&self, (i, j): Point) -> u8 {
        self.trees[i][j]
    }

    pub fn points(&self) -> impl Iterator<Item = Point> {
        (0..self.height()).cartesian_product(0..self.width())
    }

    pub fn points_up(&self, (i, j): Point) -> impl Iterator<Item = Point> {
        if i == 0 {
            1..=0
        } else {
            0..=i-1
        }.rev().map(move |ibis| (ibis, j))
    }

    pub fn points_down(&self, (i, j): Point) -> impl Iterator<Item = Point> {
        (i+1..self.height()).map(move |ibis| (ibis, j))
    }

    pub fn points_left(&self, (i, j): Point) -> impl Iterator<Item = Point> {
        if j == 0 {
            1..=0
        } else {
            0..=j-1
        }.rev().map(move |jbis| (i, jbis))
    }

    pub fn points_right(&self, (i, j): Point) -> impl Iterator<Item = Point> {
        (j+1..self.width()).map(move |jbis| (i, jbis))
    }

    pub fn is_visible(&self, point: Point) -> bool {
        let height = self.get(point);
        let smaller = |point| self.get(point) < height;

        self.points_up(point).all(smaller)
            || self.points_down(point).all(smaller)
            || self.points_left(point).all(smaller)
            || self.points_right(point).all(smaller)
    }

    pub fn scenic_score(&self, point: Point) -> usize {
        let height = self.get(point);

        self.viewing_distance(self.points_up(point), height)
            * self.viewing_distance(self.points_down(point), height)
            * self.viewing_distance(self.points_left(point), height)
            * self.viewing_distance(self.points_right(point), height)
    }

    fn viewing_distance(&self, points: impl Iterator<Item = Point>, height: u8) -> usize {
        points.enumerate().find_or_last(|(_, point)| self.get(*point) >= height).map(|(d, _)| d + 1).unwrap_or(0)
    }
}

fn parser(input: &str) -> IResult<&str, Grid> {
    map(many1(
        terminated(
            many1(map_res(anychar, |c| c.to_digit(10).map(|u| u as u8).ok_or(()))),
            tag("\n"),
        ),
    ), |trees| Grid { trees })(input)
}

fn solve_part1(input: &Grid) -> usize {
    input.points().filter(|point| input.is_visible(*point)).count()
}

fn solve_part2(input: &Grid) -> usize {
    input.points().map(|point| input.scenic_score(point)).max().unwrap()
}

fn main() {
    let input = read(8);

    let parsed = parse(parser, &input);

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(parser("30373\n25512\n65332\n33549\n35390\n"), Ok(("", Grid {
            trees: vec![
                vec![3, 0, 3, 7, 3],
                vec![2, 5, 5, 1, 2],
                vec![6, 5, 3, 3, 2],
                vec![3, 3, 5, 4, 9],
                vec![3, 5, 3, 9, 0],
            ],
        })));
    }

    #[test]
    fn test_is_visible() {
        let grid = Grid {
            trees: vec![
                vec![3, 0, 3, 7, 3],
                vec![2, 5, 5, 1, 2],
                vec![6, 5, 3, 3, 2],
                vec![3, 3, 5, 4, 9],
                vec![3, 5, 3, 9, 0],
            ],
        };

        assert_eq!(grid.is_visible((0, 0)), true, "(0, 0)");
        assert_eq!(grid.is_visible((1, 1)), true, "(1, 1)");
        assert_eq!(grid.is_visible((2, 2)), false, "(2, 2)");
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&Grid {
            trees: vec![
                vec![3, 0, 3, 7, 3],
                vec![2, 5, 5, 1, 2],
                vec![6, 5, 3, 3, 2],
                vec![3, 3, 5, 4, 9],
                vec![3, 5, 3, 9, 0],
            ],
        }), 21);
    }

    #[test]
    fn test_scenic_score() {
        let grid = Grid {
            trees: vec![
                vec![3, 0, 3, 7, 3],
                vec![2, 5, 5, 1, 2],
                vec![6, 5, 3, 3, 2],
                vec![3, 3, 5, 4, 9],
                vec![3, 5, 3, 9, 0],
            ],
        };

        assert_eq!(grid.scenic_score((1, 2)), 4, "(1, 2)");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&Grid {
            trees: vec![
                vec![3, 0, 3, 7, 3],
                vec![2, 5, 5, 1, 2],
                vec![6, 5, 3, 3, 2],
                vec![3, 3, 5, 4, 9],
                vec![3, 5, 3, 9, 0],
            ],
        }), 8);
    }

}

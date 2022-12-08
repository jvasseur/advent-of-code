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

impl Grid {
    pub fn height(&self) -> usize {
        self.trees.len()
    }

    pub fn width(&self) -> usize {
        self.trees[0].len()
    }

    pub fn get(&self, i: usize, j: usize) -> u8 {
        self.trees[i][j]
    }

    pub fn is_visible(&self, i: usize, j: usize) -> bool {
        let height = self.get(i, j);

        (0..i).all(|ibis| self.get(ibis, j) < height)
            || (i+1..self.height()).all(|ibis| self.get(ibis, j) < height)
            || (0..j).all(|jbis| self.get(i, jbis) < height)
            || (j+1..self.width()).all(|jbis| self.get(i, jbis) < height)
    }

    pub fn viewing_distance(&self, i: usize, j: usize) -> usize {
        if i == 0 || j == 0 || i == self.height() - 1 || j == self.width() - 1 {
            return 0;
        }

        let height = self.get(i, j);

        (0..=i-1).rev().enumerate().find_or_last(|(_, ibis)| self.get(*ibis, j) >= height).map(|(d, _)| d + 1).unwrap_or(0)
            * (i+1..self.height()).enumerate().find_or_last(|(_, ibis)| self.get(*ibis, j) >= height).map(|(d, _)| d + 1).unwrap_or(0)
            * (0..=j-1).rev().enumerate().find_or_last(|(_, jbis)| self.get(i, *jbis) >= height).map(|(d, _)| d + 1).unwrap_or(0)
            * (j+1..self.width()).enumerate().find_or_last(|(_, jbis)| self.get(i, *jbis) >= height).map(|(d, _)| d + 1).unwrap_or(0)
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

fn solve_part1(input: &Grid) -> u32 {
    let mut count = 0;

    for i in 0..input.height() {
        for j in 0..input.width() {
            if input.is_visible(i, j) {
                count += 1;
            }
        }
    }

    count
}

fn solve_part2(input: &Grid) -> usize {
    let mut distance = 0;

    for i in 0..input.height() {
        for j in 0..input.width() {
            if input.viewing_distance(i, j) >= distance {
                distance = input.viewing_distance(i, j);
            }
        }
    }

    distance
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

        assert_eq!(grid.is_visible(0, 0), true, "(0, 0)");
        assert_eq!(grid.is_visible(1, 1), true, "(1, 1)");
        assert_eq!(grid.is_visible(2, 2), false, "(2, 2)");
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
    fn test_viewing_distance() {
        let grid = Grid {
            trees: vec![
                vec![3, 0, 3, 7, 3],
                vec![2, 5, 5, 1, 2],
                vec![6, 5, 3, 3, 2],
                vec![3, 3, 5, 4, 9],
                vec![3, 5, 3, 9, 0],
            ],
        };

        assert_eq!(grid.viewing_distance(1, 2), 4, "(1, 2)");
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

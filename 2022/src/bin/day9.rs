#![feature(int_abs_diff)]

use advent_of_code_2022::{read, parse};
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::u32;
use nom::combinator::{map, value};
use nom::IResult;
use nom::multi::many1;
use nom::sequence::{separated_pair, terminated};
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
struct Move {
    direction: Direction,
    steps: u32,
}

impl Move {
    pub fn new(direction: Direction, steps: u32) -> Self {
        Self {
            direction,
            steps,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn parser(input: &str) -> IResult<&str, Vec<Move>> {
    many1(
        terminated(
            map(separated_pair(alt((
                value(Direction::Left, tag("L")),
                value(Direction::Right, tag("R")),
                value(Direction::Up, tag("U")),
                value(Direction::Down, tag("D")),
            )), tag(" "), u32), |(direction, steps)| Move::new(direction, steps)),
            tag("\n"),
        ),
    )(input)
}

#[derive(Debug, PartialEq)]
struct Rope {
    head: (i32, i32),
    tail: (i32, i32),
}

impl Rope {
    pub fn new() -> Self {
        Rope {
            head: (0, 0),
            tail: (0, 0),
        }
    }

    pub fn move_head(&mut self, direction: Direction) {
        self.head = match direction {
            Direction::Left => (self.head.0 - 1, self.head.1),
            Direction::Right => (self.head.0 + 1, self.head.1),
            Direction::Up => (self.head.0, self.head.1 + 1),
            Direction::Down => (self.head.0, self.head.1 - 1),
        };

        if self.head.0.abs_diff(self.tail.0) <= 1 && self.head.1.abs_diff(self.tail.1) <= 1 {
            return;
        }

        if self.head.0 == self.tail.0 {
            if self.head.1 > self.tail.1 {
                self.tail = (self.tail.0, self.tail.1 + 1);
            } else {
                self.tail = (self.tail.0, self.tail.1 - 1);
            }

            return;
        }

        if self.head.1 == self.tail.1 {
            if self.head.0 > self.tail.0 {
                self.tail = (self.tail.0 + 1, self.tail.1);
            } else {
                self.tail = (self.tail.0 - 1, self.tail.1);
            }

            return;
        }

        if self.head.1 > self.tail.1 {
            self.tail = (self.tail.0, self.tail.1 + 1);
        } else {
            self.tail = (self.tail.0, self.tail.1 - 1);
        }

        if self.head.0 > self.tail.0 {
            self.tail = (self.tail.0 + 1, self.tail.1);
        } else {
            self.tail = (self.tail.0 - 1, self.tail.1);
        }
    }
}

#[derive(Debug, PartialEq)]
struct RopeV2 {
    knots: [(i32, i32);10],
}

impl RopeV2 {
    pub fn new() -> Self {
        RopeV2 {
            knots: [(0, 0); 10],
        }
    }

    pub fn move_head(&mut self, direction: Direction) {
        self.knots[0] = match direction {
            Direction::Left => (self.knots[0].0 - 1, self.knots[0].1),
            Direction::Right => (self.knots[0].0 + 1, self.knots[0].1),
            Direction::Up => (self.knots[0].0, self.knots[0].1 + 1),
            Direction::Down => (self.knots[0].0, self.knots[0].1 - 1),
        };

        for i in 1..10 {
            if self.knots[i - 1].0.abs_diff(self.knots[i].0) <= 1 && self.knots[i - 1].1.abs_diff(self.knots[i].1) <= 1 {
                continue;
            }

            if self.knots[i - 1].0 == self.knots[i].0 {
                if self.knots[i - 1].1 > self.knots[i].1 {
                    self.knots[i] = (self.knots[i].0, self.knots[i].1 + 1);
                } else {
                    self.knots[i] = (self.knots[i].0, self.knots[i].1 - 1);
                }

                continue;
            }

            if self.knots[i - 1].1 == self.knots[i].1 {
                if self.knots[i - 1].0 > self.knots[i].0 {
                    self.knots[i] = (self.knots[i].0 + 1, self.knots[i].1);
                } else {
                    self.knots[i] = (self.knots[i].0 - 1, self.knots[i].1);
                }

                continue;
            }

            if self.knots[i - 1].1 > self.knots[i].1 {
                self.knots[i] = (self.knots[i].0, self.knots[i].1 + 1);
            } else {
                self.knots[i] = (self.knots[i].0, self.knots[i].1 - 1);
            }

            if self.knots[i - 1].0 > self.knots[i].0 {
                self.knots[i] = (self.knots[i].0 + 1, self.knots[i].1);
            } else {
                self.knots[i] = (self.knots[i].0 - 1, self.knots[i].1);
            }
        }
    }
}


fn solve_part1(input: &[Move]) -> usize {
    let mut positions = HashSet::new();
    let mut rope = Rope::new();

    positions.insert(rope.tail);

    for mv in input {
        for _ in 0..mv.steps {
            rope.move_head(mv.direction);

            positions.insert(rope.tail);
        }
    }

    positions.len()
}

fn solve_part2(input: &[Move]) -> usize {
    let mut positions = HashSet::new();
    let mut rope = RopeV2::new();

    positions.insert(rope.knots[9]);

    for mv in input {
        for _ in 0..mv.steps {
            rope.move_head(mv.direction);

            positions.insert(rope.knots[9]);
        }
    }

    positions.len()
}

fn main() {
    let input = read(9);

    let parsed = parse(parser, &input);

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(parser("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n"), Ok(("", vec![
            Move::new(Direction::Right, 4),
            Move::new(Direction::Up, 4),
            Move::new(Direction::Left, 3),
            Move::new(Direction::Down, 1),
            Move::new(Direction::Right, 4),
            Move::new(Direction::Down, 1),
            Move::new(Direction::Left, 5),
            Move::new(Direction::Right, 2),
        ])));
    }

    #[test]
    fn test_move_head_right() {
        let mut rope = Rope::new();

        rope.move_head(Direction::Right);

        assert_eq!(rope.head, (1, 0));
        assert_eq!(rope.tail, (0, 0));

        rope.move_head(Direction::Right);

        assert_eq!(rope.head, (2, 0));
        assert_eq!(rope.tail, (1, 0));
    }

    #[test]
    fn test_move_head_up() {
        let mut rope = Rope::new();

        rope.move_head(Direction::Up);

        assert_eq!(rope.head, (0, 1));
        assert_eq!(rope.tail, (0, 0));

        rope.move_head(Direction::Up);

        assert_eq!(rope.head, (0, 2));
        assert_eq!(rope.tail, (0, 1));
    }

    #[test]
    fn test_move_head_down() {
        let mut rope = Rope::new();

        rope.move_head(Direction::Down);

        assert_eq!(rope.head, (0, -1));
        assert_eq!(rope.tail, (0, 0));

        rope.move_head(Direction::Down);

        assert_eq!(rope.head, (0, -2));
        assert_eq!(rope.tail, (0, -1));
    }

    #[test]
    fn test_move_head_diag() {
        let mut rope = Rope::new();

        rope.move_head(Direction::Right);
        rope.move_head(Direction::Up);

        assert_eq!(rope.head, (1, 1));
        assert_eq!(rope.tail, (0, 0));

        rope.move_head(Direction::Up);

        assert_eq!(rope.head, (1, 2));
        assert_eq!(rope.tail, (1, 1));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&vec![
            Move::new(Direction::Right, 4),
            Move::new(Direction::Up, 4),
            Move::new(Direction::Left, 3),
            Move::new(Direction::Down, 1),
            Move::new(Direction::Right, 4),
            Move::new(Direction::Down, 1),
            Move::new(Direction::Left, 5),
            Move::new(Direction::Right, 2),
        ]), 13);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&vec![
            Move::new(Direction::Right, 4),
            Move::new(Direction::Up, 4),
            Move::new(Direction::Left, 3),
            Move::new(Direction::Down, 1),
            Move::new(Direction::Right, 4),
            Move::new(Direction::Down, 1),
            Move::new(Direction::Left, 5),
            Move::new(Direction::Right, 2),
        ]), 1);
    }
}

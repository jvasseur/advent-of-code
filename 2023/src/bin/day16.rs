use std::collections::HashSet;
use advent_of_code_2023::{read, Parsable, util::{Grid, Point}};
use nom::{IResult, branch::alt, combinator::{map, value}, bytes::complete::tag, multi::many1, sequence::terminated};

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    grid: Grid<Element>,
}

impl Input {
    fn new(grid: impl Into<Grid<Element>>) -> Self {
        Self { grid: grid.into() }
    }

    fn energized_count(&self, start: (Point, Direction)) -> usize {
        let mut rays = vec![start];
        let mut energized = Grid::new_fill(self.grid.rows(), self.grid.cols(), false);
        let mut seen = HashSet::new();

        while rays.len() > 0 {
            let mut new_rays = vec![];

            for (point, direction) in rays {
                if seen.contains(&(point, direction)) {
                    continue;
                }

                seen.insert((point, direction));

                energized.set(&point, true);

                for new_direction in self.grid.get(&point).ray(direction) {
                    match new_direction {
                        Direction::Up => {
                            if point.row > 0 {
                                new_rays.push((point.up(1), new_direction));
                            }
                        },
                        Direction::Down => {
                            if point.row < self.grid.rows() - 1 {
                                new_rays.push((point.down(1), new_direction));
                            }
                        },
                        Direction::Left => {
                            if point.col > 0 {
                                new_rays.push((point.left(1), new_direction));
                            }
                        },
                        Direction::Right => {
                            if point.col < self.grid.cols() - 1 {
                                new_rays.push((point.right(1), new_direction));
                            }
                        },
                    }
                }
            }

            rays = new_rays;
        }

        let mut energized_count = 0;

        for row in 0..energized.rows() {
            for col in 0..energized.cols() {
                if *energized.get(&Point { row, col }) {
                    energized_count += 1;
                }
            }
        }

        energized_count
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        map(many1(terminated(many1(Element::parser), tag("\n"))), Input::new)(input)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Element {
    EmptySpace,
    MirrorRight,
    MirrorLeft,
    SplitterVertical,
    SplitterHorizontal,
}

impl Element {
    fn ray(&self, direction: Direction) -> Vec<Direction> {
        match self {
            Self::EmptySpace => vec![direction],
            Self::MirrorRight => vec![match direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            }],
            Self::MirrorLeft => vec![match direction {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            }],
            Self::SplitterVertical => match direction {
                Direction::Up | Direction::Down => vec![direction],
                Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
            }
            Self::SplitterHorizontal => match direction {
                Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
                Direction::Left | Direction::Right => vec![direction],
            },
        }
    }
}

impl Parsable for Element {
    fn parser(input: &str) -> IResult<&str, Self> {
        alt((
            value(Element::EmptySpace, tag(".")),
            value(Element::MirrorRight, tag("/")),
            value(Element::MirrorLeft, tag("\\")),
            value(Element::SplitterVertical, tag("|")),
            value(Element::SplitterHorizontal, tag("-")),
        ))(input)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn solve_part1(input: &Input) -> usize {
    input.energized_count((Point { row: 0, col: 0 }, Direction::Right))
}

fn solve_part2(input: &Input) -> usize {
    let mut max = 0;

    for col in 0..input.grid.cols() {
        max = std::cmp::max(max, input.energized_count((Point { row: 0, col }, Direction::Down)));
        max = std::cmp::max(max, input.energized_count((Point { row: input.grid.rows() - 1, col }, Direction::Up)));
    }

    for row in 0..input.grid.rows() {
        max = std::cmp::max(max, input.energized_count((Point { row, col: 0 }, Direction::Right)));
        max = std::cmp::max(max, input.energized_count((Point { row, col: input.grid.cols() - 1 }, Direction::Left)));
    }

    max
}

fn main() {
    let input = read(16);
    let parsed = Input::parse(&input).unwrap();

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Element::*;

    const INPUT: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....
";

    fn parsed_input() -> Input {
        Input::new(vec![
            vec![EmptySpace, SplitterVertical, EmptySpace, EmptySpace, EmptySpace, MirrorLeft, EmptySpace, EmptySpace, EmptySpace, EmptySpace],
            vec![SplitterVertical, EmptySpace, SplitterHorizontal, EmptySpace, MirrorLeft, EmptySpace, EmptySpace, EmptySpace, EmptySpace, EmptySpace],
            vec![EmptySpace, EmptySpace, EmptySpace, EmptySpace, EmptySpace, SplitterVertical, SplitterHorizontal, EmptySpace, EmptySpace, EmptySpace],
            vec![EmptySpace, EmptySpace, EmptySpace, EmptySpace, EmptySpace, EmptySpace, EmptySpace, EmptySpace, SplitterVertical, EmptySpace],
            vec![EmptySpace, EmptySpace, EmptySpace, EmptySpace, EmptySpace, EmptySpace, EmptySpace, EmptySpace, EmptySpace, EmptySpace],
            vec![EmptySpace, EmptySpace, EmptySpace, EmptySpace, EmptySpace, EmptySpace, EmptySpace, EmptySpace, EmptySpace, MirrorLeft],
            vec![EmptySpace, EmptySpace, EmptySpace, EmptySpace, MirrorRight, EmptySpace, MirrorLeft, MirrorLeft, EmptySpace, EmptySpace],
            vec![EmptySpace, SplitterHorizontal, EmptySpace, SplitterHorizontal, MirrorRight, EmptySpace, EmptySpace, SplitterVertical, EmptySpace, EmptySpace],
            vec![EmptySpace, SplitterVertical, EmptySpace, EmptySpace, EmptySpace, EmptySpace, SplitterHorizontal, SplitterVertical, EmptySpace, MirrorLeft],
            vec![EmptySpace, EmptySpace, MirrorRight, MirrorRight, EmptySpace, SplitterVertical, EmptySpace, EmptySpace, EmptySpace, EmptySpace],
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(Input::parse(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 46);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 51);
    }
}

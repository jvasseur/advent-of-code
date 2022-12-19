use advent_of_code_2022::{read, parse};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::IResult;
use nom::multi:: many0;
use nom::sequence::terminated;
use std::cmp::max;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

type Input = Vec<Direction>;

fn direction_parser(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Left, tag("<")),
        value(Direction::Right, tag(">")),
    ))(input)
}

fn parser(input: &str) -> IResult<&str, Input> {
    terminated(many0(direction_parser), tag("\n"))(input)
}

const ROCKS: [[[bool; 4]; 4]; 5] = [
    [
        // ....
        // ....
        // ....
        // #### y=0
        [true, false, false, false],
        [true, false, false, false],
        [true, false, false, false],
        [true, false, false, false],
    ],
    [
        // ....
        // .#..
        // ###.
        // .#.. y=0
        [false, true, false, false],
        [true, true, true, false],
        [false, true, false, false],
        [false, false, false, false],
    ],
    [
        // ....
        // ..#.
        // ..#.
        // ###. y=0
        [true, false, false, false],
        [true, false, false, false],
        [true, true, true, false],
        [false, false, false, false],
    ],
    [
        // #...
        // #...
        // #...
        // #... y=0
        [true, true, true, true],
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    [
        // ....
        // ....
        // ##..
        // ##..
        [true, true, false, false],
        [true, true, false, false],
        [false, false, false, false],
        [false, false, false, false],
    ],
];

const HEIGHTS: [usize; 5] = [
    1,
    3,
    3,
    4,
    2,
];

const WIDTHS: [usize; 5] = [
    4,
    3,
    3,
    1,
    2,
];

fn intersect<const HEIGHT: usize>(map: &[[bool; HEIGHT]; 7], rock: usize, x: usize, y: usize) -> bool {
    for rx in 0..WIDTHS[rock] {
        for ry in 0..HEIGHTS[rock] {
            if ROCKS[rock][rx][ry] && map[x + rx][y + ry] {
                return true;
            }
        }
    }

    return false
}

fn solve_part1(input: &Input) -> usize {
    let mut cycle: usize = 0;
    let mut rock: usize = 0;
    let mut height: usize = 0;

    let mut map = [[false; 6572]; 7];

    for _ in 0..2022 {
        let mut x: usize = 2;
        let mut y: usize = height + 3;

        loop {
            // Lateral movement
            let movable = match input[cycle] {
                Direction::Left => x > 0,
                Direction::Right => x + WIDTHS[rock] < 7,
            };

            if movable {
                let new_x = match input[cycle] {
                    Direction::Left => x - 1,
                    Direction::Right => x + 1,
                };

                if !intersect(&map, rock, new_x, y) {
                    x = new_x;
                }
            }

            cycle += 1;
            cycle %= input.len();

            // Down movement
            if y == 0 || intersect(&map, rock, x, y - 1) {
                for rx in 0..WIDTHS[rock] {
                    for ry in 0..HEIGHTS[rock] {
                        if ROCKS[rock][rx][ry] {
                            map[x + rx][y + ry] = true;
                        }
                    }
                }

                height = max(height, y + HEIGHTS[rock]);

                break;
            } else {
                y -= 1;
            }
        }

        rock += 1;
        rock %= ROCKS.len();
    }

    height
}

fn solve_part2(input: &Input) -> usize {
    0
}

fn main() {
    let input = read(17);

    let parsed = parse(parser, &input);

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>\n";

    fn parsed_input() -> Input {
        vec![
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Left,
            Direction::Left,
            Direction::Right,
            Direction::Left,
            Direction::Right,
            Direction::Right,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Right,
            Direction::Right,
            Direction::Left,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Right,
            Direction::Right,
            Direction::Right,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Right,
            Direction::Left,
            Direction::Left,
            Direction::Left,
            Direction::Right,
            Direction::Right,
            Direction::Left,
            Direction::Right,
            Direction::Right,
            Direction::Left,
            Direction::Left,
            Direction::Right,
            Direction::Right,

        ]
    }

    #[test]
    fn test_parser() {
        assert_eq!(parser(INPUT), Ok(("", parsed_input())));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 3068);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 1514285714288);
    }
}

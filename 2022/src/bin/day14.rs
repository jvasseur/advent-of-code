use advent_of_code_2022::{read, parse};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::u32;
use nom::combinator::map;
use nom::IResult;
use nom::multi::{separated_list0, many0};
use nom::sequence::{separated_pair, terminated};

type Point = (usize, usize);

type Rock = Vec<Point>;

type Input = Vec<Rock>;

fn point_parser(input: &str) -> IResult<&str, Point> {
    map(separated_pair(u32, tag(","), u32), |(x, y)| (x as usize, y as usize))(input)
}

fn rock_parser(input: &str) -> IResult<&str, Rock> {
    separated_list0(tag(" -> "), point_parser)(input)
}

fn parser(input: &str) -> IResult<&str, Vec<Rock>> {
    many0(terminated(rock_parser, tag("\n")))(input)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Space {
    Air,
    Rock,
    Sand,
}

struct Grid {
    x_min: usize,
    x_max: usize,
    y_min: usize,
    y_max: usize,

    spaces: Vec<Vec<Space>>,
}

impl Grid {
    pub fn new(x_min: usize, x_max: usize, y_min: usize, y_max: usize) -> Self {
        let mut col_prototype = Vec::new();
        col_prototype.resize(y_max - y_min + 1, Space::Air);

        let mut spaces = Vec::new();
        spaces.resize(x_max - x_min + 1, col_prototype);

        Grid {
            x_min,
            x_max,
            y_min,
            y_max,
            spaces,
        }
    }

    pub fn get(&self, (x, y): Point) -> Space {
        if x < self.x_min || x > self.x_max || y < self.y_min || y > self.y_max {
            Space::Air
        } else {
            self.spaces[x - self.x_min][y - self.y_min]
        }
    }

    pub fn set(&mut self, (x, y): Point, value: Space) {
        self.spaces[x - self.x_min][y - self.y_min] = value;
    }

    pub fn resize(&self, x_min: usize, x_max: usize, y_min: usize, y_max: usize) -> Self {
        let mut new_grid = Grid::new(x_min, x_max, y_min, y_max);

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                new_grid.set((x, y), self.get((x, y)));
            }
        }

        new_grid
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in self.y_min..=self.y_max {
            for x in self.x_min..=self.x_max {
                write!(f, "{}", match self.get((x, y)) {
                    Space::Air => '.',
                    Space::Sand => 'o',
                    Space::Rock => '#',
                })?
            }
            write!(f, "\n")?
        }

        Ok(())
    }
}

impl From<&Input> for Grid {
    fn from(input: &Input) -> Self {
        let (x_min, x_max) = input.iter().flatten().map(|(x, _)| x).minmax().into_option().unwrap();
        let y_min = 0;
        let y_max = input.iter().flatten().map(|(_, y)| y).max().unwrap();

        let mut grid = Grid::new(*x_min, *x_max, y_min, *y_max);

        for rock in input {
            for (start, end) in rock.iter().tuple_windows() {
                if start.0 == end.0 {
                    let range = if start.1 < end.1 {
                        start.1..=end.1
                    } else {
                        end.1..=start.1
                    };

                    for y in range {
                        grid.set((start.0, y), Space::Rock);
                    }
                }

                if start.1 == end.1 {
                    let range = if start.0 < end.0 {
                        start.0..=end.0
                    } else {
                        end.0..=start.0
                    };

                    for x in range {
                        grid.set((x, start.1), Space::Rock);
                    }
                }
            }
        }

        grid
    }
}

fn solve_part1(input: &Input) -> usize {
    let mut grid = Grid::from(input);
    let mut sand_count = 0;

    loop {
        let mut sand: Point = (500, 0);

        loop {
            if sand.0 < grid.x_min || sand.0 > grid.x_max || sand.1 < grid.y_min || sand.1 > grid.y_max {
                return sand_count;
            }

            if grid.get((sand.0, sand.1 + 1)) == Space::Air {
                sand = (sand.0, sand.1 + 1);

                continue;
            }

            if grid.get((sand.0 - 1, sand.1 + 1)) == Space::Air {
                sand = (sand.0 - 1, sand.1 + 1);

                continue;
            }

            if grid.get((sand.0 + 1, sand.1 + 1)) == Space::Air {
                sand = (sand.0 + 1, sand.1 + 1);

                continue;
            }

            grid.set(sand, Space::Sand);
            sand_count += 1;
            break;
        }
    }
}

fn solve_part2(input: &Input) -> usize {
    let mut grid = Grid::from(input);

    let y_max = grid.y_max + 2;

    grid = grid.resize(500 - y_max, 500 + y_max, 0, y_max);

    for x in grid.x_min..=grid.x_max {
        grid.set((x, y_max), Space::Rock);
    }

    let mut sand_count = 0;

    loop {
        let mut sand: Point = (500, 0);

        loop {
            if grid.get((sand.0, sand.1 + 1)) == Space::Air {
                sand = (sand.0, sand.1 + 1);

                continue;
            }

            if grid.get((sand.0 - 1, sand.1 + 1)) == Space::Air {
                sand = (sand.0 - 1, sand.1 + 1);

                continue;
            }

            if grid.get((sand.0 + 1, sand.1 + 1)) == Space::Air {
                sand = (sand.0 + 1, sand.1 + 1);

                continue;
            }

            grid.set(sand, Space::Sand);
            sand_count += 1;

            if sand == (500, 0) {
                return sand_count;
            }

            break;
        }
    }
}

fn main() {
    let input = read(14);

    let parsed = parse(parser, &input);

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9\n";

    fn parsed_input() -> Input {
        vec![
            vec![(498,4), (498,6), (496,6)],
            vec![(503,4), (502,4), (502,9), (494,9)],
        ]
    }

    #[test]
    fn test_parser() {
        assert_eq!(parser(INPUT), Ok(("", parsed_input())));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 24);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 93);
    }
}

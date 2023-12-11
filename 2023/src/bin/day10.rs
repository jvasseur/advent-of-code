use advent_of_code_2023::{read, Parsable};
use advent_of_code_2023::util::{Grid, Point};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::IResult;
use nom::combinator::value;
use nom::multi::many0;
use nom::sequence::terminated;
use nom::branch::alt;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    lines: Vec<Vec<Item>>,
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, lines) = many0(terminated(many0(Item::parser), tag("\n")))(input)?;

        Ok((input, Input { lines }))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Item {
    Pipe(Pipe),
    Ground,
    Start,
}

impl Parsable for Item {
    fn parser(input: &str) -> IResult<&str, Self> {
        alt((
            value(Item::Pipe(Pipe::NS), tag("|")),
            value(Item::Pipe(Pipe::EW), tag("-")),
            value(Item::Pipe(Pipe::NE), tag("L")),
            value(Item::Pipe(Pipe::NW), tag("J")),
            value(Item::Pipe(Pipe::SW), tag("7")),
            value(Item::Pipe(Pipe::SE), tag("F")),
            value(Item::Ground, tag(".")),
            value(Item::Start, tag("S")),
        ))(input)
    }

}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
}

impl Pipe {
    fn connections(&self, point: &Point) -> [Point; 2] {
        match self {
            Pipe::NS => [Point { row: point.row - 1, col: point.col }, Point { row: point.row + 1, col: point.col }],
            Pipe::EW => [Point { row: point.row, col: point.col + 1 }, Point { row: point.row, col: point.col - 1 }],
            Pipe::NE => [Point { row: point.row - 1, col: point.col }, Point { row: point.row, col: point.col + 1 }],
            Pipe::NW => [Point { row: point.row - 1, col: point.col }, Point { row: point.row, col: point.col - 1 }],
            Pipe::SW => [Point { row: point.row + 1, col: point.col }, Point { row: point.row, col: point.col - 1 }],
            Pipe::SE => [Point { row: point.row + 1, col: point.col }, Point { row: point.row, col: point.col + 1 }],
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Map {
    lines: Vec<Vec<Option<Pipe>>>,
    start: Point,
}

impl Map {
    fn get(&self, point: &Point) -> Option<Pipe> {
        self.lines[point.row][point.col]
    }

    fn connections(&self, point: &Point) -> Option<[Point; 2]> {
        self.get(point).map(|pipe| pipe.connections(point))
    }
}

impl From<&Input> for Map {
    fn from(value: &Input) -> Self {
        let mut start = None;

        for line in 0..value.lines.len() {
            for column in 0..value.lines[line].len() {
                if value.lines[line][column] == Item::Start {
                    start = Some(Point { row: line, col: column });
                }
            }
        }

        let start = start.unwrap();

        let north = match value.lines[start.row - 1][start.col] {
            Item::Pipe(pipe) => match pipe {
                Pipe::NS => true,
                Pipe::EW => false,
                Pipe::NE => false,
                Pipe::NW => false,
                Pipe::SW => true,
                Pipe::SE => true,

            }
            Item::Ground => false,
            Item::Start => panic!("Multiple start"),
        };

        let south = match value.lines[start.row + 1][start.col] {
            Item::Pipe(pipe) => match pipe {
                Pipe::NS => true,
                Pipe::EW => false,
                Pipe::NE => true,
                Pipe::NW => true,
                Pipe::SW => false,
                Pipe::SE => false,

            }
            Item::Ground => false,
            Item::Start => panic!("Multiple start"),
        };

        let west = match value.lines[start.row][start.col - 1] {
            Item::Pipe(pipe) => match pipe {
                Pipe::NS => false,
                Pipe::EW => true,
                Pipe::NE => true,
                Pipe::NW => false,
                Pipe::SW => false,
                Pipe::SE => true,

            }
            Item::Ground => false,
            Item::Start => panic!("Multiple start"),
        };

        let east = match value.lines[start.row][start.col + 1] {
            Item::Pipe(pipe) => match pipe {
                Pipe::NS => false,
                Pipe::EW => true,
                Pipe::NE => false,
                Pipe::NW => true,
                Pipe::SW => true,
                Pipe::SE => false,

            }
            Item::Ground => false,
            Item::Start => panic!("Multiple start"),
        };

        let start_pipe = match (north, south, west, east) {
            (true, true, false, false) => Pipe::NS,
            (false, false, true, true) => Pipe::EW,
            (true, false, false, true) => Pipe::NE,
            (true, false, true, false) => Pipe::NW,
            (false, true, true, false) => Pipe::SW,
            (false, true, false, true) => Pipe::SE,
            _ => panic!("Invalid start")
        };

        let lines = value.lines.iter().map(|line| line.iter().map(|item| match item {
            Item::Pipe(pipe) => Some(*pipe),
            Item::Ground => None,
            Item::Start => Some(start_pipe),
        }).collect()).collect();

        Map { lines, start }
    }
}

fn solve_part1(input: &Input) -> u32 {
    let map = Map::from(input);

    let [connection, _] = map.connections(&map.start).unwrap();

    let mut previous = map.start;
    let mut current = connection;
    let mut steps = 1;

    while current != map.start {
        let connections = map.connections(&current).unwrap();

        let next = connections.into_iter().filter(|point| point != &previous).exactly_one().unwrap();

        (previous, current) = (current, next);
        steps += 1;
    }

    steps / 2
}

fn solve_part2(input: &Input) -> u32 {
    let map = Map::from(input);

    let mut points = Vec::new();

    let [connection, _] = map.connections(&map.start).unwrap();

    let mut previous = map.start;
    let mut current = connection;
    points.push(map.start);

    while current != map.start {
        points.push(current);
        let connections = map.connections(&current).unwrap();

        let next = connections.into_iter().filter(|point| point != &previous).exactly_one().unwrap();

        (previous, current) = (current, next);
    }

    let mut enclosure = Grid::new_fill(map.lines.len() * 3, map.lines[0].len() * 3, false);

    for point in points {
        let pipe = map.get(&point).unwrap();

        let center = Point {
            row: point.row * 3 + 1,
            col: point.col * 3 + 1,
        };

        enclosure.set(&center, true);

        match pipe {
            Pipe::NS => {
                enclosure.set(&center.up(1), true);
                enclosure.set(&center.down(1), true);
            },
            Pipe::EW => {
                enclosure.set(&center.right(1), true);
                enclosure.set(&center.left(1), true);
            },
            Pipe::NE => {
                enclosure.set(&center.up(1), true);
                enclosure.set(&center.right(1), true);
            },
            Pipe::NW => {
                enclosure.set(&center.up(1), true);
                enclosure.set(&center.left(1), true);
            },
            Pipe::SW => {
                enclosure.set(&center.down(1), true);
                enclosure.set(&center.left(1), true);
            },
            Pipe::SE => {
                enclosure.set(&center.down(1), true);
                enclosure.set(&center.right(1), true);
            },
        }
    }

    let mut outside = Grid::new_fill(map.lines.len() * 3, map.lines[0].len() * 3, false);

    for col in 0..outside.cols() {
        outside.set(&Point { row: 0, col }, true);
        outside.set(&Point { row: outside.rows() - 1, col }, true);
    }

    for row in 0..outside.rows() {
        outside.set(&Point { row, col: 0 }, true);
        outside.set(&Point { row, col: outside.cols() - 1 }, true);
    }

    let mut changed = true;
    while changed {
        changed = false;

        for row in 0..outside.rows() {
            for col in 0..outside.cols() {
                let point = Point { row, col };

                if !outside.get(&point) {
                    continue;
                }

                let mut points = Vec::new();

                if point.row > 0 {
                    points.push(point.up(1));
                }

                if point.row < outside.rows() - 1 {
                    points.push(point.down(1));
                }

                if point.col > 0 {
                    points.push(point.left(1));
                }

                if point.col < outside.cols() - 1 {
                    points.push(point.right(1));
                }

                for point in points {
                    if !enclosure.get(&point) && !outside.get(&point) {
                        outside.set(&point, true);
                        changed = true;
                    }
                }
            }
        }
    }

    let mut count = 0;

    for row in 0..map.lines.len() {
        for col in 0..map.lines[0].len() {
            let center = Point {
                row: row * 3 + 1,
                col: col * 3 + 1,
            };

            if !enclosure.get(&center) && !outside.get(&center) {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let input = read(10);

    let parsed = Input::parse(&input).unwrap();

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ".....
.S-7.
.|.|.
.L-J.
.....
";

    fn parsed_input() -> Input {
        Input {
            lines: vec![
                vec![Item::Ground, Item::Ground, Item::Ground, Item::Ground, Item::Ground],
                vec![Item::Ground, Item::Start, Item::Pipe(Pipe::EW), Item::Pipe(Pipe::SW), Item::Ground],
                vec![Item::Ground, Item::Pipe(Pipe::NS), Item::Ground, Item::Pipe(Pipe::NS), Item::Ground],
                vec![Item::Ground, Item::Pipe(Pipe::NE), Item::Pipe(Pipe::EW), Item::Pipe(Pipe::NW), Item::Ground],
                vec![Item::Ground, Item::Ground, Item::Ground, Item::Ground, Item::Ground],
            ],
        }
    }

    #[test]
    fn test_parser() {
        assert_eq!(Input::parse(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 4);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 1);
    }
}

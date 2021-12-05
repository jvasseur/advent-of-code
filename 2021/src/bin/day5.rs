use advent_of_code_2021::{read, parse_lines};
use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::u16;
use nom::sequence::tuple;

#[derive(Clone,Debug,Eq,PartialEq)]
struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn new(x: u16, y: u16) -> Self {
        Point { x, y }
    }
}

#[derive(Clone,Debug,Eq,PartialEq)]
struct Line {
    point1: Point,
    point2: Point,
}

impl Line {
    fn new(point1: Point, point2: Point) -> Self {
        Line { point1, point2 }
    }

    fn is_vertical(&self) -> bool {
        self.point1.x == self.point2.x
    }

    fn is_horizontal(&self) -> bool {
        self.point1.y == self.point2.y
    }

    fn points(&self) -> Vec<Point> {
        let xsign: i32 = if self.point2.x > self.point1.x {
            1
        } else if self.point2.x < self.point1.x {
            -1
        } else {
            0
        };

        let ysign: i32 = if self.point2.y > self.point1.y {
            1
        } else if self.point2.y < self.point1.y {
            -1
        } else {
            0
        };

        let steps = if xsign != 0 {
            let x1: i32 = self.point1.x.into();
            let x2: i32 = self.point2.x.into();

            (x1 - x2).abs()
        } else {
            let y1: i32 = self.point1.y.into();
            let y2: i32 = self.point2.y.into();

            (y1 - y2).abs()
        };

        Vec::from_iter((0..=steps).map(|i| {
            let xbase: i32 = self.point1.x.into();
            let ybase: i32 = self.point1.y.into();

            let x = xbase + xsign * i;
            let y = ybase + ysign * i;

            Point::new(x.try_into().unwrap(), y.try_into().unwrap())
        }))
    }
}

fn line_parser(input: &str) -> IResult<&str, Line> {
    let (rest, (x1, _, y1, _, x2, _, y2)) = tuple((u16, tag(","), u16, tag(" -> "), u16, tag(","), u16))(input)?;

    Ok((rest, Line::new(Point::new(x1, y1), Point::new(x2, y2))))
}

fn solve_part1(input: &[Line]) -> u32 {
    let mut grid = [[0_u8; 1000]; 1000];

    for line in input {
        if line.is_horizontal() || line.is_vertical() {
            for point in line.points() {
                let x: usize = point.x.into();
                let y: usize = point.y.into();

                grid[x][y] += 1;
            }
        }
    }

    let mut twos = 0;

    for x in 0..1000 {
        for y in 0..1000 {
            if grid[x][y] >= 2 {
                twos += 1;
            }
        }
    }

    twos
}

fn solve_part2(input: &[Line]) -> u32 {
    let mut grid = [[0_u8; 1000]; 1000];

    for line in input {
        for point in line.points() {
            let x: usize = point.x.into();
            let y: usize = point.y.into();

            grid[x][y] += 1;
        }
    }

    let mut twos = 0;

    for x in 0..1000 {
        for y in 0..1000 {
            if grid[x][y] >= 2 {
                twos += 1;
            }
        }
    }

    twos
}

fn main() {
    let input = read(5);

    let parsed_input = parse_lines(line_parser, &input);

    println!("{}", solve_part1(&parsed_input));
    println!("{}", solve_part2(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::line_parser;
    use super::Line;
    use super::Point;
    use super::solve_part1;
    use super::solve_part2;

    #[test]
    fn test_line_parser() {
        assert_eq!(line_parser("1,1 -> 1,3"), Ok(("", Line { point1: Point {x: 1, y: 1 }, point2: Point { x: 1, y: 3 } })));
    }

    #[test]
    fn test_points_1() {
        let line = Line {
            point1: Point { x: 1, y: 1 },
            point2: Point { x: 1, y: 3 },
        };

        assert_eq!(line.points(), vec![
            Point::new(1, 1),
            Point::new(1, 2),
            Point::new(1, 3),
        ]);
    }

    #[test]
    fn test_points_2() {
        let line = Line {
            point1: Point::new(9, 7),
            point2: Point::new(7, 7),
        };

        assert_eq!(line.points(), vec![
            Point::new(9, 7),
            Point::new(8, 7),
            Point::new(7, 7),
        ]);
    }

    #[test]
    fn test_points_3() {
        let line = Line::new(Point::new(0, 9), Point::new(5, 9));

        assert_eq!(line.points(), vec![
            Point::new(0, 9),
            Point::new(1, 9),
            Point::new(2, 9),
            Point::new(3, 9),
            Point::new(4, 9),
            Point::new(5, 9),
        ]);
    }

    #[test]
    fn test_solve_part_1() {
        let input = vec![
            Line::new(Point::new(0, 9), Point::new(5, 9)),
            Line::new(Point::new(8, 0), Point::new(0, 8)),
            Line::new(Point::new(9, 4), Point::new(3, 4)),
            Line::new(Point::new(2, 2), Point::new(2, 1)),
            Line::new(Point::new(7, 0), Point::new(7, 4)),
            Line::new(Point::new(6, 4), Point::new(2, 0)),
            Line::new(Point::new(0, 9), Point::new(2, 9)),
            Line::new(Point::new(3, 4), Point::new(1, 4)),
            Line::new(Point::new(0, 0), Point::new(8, 8)),
            Line::new(Point::new(5, 5), Point::new(8, 2)),
        ];

        assert_eq!(solve_part1(&input), 5);
    }

    #[test]
    fn test_solve_part_2() {
        let input = vec![
            Line::new(Point::new(0, 9), Point::new(5, 9)),
            Line::new(Point::new(8, 0), Point::new(0, 8)),
            Line::new(Point::new(9, 4), Point::new(3, 4)),
            Line::new(Point::new(2, 2), Point::new(2, 1)),
            Line::new(Point::new(7, 0), Point::new(7, 4)),
            Line::new(Point::new(6, 4), Point::new(2, 0)),
            Line::new(Point::new(0, 9), Point::new(2, 9)),
            Line::new(Point::new(3, 4), Point::new(1, 4)),
            Line::new(Point::new(0, 0), Point::new(8, 8)),
            Line::new(Point::new(5, 5), Point::new(8, 2)),
        ];

        assert_eq!(solve_part2(&input), 12);
    }
}

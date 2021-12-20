use advent_of_code_2021::{parse, read};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::value;
use nom::multi::many0;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::IResult;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Point {
    Dark,
    Light,
}

fn to_num(points: &[Point]) -> usize {
    points
        .into_iter()
        .rev()
        .enumerate()
        .filter(|(_, point)| point == &&Point::Light)
        .map(|(i, _)| 2_usize.pow(i as u32))
        .sum()
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Point::Dark => '.',
                Point::Light => '#',
            }
        )
    }
}

fn point_parser(input: &str) -> IResult<&str, Point> {
    alt((value(Point::Dark, tag(".")), value(Point::Light, tag("#"))))(input)
}

fn image_parser(input: &str) -> IResult<&str, Vec<Vec<Point>>> {
    many0(terminated(many0(point_parser), newline))(input)
}

fn enhancement_parser(input: &str) -> IResult<&str, Vec<Point>> {
    terminated(many0(point_parser), newline)(input)
}

fn input_parser(input: &str) -> IResult<&str, (Vec<Point>, Vec<Vec<Point>>)> {
    separated_pair(enhancement_parser, newline, image_parser)(input)
}

struct Map {
    height: usize,
    width: usize,
    points: Vec<Vec<Point>>,
    default: Point,
}

impl Map {
    fn new(height: usize, width: usize) -> Self {
        Self {
            height,
            width,
            points: vec![vec![Point::Dark; width]; height],
            default: Point::Dark,
        }
    }

    fn get(&self, i: i32, j: i32) -> Point {
        if i < 0 || i >= self.height as i32 || j < 0 || j >= self.width as i32 {
            self.default.to_owned()
        } else {
            self.points[i as usize][j as usize].to_owned()
        }
    }

    fn enhance(&self, enhancement: &Vec<Point>) -> Self {
        let mut new_map = Map::new(self.height + 2, self.width + 2);

        new_map.default = match self.default {
            Point::Dark => enhancement[0].to_owned(),
            Point::Light => enhancement[511].to_owned(),
        };

        for i in 0..new_map.height {
            for j in 0..new_map.width {
                let is = i as i32;
                let js = j as i32;

                let index = to_num(&[
                    self.get(is - 2, js - 2),
                    self.get(is - 2, js - 1),
                    self.get(is - 2, js),
                    self.get(is - 1, js - 2),
                    self.get(is - 1, js - 1),
                    self.get(is - 1, js),
                    self.get(is, js - 2),
                    self.get(is, js - 1),
                    self.get(is, js),
                ]);

                new_map.points[i][j] = enhancement[index].to_owned();
            }
        }

        new_map
    }
}

impl From<&Vec<Vec<Point>>> for Map {
    fn from(points: &Vec<Vec<Point>>) -> Self {
        Self {
            height: points.len(),
            width: points[0].len(),
            points: points.to_owned(),
            default: Point::Dark,
        }
    }
}

fn solve_part_1(input: &(Vec<Point>, Vec<Vec<Point>>)) -> usize {
    let mut map = Map::from(&input.1);

    map = map.enhance(&input.0);
    map = map.enhance(&input.0);

    map.points
        .into_iter()
        .flatten()
        .filter(|point| point == &Point::Light)
        .count()
}

fn solve_part_2(input: &(Vec<Point>, Vec<Vec<Point>>)) -> usize {
    let mut map = Map::from(&input.1);

    for _ in 0..50 {
        map = map.enhance(&input.0);
    }

    map.points
        .into_iter()
        .flatten()
        .filter(|point| point == &Point::Light)
        .count()
}

fn main() {
    let input = read(20);

    let parsed_input = parse(input_parser, &input);

    println!("{}", solve_part_1(&parsed_input));
    println!("{}", solve_part_2(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::input_parser;
    use super::solve_part_1;
    use super::solve_part_2;
    use super::to_num;
    use super::Point;
    use nom::Finish;

    static TEXT: &str = "\
        ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##\
        #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###\
        .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.\
        .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....\
        .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..\
        ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....\
        ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#\n\
        \n\
        #..#.\n\
        #....\n\
        ##..#\n\
        ..#..\n\
        ..###\n\
";

    #[test]
    fn test_input_parser() {
        let (input, result) = input_parser(TEXT).finish().unwrap();

        assert_eq!(input, "");

        assert_eq!(result.0.len(), 512);
        assert_eq!(
            result.1,
            vec![
                vec![
                    Point::Light,
                    Point::Dark,
                    Point::Dark,
                    Point::Light,
                    Point::Dark
                ],
                vec![
                    Point::Light,
                    Point::Dark,
                    Point::Dark,
                    Point::Dark,
                    Point::Dark
                ],
                vec![
                    Point::Light,
                    Point::Light,
                    Point::Dark,
                    Point::Dark,
                    Point::Light
                ],
                vec![
                    Point::Dark,
                    Point::Dark,
                    Point::Light,
                    Point::Dark,
                    Point::Dark
                ],
                vec![
                    Point::Dark,
                    Point::Dark,
                    Point::Light,
                    Point::Light,
                    Point::Light
                ],
            ]
        );
    }

    #[test]
    fn test_to_num() {
        assert_eq!(
            to_num(&[
                Point::Dark,
                Point::Dark,
                Point::Dark,
                Point::Light,
                Point::Dark,
                Point::Dark,
                Point::Dark,
                Point::Light,
                Point::Dark,
            ]),
            34
        );
    }

    #[test]
    fn test_solve_part_1() {
        let (_, result) = input_parser(TEXT).finish().unwrap();

        assert_eq!(solve_part_1(&result), 35);
    }

    #[test]
    fn test_solve_part_2() {
        let (_, result) = input_parser(TEXT).finish().unwrap();

        assert_eq!(solve_part_2(&result), 3351);
    }
}

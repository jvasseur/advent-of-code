use advent_of_code_2021::{parse_lines, read};
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::branch::alt;
use nom::combinator::value;
use nom::IResult;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Cucumber {
    East,
    South,
}

fn line_parser(input: &str) -> IResult<&str, Vec<Option<Cucumber>>> {
    many0(alt((
        value(None, tag(".")),
        value(Some(Cucumber::East), tag(">")),
        value(Some(Cucumber::South), tag("v")),
    )))(input)
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Map {
    height: usize,
    width: usize,
    points: Vec<Vec<Option<Cucumber>>>,
}

impl From<&Vec<Vec<Option<Cucumber>>>> for Map {
    fn from(points: &Vec<Vec<Option<Cucumber>>>) -> Self {
        Self {
            height: points.len(),
            width: points[0].len(),
            points: points.to_owned(),
        }
    }
}

fn solve_part_1(input: &Vec<Vec<Option<Cucumber>>>) -> u32 {
    let mut map = Map::from(input);
    let mut run = 0;

    loop {
        let mut moved = false;

        // Move east
        let mut new = map.clone();
        for i in 0..map.height {
            for j in 0..map.width {
                if map.points[i][j] != Some(Cucumber::East) {
                    continue;
                }

                let next = (j + 1).rem_euclid(map.width);

                if map.points[i][next] == None {
                    new.points[i][j] = None;
                    new.points[i][next] = Some(Cucumber::East);
                    moved = true;
                }
            }
        }

        map = new;

        // Move south
        let mut new = map.clone();
        for i in 0..map.height {
            for j in 0..map.width {
                if map.points[i][j] != Some(Cucumber::South) {
                    continue;
                }

                let next = (i + 1).rem_euclid(map.height);

                if map.points[next][j] == None {
                    new.points[i][j] = None;
                    new.points[next][j] = Some(Cucumber::South);
                    moved = true;
                }
            }
        }

        map = new;

        run += 1;

        if !moved {
            return run;
        }
    }
}

fn main() {
    let input = read(25);

    let parsed_input = parse_lines(line_parser, &input);

    println!("Part 1: {}", solve_part_1(&parsed_input));
}

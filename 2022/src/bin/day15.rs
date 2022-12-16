use advent_of_code_2022::{read, parse};
use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::IResult;
use nom::multi::many0;
use nom::sequence::{preceded, terminated};
use std::ops::RangeInclusive;

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point {
            x,
            y,
        }
    }

    pub fn distance(&self, other: &Point) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Sensor {
    sensor: Point,
    beacon: Point,
}

impl Sensor {
    pub fn new(sensor: Point, beacon: Point) -> Self {
        Sensor {
            sensor,
            beacon,
        }
    }

    pub fn covers_row(&self, y: i32) -> Option<RangeInclusive<i32>> {
        let distance = self.sensor.distance(&self.beacon);
        let delta = self.sensor.y.abs_diff(y);

        if delta > distance {
            None
        } else {
            let length = (distance - delta) as i32;

            Some(self.sensor.x - length..=self.sensor.x + length)
        }
    }
}

type Input = Vec<Sensor>;

fn point_parser(input: &str) -> IResult<&str, Point> {
    let (input, x) = preceded(tag("x="), i32)(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, y) = preceded(tag("y="), i32)(input)?;

    Ok((input, Point::new(x, y)))
}

fn sensor_parser(input: &str) -> IResult<&str, Sensor> {
    let (input, _) = tag("Sensor at ")(input)?;
    let (input, sensor) = point_parser(input)?;
    let (input, _) = tag(": closest beacon is at ")(input)?;
    let (input, beacon) = point_parser(input)?;

    Ok((input, Sensor::new(sensor, beacon)))
}

fn parser(input: &str) -> IResult<&str, Input> {
    many0(terminated(sensor_parser, tag("\n")))(input)
}

fn solve_part1(input: &Input, y: i32) -> usize {
    let ranges: Vec<RangeInclusive<i32>> = input.iter().filter_map(|sensor| sensor.covers_row(y)).collect();

    let min = *ranges.iter().map(|range| range.start()).min().unwrap();
    let max = *ranges.iter().map(|range| range.end()).max().unwrap();

    (min..=max)
        .filter(|x| input.iter().all(|sensor| sensor.beacon != Point::new(*x, y)))
        .filter(|x| ranges.iter().any(|range| range.contains(x)))
        .count()
}

fn solve_part2(input: &Input, max: i32) -> u64 {
    for y in 0..=max {
        let ranges: Vec<RangeInclusive<i32>> = input.iter().filter_map(|sensor| sensor.covers_row(y)).collect();

        let mut x = 0;

        'outer: loop {
            if x > max {
                break;
            }

            for range in &ranges {
                if range.contains(&x) {
                    x = range.end() + 1;

                    continue 'outer;
                }
            }

            return (x as u64) * 4000000 + (y as u64);
        }
    }

    panic!("Here be dragons");
}

fn main() {
    let input = read(15);

    let parsed = parse(parser, &input);

    println!("{}", solve_part1(&parsed, 2000000));
    println!("{}", solve_part2(&parsed, 4000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    fn parsed_input() -> Input {
        vec![
            Sensor::new(Point::new(2, 18), Point::new(-2, 15)),
            Sensor::new(Point::new(9, 16), Point::new(10, 16)),
            Sensor::new(Point::new(13, 2), Point::new(15, 3)),
            Sensor::new(Point::new(12, 14), Point::new(10, 16)),
            Sensor::new(Point::new(10, 20), Point::new(10, 16)),
            Sensor::new(Point::new(14, 17), Point::new(10, 16)),
            Sensor::new(Point::new(8, 7), Point::new(2, 10)),
            Sensor::new(Point::new(2, 0), Point::new(2, 10)),
            Sensor::new(Point::new(0, 11), Point::new(2, 10)),
            Sensor::new(Point::new(20, 14), Point::new(25, 17)),
            Sensor::new(Point::new(17, 20), Point::new(21, 22)),
            Sensor::new(Point::new(16, 7), Point::new(15, 3)),
            Sensor::new(Point::new(14, 3), Point::new(15, 3)),
            Sensor::new(Point::new(20, 1), Point::new(15, 3)),
        ]
    }

    #[test]
    fn test_parser() {
        assert_eq!(parser(INPUT), Ok(("", parsed_input())));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input(), 10), 26);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input(), 20), 56000011);
    }
}

use advent_of_code_2022::{read, parse};
use nom::bytes::complete::tag;
use nom::character::complete::i8;
use nom::IResult;
use nom::multi:: many0;
use nom::sequence::terminated;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i8,
    y: i8,
    z: i8
}

impl Point {
    pub fn new(x: i8, y: i8, z: i8) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    pub fn neighbours(&self) -> [Point; 6] {
        [
            Point::new(self.x - 1, self.y, self.z),
            Point::new(self.x + 1, self.y, self.z),
            Point::new(self.x, self.y - 1, self.z),
            Point::new(self.x, self.y + 1, self.z),
            Point::new(self.x, self.y, self.z - 1),
            Point::new(self.x, self.y, self.z + 1),
        ]
    }
}

type Input = Vec<Point>;

fn point_parser(input: &str) -> IResult<&str, Point> {
    let (input, x) = i8(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = i8(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, z) = i8(input)?;

    Ok((input, Point::new(x, y, z)))
}

fn parser(input: &str) -> IResult<&str, Input> {
    many0(terminated(point_parser, tag("\n")))(input)
}

fn solve_part1(input: &Input) -> u16 {
    let points: HashSet<Point> = HashSet::from_iter(input.iter().cloned());

    let mut surface = 0;

    for point in &points {
        for neighbour in point.neighbours() {
            if !points.contains(&neighbour) {
                surface += 1;
            }
        }
    }

    surface
}

fn solve_part2(input: &Input) -> u16 {
    let points: HashSet<Point> = HashSet::from_iter(input.iter().cloned());

    let mut x_min = i8::MAX;
    let mut x_max = i8::MIN;
    let mut y_min = i8::MAX;
    let mut y_max = i8::MIN;
    let mut z_min = i8::MAX;
    let mut z_max = i8::MIN;

    for point in &points {
        if point.x < x_min {
            x_min = point.x;
        }
        if point.x > x_max {
            x_max = point.x;
        }
        if point.y < y_min {
            y_min = point.y;
        }
        if point.y > y_max {
            y_max = point.y;
        }
        if point.z < z_min {
            z_min = point.z;
        }
        if point.z > z_max {
            z_max = point.z;
        }
    }

    let x_range = x_min - 1..=x_max + 1;
    let y_range = y_min - 1..=y_max + 1;
    let z_range = z_min - 1..=z_max + 1;

    let mut exterior: HashSet<Point> = HashSet::new();

    exterior.insert(Point::new(*x_range.start(), *y_range.start(),*z_range.start()));

    loop {
        let mut changed = false;

        for exterior_point in exterior.clone() {
            for neighbourg in exterior_point.neighbours() {
                if x_range.contains(&neighbourg.x) && y_range.contains(&neighbourg.y) && z_range.contains(&neighbourg.z) {
                    if !points.contains(&neighbourg) && !exterior.contains(&neighbourg) {
                        exterior.insert(neighbourg);

                        changed = true;
                    }
                }
            }
        }

        if !changed {
            break;
        }
    }

    let mut surface = 0;

    for point in &points {
        for neighbour in point.neighbours() {
            if exterior.contains(&neighbour) {
                surface += 1;
            }
        }
    }

    surface

}

fn main() {
    let input = read(18);

    let parsed = parse(parser, &input);

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

    fn parsed_input() -> Input {
        vec![
            Point::new(2, 2, 2),
            Point::new(1, 2, 2),
            Point::new(3, 2, 2),
            Point::new(2, 1, 2),
            Point::new(2, 3, 2),
            Point::new(2, 2, 1),
            Point::new(2, 2, 3),
            Point::new(2, 2, 4),
            Point::new(2, 2, 6),
            Point::new(1, 2, 5),
            Point::new(3, 2, 5),
            Point::new(2, 1, 5),
            Point::new(2, 3, 5),
        ]
    }

    #[test]
    fn test_parser() {
        assert_eq!(parser(INPUT), Ok(("", parsed_input())));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 64);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 58);
    }
}

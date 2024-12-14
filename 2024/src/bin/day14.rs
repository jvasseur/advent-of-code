use std::{thread, time::Duration};
use advent_of_code_2024::{geometry::{Point, Vector}, parser::*, read};
use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete::i64, IResult};

#[derive(Clone, PartialEq, Eq, Debug)]
struct Robot {
    position: Point,
    velocity: Vector,
}

impl Robot {
    fn new(position: Point, velocity: Vector) -> Self {
        Self { position, velocity }
    }

    fn get_position(&self, cycles: i64, space: (i64, i64)) -> Point {
        Point::new(
            (self.position.x + self.velocity.dx * cycles).rem_euclid(space.0),
            (self.position.y + self.velocity.dy * cycles).rem_euclid(space.1),
        )
    }
}

impl Parsable for Robot {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("p=")(input)?;
        let (input, px) = i64(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, py) = i64(input)?;
        let (input, _) = tag(" v=")(input)?;
        let (input, vx) = i64(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, vy) = i64(input)?;

        Ok((input, Robot::new(Point::new(px, py), Vector::new(vx, vy))))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    robots: Vec<Robot>,
}

impl Input {
    fn new(robots: impl Into<Vec<Robot>>) -> Self {
        Self { robots: robots.into() }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, robots) = lines_parser(input)?;

        Ok((input, Input::new(robots)))
    }
}

fn solve_part1(input: &Input, space: (i64, i64)) -> usize {
    let mut quadrants = [0, 0, 0, 0];

    let x_mid = (space.0 - 1) / 2;
    let y_mid = (space.1 - 1) / 2;

    for robot in &input.robots {
        let position = robot.get_position(100, space);

        if position.x < x_mid {
            if position.y < y_mid {
                quadrants[0] += 1;
            }
            if position.y > y_mid {
                quadrants[1] += 1;
            }
        }
        if position.x > x_mid {
            if position.y < y_mid {
                quadrants[2] += 1;
            }
            if position.y > y_mid {
                quadrants[3] += 1;
            }
        }
    }

    quadrants.iter().product()
}

fn solve_part2(input: &Input) {
    'cycle: for cycle in 1.. {

        let mut grid = [[0; 101]; 103];

        for robot in &input.robots {
            let position = robot.get_position(cycle, (101, 103));

            if grid[position.y as usize][position.x as usize] == 0 {
                grid[position.y as usize][position.x as usize] += 1;
            } else {
                continue 'cycle;
            }
        }

        println!("{:?}", cycle);
        println!("{}", grid.iter().map(|row| row.iter().map(|a| match a { 1 => 'X', _ => ' ' }).join("")).join("\n"));

        thread::sleep(Duration::from_millis(1000));
    }
}

fn main() {
    let input = parse(&read(14).unwrap()).unwrap();

    println!("{}", solve_part1(&input, (101, 103)));

    // Let the human do it !
    solve_part2(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    fn parsed_input() -> Input {
        Input::new([
            Robot::new(Point::new(0, 4), Vector::new(3, -3)),
            Robot::new(Point::new(6, 3), Vector::new(-1, -3)),
            Robot::new(Point::new(10, 3), Vector::new(-1, 2)),
            Robot::new(Point::new(2, 0), Vector::new(2, -1)),
            Robot::new(Point::new(0, 0), Vector::new(1, 3)),
            Robot::new(Point::new(3, 0), Vector::new(-2, -2)),
            Robot::new(Point::new(7, 6), Vector::new(-1, -3)),
            Robot::new(Point::new(3, 0), Vector::new(-1, -2)),
            Robot::new(Point::new(9, 3), Vector::new(2, 3)),
            Robot::new(Point::new(7, 3), Vector::new(-1, 2)),
            Robot::new(Point::new(2, 4), Vector::new(2, -3)),
            Robot::new(Point::new(9, 5), Vector::new(-3, -3)),
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse::<Input>(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_get_position() {
        assert_eq!(Robot::new(Point::new(2, 4), Vector::new(2, -3)).get_position(5, (11, 7)), Point::new(1, 3));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input(), (11, 7)), 12);
    }
}

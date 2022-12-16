use advent_of_code_2022::{read, parse};
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::combinator::map;
use nom::IResult;
use nom::multi::many1;
use nom::sequence::{preceded, terminated};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn parser(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(
        terminated(
            alt((
                map(tag("noop"), |_| Instruction::Noop),
                map(preceded(tag("addx "), i32), |v| Instruction::Addx(v)),
            )),
            tag("\n"),
        ),
    )(input)
}

fn solve_part1(input: &[Instruction]) -> i32 {
    let mut registry = 1;
    let mut values = Vec::from([1]);

    for instruction in input {
        match instruction {
            Instruction::Noop => {
                values.push(registry);
            },
            Instruction::Addx(amount) => {
                values.push(registry);
                values.push(registry);

                registry += amount;
            },
        }
    }

    20 * values[20] + 60 * values[60] + 100 * values[100] + 140 * values[140] + 180 * values[180] + 220 * values[220]
}

struct Screen {
    registry: i32,
    screen: [[bool; 40]; 6],
    cycle: usize,
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            registry: 1,
            screen: [[false; 40]; 6],
            cycle: 0,
        }
    }

    pub fn draw(&mut self) {
        let cycle_vertical = self.cycle.div_euclid(40);
        let cycle_horizontal = self.cycle.rem_euclid(40);

        if (cycle_horizontal as i32).abs_diff(self.registry) <= 1 {
            self.screen[cycle_vertical][cycle_horizontal] = true;
        }

        self.cycle += 1;
    }
}

impl ToString for Screen {
    fn to_string(&self) -> String {
        self.screen.iter().map(|row| row.iter().map(|v| if *v { '#' } else { '.' }).collect::<String>()).join("\n")
    }
}

fn solve_part2(input: &[Instruction]) -> String {
    let mut screen = Screen::new();

    for instruction in input {
        match instruction {
            Instruction::Noop => {
                screen.draw();
            },
            Instruction::Addx(amount) => {
                screen.draw();
                screen.draw();

                screen.registry += amount;
            },
        }
    }

    screen.to_string()
}

fn main() {
    let input = read(10);

    let parsed = parse(parser, &input);

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(parser("noop\naddx 3\naddx -5\n"), Ok(("", vec![
            Instruction::Noop,
            Instruction::Addx(3),
            Instruction::Addx(-5),
        ])));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&vec![
            Instruction::Addx(15),
            Instruction::Addx(-11),
            Instruction::Addx(6),
            Instruction::Addx(-3),
            Instruction::Addx(5),
            Instruction::Addx(-1),
            Instruction::Addx(-8),
            Instruction::Addx(13),
            Instruction::Addx(4),
            Instruction::Noop,
            Instruction::Addx(-1),
            Instruction::Addx(5),
            Instruction::Addx(-1),
            Instruction::Addx(5),
            Instruction::Addx(-1),
            Instruction::Addx(5),
            Instruction::Addx(-1),
            Instruction::Addx(5),
            Instruction::Addx(-1),
            Instruction::Addx(-35),
            Instruction::Addx(1),
            Instruction::Addx(24),
            Instruction::Addx(-19),
            Instruction::Addx(1),
            Instruction::Addx(16),
            Instruction::Addx(-11),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(21),
            Instruction::Addx(-15),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(-3),
            Instruction::Addx(9),
            Instruction::Addx(1),
            Instruction::Addx(-3),
            Instruction::Addx(8),
            Instruction::Addx(1),
            Instruction::Addx(5),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(-36),
            Instruction::Noop,
            Instruction::Addx(1),
            Instruction::Addx(7),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(2),
            Instruction::Addx(6),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(1),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(7),
            Instruction::Addx(1),
            Instruction::Noop,
            Instruction::Addx(-13),
            Instruction::Addx(13),
            Instruction::Addx(7),
            Instruction::Noop,
            Instruction::Addx(1),
            Instruction::Addx(-33),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(2),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(8),
            Instruction::Noop,
            Instruction::Addx(-1),
            Instruction::Addx(2),
            Instruction::Addx(1),
            Instruction::Noop,
            Instruction::Addx(17),
            Instruction::Addx(-9),
            Instruction::Addx(1),
            Instruction::Addx(1),
            Instruction::Addx(-3),
            Instruction::Addx(11),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(1),
            Instruction::Noop,
            Instruction::Addx(1),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(-13),
            Instruction::Addx(-19),
            Instruction::Addx(1),
            Instruction::Addx(3),
            Instruction::Addx(26),
            Instruction::Addx(-30),
            Instruction::Addx(12),
            Instruction::Addx(-1),
            Instruction::Addx(3),
            Instruction::Addx(1),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(-9),
            Instruction::Addx(18),
            Instruction::Addx(1),
            Instruction::Addx(2),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(9),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(-1),
            Instruction::Addx(2),
            Instruction::Addx(-37),
            Instruction::Addx(1),
            Instruction::Addx(3),
            Instruction::Noop,
            Instruction::Addx(15),
            Instruction::Addx(-21),
            Instruction::Addx(22),
            Instruction::Addx(-6),
            Instruction::Addx(1),
            Instruction::Noop,
            Instruction::Addx(2),
            Instruction::Addx(1),
            Instruction::Noop,
            Instruction::Addx(-10),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(20),
            Instruction::Addx(1),
            Instruction::Addx(2),
            Instruction::Addx(2),
            Instruction::Addx(-6),
            Instruction::Addx(-11),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
        ]), 13140);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&vec![
            Instruction::Addx(15),
            Instruction::Addx(-11),
            Instruction::Addx(6),
            Instruction::Addx(-3),
            Instruction::Addx(5),
            Instruction::Addx(-1),
            Instruction::Addx(-8),
            Instruction::Addx(13),
            Instruction::Addx(4),
            Instruction::Noop,
            Instruction::Addx(-1),
            Instruction::Addx(5),
            Instruction::Addx(-1),
            Instruction::Addx(5),
            Instruction::Addx(-1),
            Instruction::Addx(5),
            Instruction::Addx(-1),
            Instruction::Addx(5),
            Instruction::Addx(-1),
            Instruction::Addx(-35),
            Instruction::Addx(1),
            Instruction::Addx(24),
            Instruction::Addx(-19),
            Instruction::Addx(1),
            Instruction::Addx(16),
            Instruction::Addx(-11),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(21),
            Instruction::Addx(-15),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(-3),
            Instruction::Addx(9),
            Instruction::Addx(1),
            Instruction::Addx(-3),
            Instruction::Addx(8),
            Instruction::Addx(1),
            Instruction::Addx(5),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(-36),
            Instruction::Noop,
            Instruction::Addx(1),
            Instruction::Addx(7),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(2),
            Instruction::Addx(6),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(1),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(7),
            Instruction::Addx(1),
            Instruction::Noop,
            Instruction::Addx(-13),
            Instruction::Addx(13),
            Instruction::Addx(7),
            Instruction::Noop,
            Instruction::Addx(1),
            Instruction::Addx(-33),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(2),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(8),
            Instruction::Noop,
            Instruction::Addx(-1),
            Instruction::Addx(2),
            Instruction::Addx(1),
            Instruction::Noop,
            Instruction::Addx(17),
            Instruction::Addx(-9),
            Instruction::Addx(1),
            Instruction::Addx(1),
            Instruction::Addx(-3),
            Instruction::Addx(11),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(1),
            Instruction::Noop,
            Instruction::Addx(1),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(-13),
            Instruction::Addx(-19),
            Instruction::Addx(1),
            Instruction::Addx(3),
            Instruction::Addx(26),
            Instruction::Addx(-30),
            Instruction::Addx(12),
            Instruction::Addx(-1),
            Instruction::Addx(3),
            Instruction::Addx(1),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(-9),
            Instruction::Addx(18),
            Instruction::Addx(1),
            Instruction::Addx(2),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(9),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(-1),
            Instruction::Addx(2),
            Instruction::Addx(-37),
            Instruction::Addx(1),
            Instruction::Addx(3),
            Instruction::Noop,
            Instruction::Addx(15),
            Instruction::Addx(-21),
            Instruction::Addx(22),
            Instruction::Addx(-6),
            Instruction::Addx(1),
            Instruction::Noop,
            Instruction::Addx(2),
            Instruction::Addx(1),
            Instruction::Noop,
            Instruction::Addx(-10),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(20),
            Instruction::Addx(1),
            Instruction::Addx(2),
            Instruction::Addx(2),
            Instruction::Addx(-6),
            Instruction::Addx(-11),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
        ]), "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....");
    }
}

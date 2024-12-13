use advent_of_code_2024::{geometry::{Point, Vector}, parser::*, read};
use nom::{bytes::complete::tag, character::complete::i64, multi::separated_list1, IResult};

#[derive(Clone, PartialEq, Eq, Debug)]
struct Machine {
    a: Vector,
    b: Vector,
    prize: Point,
}

impl Machine {
    fn new(a: Vector, b: Vector, prize: Point) -> Self {
        Self { a, b, prize }
    }

    fn convert(&self) -> Self {
        Machine::new(
            self.a.clone(),
            self.b.clone(),
            Point::new(self.prize.x + 10000000000000, self.prize.y + 10000000000000),
        )
    }

    fn get_solution(&self) -> Option<(i64, i64)> {
        /*
         * adx * na + bdx * nb = px
         * ady * na + bdy * nb = py
         *
         * adx * bdy * na + bdx * bdy * nb = px * bdy
         * ady * bdx * na + bdx * bdy * nb = py * bdx
         *
         * adx * bdy * na + bdx * bdy * nb - ady * bdx * na + bdx * bdy * nb = px * bdy - py * bdx
         *
         * na * (adx * bdy - ady * bdx) = px * bdy - py * bdx
         * na = (px * bdy - py * bdx) / (adx * bdy - ady * bdx)
         *
         * nb = (px - adx * na) / bdx
         */

        let numerator = self.b.dx * self.prize.y - self.b.dy * self.prize.x;
        let denominator = self.b.dx * self.a.dy - self.b.dy * self.a.dx;

        if numerator % denominator == 0 {
            let na = numerator / denominator;

            let b_numerator = self.prize.x - self.a.dx * na;
            let b_denominator = self.b.dx;

            if b_numerator % b_denominator == 0 {
                let nb = (self.prize.x - self.a.dx * na) / self.b.dx;

                Some((na, nb))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Parsable for Machine {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("Button A: X+")(input)?;
        let (input, adx) = i64(input)?;
        let (input, _) = tag(", Y+")(input)?;
        let (input, ady) = i64(input)?;
        let (input, _) = tag("\nButton B: X+")(input)?;
        let (input, bdx) = i64(input)?;
        let (input, _) = tag(", Y+")(input)?;
        let (input, bdy) = i64(input)?;
        let (input, _) = tag("\nPrize: X=")(input)?;
        let (input, px) = i64(input)?;
        let (input, _) = tag(", Y=")(input)?;
        let (input, py) = i64(input)?;
        let (input, _) = tag("\n")(input)?;

        Ok((input, Machine::new(Vector::new(adx, ady), Vector::new(bdx, bdy), Point::new(px, py))))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    machines: Vec<Machine>
}

impl Input {
    fn new(machines: impl Into<Vec<Machine>>) -> Self {
        Self { machines: machines.into() }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, machines) = separated_list1(tag("\n"), Machine::parser)(input)?;

        Ok((input, Input::new(machines)))
    }
}

fn solve_part1(input: &Input) -> i64 {
    input.machines.iter().map(|machine| {
        if let Some((na, nb)) = machine.get_solution() {
            na * 3 + nb
        } else {
            0
        }
    }).sum()
}

fn solve_part2(input: &Input) -> i64 {
    input.machines.iter().map(|machine| {
        if let Some((na, nb)) = machine.convert().get_solution() {
            na * 3 + nb
        } else {
            0
        }
    }).sum()
}

fn main() {
    let input = parse(&read(13).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    fn parsed_input() -> Input {
        Input::new([
            Machine::new(Vector::new(94, 34), Vector::new(22, 67), Point::new(8400, 5400)),
            Machine::new(Vector::new(26, 66), Vector::new(67, 21), Point::new(12748, 12176)),
            Machine::new(Vector::new(17, 86), Vector::new(84, 37), Point::new(7870, 6450)),
            Machine::new(Vector::new(69, 23), Vector::new(27, 71), Point::new(18641, 10279)),
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse::<Input>(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_convert() {
        let machines = parsed_input().machines;

        assert_eq!(machines[0].convert(), Machine::new(Vector::new(94, 34), Vector::new(22, 67), Point::new(10000000008400, 10000000005400)));
    }

    #[test]
    fn test_get_solution() {
        let machines = parsed_input().machines;

        assert_eq!(machines[0].get_solution(), Some((80, 40)));
        assert_eq!(machines[1].get_solution(), None);
        assert_eq!(machines[2].get_solution(), Some((38, 86)));
        assert_eq!(machines[3].get_solution(), None);
    }

    #[test]
    fn test_converted_get_solution() {
        let machines = parsed_input().machines;

        assert!(machines[0].convert().get_solution().is_none());
        assert!(machines[1].convert().get_solution().is_some());
        assert!(machines[2].convert().get_solution().is_none());
        assert!(machines[3].convert().get_solution().is_some());
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 480);
    }
}

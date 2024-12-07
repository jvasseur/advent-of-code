use advent_of_code_2024::{parser::*, read};
use itertools::Itertools;
use std::iter::repeat_n;
use nom::{bytes::complete::tag, multi::separated_list1, IResult};

type Value = u64;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Equation {
    test_value: Value,
    available_values: Vec<Value>
}

impl Equation {
    fn new(test_value: Value, available_values: impl Into<Vec<Value>>) -> Self {
        Self { test_value, available_values: available_values.into() }
    }
}

impl Parsable for Equation {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, test_value) = Value::parser(input)?;
        let (input, _) = tag(": ")(input)?;
        let (input, available_values) = separated_list1(tag(" "), Value::parser)(input)?;

        Ok((input, Equation::new(test_value, available_values)))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    equations: Vec<Equation>
}

impl Input {
    fn new(equations: impl Into<Vec<Equation>>) -> Self {
        Self { equations: equations.into() }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, equations) = lines_parser(input)?;

        Ok((input, Input::new(equations)))
    }
}

enum Operation {
    Add,
    Mul,
    Con,
}

fn sum_valid_equations(equations: &[Equation], operations: &[Operation]) -> u64 {
    equations.iter().filter(|equation| repeat_n(operations.iter(), equation.available_values.len() - 1).multi_cartesian_product().any(|operations| {
        let mut value = equation.available_values[0];

        for (index, operation) in operations.iter().enumerate() {
            match operation {
                Operation::Add => value += equation.available_values[index + 1],
                Operation::Mul => value *= equation.available_values[index + 1],
                Operation::Con => value = (format!("{}{}", value, equation.available_values[index + 1])).parse::<u64>().unwrap(),
            }

            if value > equation.test_value {
                return false;
            }
        }

        return value == equation.test_value;
    })).map(|equation| equation.test_value).sum()
}

fn solve_part1(input: &Input) -> u64 {
    sum_valid_equations(&input.equations, &[Operation::Add, Operation::Mul])
}

fn solve_part2(input: &Input) -> u64 {
    sum_valid_equations(&input.equations, &[Operation::Add, Operation::Mul, Operation::Con])
}

fn main() {
    let input = parse(&read(7).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    fn parsed_input() -> Input {
        Input::new([
            Equation::new(190, [10, 19]),
            Equation::new(3267, [81, 40, 27]),
            Equation::new(83, [17, 5]),
            Equation::new(156, [15, 6]),
            Equation::new(7290, [6, 8, 6, 15]),
            Equation::new(161011, [16, 10, 13]),
            Equation::new(192, [17, 8, 14]),
            Equation::new(21037, [9, 7, 18, 13]),
            Equation::new(292, [11, 6, 16, 20]),
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse::<Input>(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 3749);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 11387);
    }
}

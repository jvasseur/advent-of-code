use advent_of_code_2025::{grid::{Grid, Point}, parser::*, read};
use derive_more::IntoIterator;
use nom::{IResult, bytes::complete::{is_not, tag}, multi::many1, sequence::terminated};

fn digits_to_number(digits: impl DoubleEndedIterator<Item=u8>) -> u64 {
    digits
        .rev()
        .enumerate()
        .map(|(index, digit)| 10_u64.pow(index as u32) * (digit as u64))
        .sum::<u64>()
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Mul,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Problem {
    digits: Grid<Option<u8>>,
    operator: Operator,
}

impl Problem {
    fn new(digits: impl Into<Grid<Option<u8>>>, operator: impl Into<Operator>) -> Self {
        Self {
            digits: digits.into(),
            operator: operator.into(),
        }
    }

    fn solve_vertical(&self) -> u64 {
        let numbers = (0..self.digits.rows())
            .map(|row| digits_to_number((0..self.digits.cols())
                .filter_map(|col| *self.digits.get(&Point { row: row as i32, col: col as i32 }))
            ));

        match self.operator {
            Operator::Add => numbers.sum(),
            Operator::Mul => numbers.product(),
        }
    }

    fn solve_horizontal(&self) -> u64 {
        let numbers = (0..self.digits.cols())
            .map(|col| digits_to_number((0..self.digits.rows())
                .filter_map(|row| *self.digits.get(&Point { row: row as i32, col: col as i32 }))
            ));

        match self.operator {
            Operator::Add => numbers.sum(),
            Operator::Mul => numbers.product(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, IntoIterator)]
#[into_iterator(owned, ref, ref_mut)]
struct Input (Vec<Problem>);

impl Input {
    fn new(problems: impl Into<Vec<Problem>>) -> Self {
        Self(problems.into())
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, mut lines) = many1(terminated(is_not("\n"), tag("\n")))(input)?;

        let operator_line = lines.pop().unwrap();
        let digits_lines = lines;

        let mut problems = Vec::new();

        let mut digits = vec![Vec::new(); digits_lines.len()];
        let mut operator = None;

        for i in 0..operator_line.len() {
            let operator_char = operator_line.chars().nth(i).unwrap();
            let digit_chars = digits_lines.iter().map(|line|  line.chars().nth(i).unwrap()).collect::<Vec<_>>();

            if digit_chars.iter().all(|&digit_char| digit_char == ' ') && operator_char == ' ' {
                problems.push(Problem::new(digits, operator.unwrap()));

                digits = vec![Vec::new(); digits_lines.len()];
                operator = None;

                continue;
            }

            match operator_char {
                '+' => {
                    operator = Some(Operator::Add);
                },
                '*' => {
                    operator = Some(Operator::Mul);
                },
                _ => {}
            }

            for (line, digit_char) in digit_chars.iter().enumerate() {
                digits[line].push(match digit_char {
                    ' ' => None,
                    char => Some(char.to_string().parse().unwrap()),
                })
            }
        }

        problems.push(Problem::new(digits, operator.unwrap()));

        Ok((input, Input::new(problems)))
    }
}

fn solve_part1(input: &Input) -> u64 {
    input
        .into_iter()
        .map(|problem| problem.solve_vertical())
        .sum()
}

fn solve_part2(input: &Input) -> u64 {
    input
        .into_iter()
        .map(|problem| problem.solve_horizontal())
        .sum()
}

fn main() {
    let input = from_str(&read(6).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  \n";

    fn parsed_input() -> Input {
        Input::new([
            Problem::new(vec![
                vec![Some(1), Some(2), Some(3)],
                vec![None, Some(4), Some(5)],
                vec![None, None, Some(6)],
            ], Operator::Mul),
            Problem::new(vec![
                vec![Some(3), Some(2), Some(8)],
                vec![Some(6), Some(4), None],
                vec![Some(9), Some(8), None],
            ], Operator::Add),
            Problem::new(vec![
                vec![None, Some(5), Some(1)],
                vec![Some(3), Some(8), Some(7)],
                vec![Some(2), Some(1), Some(5)],
            ], Operator::Mul),
            Problem::new(vec![
                vec![Some(6), Some(4), None],
                vec![Some(2), Some(3), None],
                vec![Some(3), Some(1), Some(4)],
            ], Operator::Add),
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse(INPUT), Ok(("", parsed_input())));
    }

    #[test]
    fn test_solve_vertical() {
        assert_eq!(parsed_input().0[0].solve_vertical(), 33210);
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 4277556);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 3263827);
    }
}

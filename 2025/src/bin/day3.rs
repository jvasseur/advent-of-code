use advent_of_code_2025::{parser::*, read};
use derive_more::IntoIterator;
use nom::{IResult, character::complete::{anychar, u8}, combinator::{map, map_parser, recognize}, multi::many1};

#[derive(Clone, Debug, PartialEq, Eq, IntoIterator)]
struct Bank(Vec<u8>);

impl Bank {
    fn new(batteries: impl Into<Vec<u8>>) -> Self {
        Self(batteries.into())
    }

    fn highest_joltage(&self, count: usize) -> u64 {
        let mut start = 0;
        let mut digits = Vec::new();

        for i in 0..count {
            let end = self.0.len() - count + i;

            let (position, digit) = self.0[start..=end].iter().enumerate().rev().max_by_key(|(_, &value)| value).unwrap();

            start += position + 1;
            digits.push(digit);
        }

        digits.into_iter()
            .rev()
            .enumerate()
            .map(|(i, digit)| (*digit as u64) * 10_u64.pow(i as u32))
            .sum()
    }
}

impl Parsable for Bank {
    fn parser(input: &str) -> IResult<&str, Self> {
        map(
            many1(
                map_parser(
                    recognize(anychar),
                    u8,
                ),
            ),
            Bank::new,
        )(input)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, IntoIterator)]
#[into_iterator(owned, ref, ref_mut)]
struct Input(Vec<Bank>);

impl Input {
    fn new(banks: impl Into<Vec<Bank>>) -> Self {
        Self(banks.into())
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        map(
        parse_lines,
            Input::new,
        )(input)
    }
}

fn solve_part1(input: &Input) -> u64 {
    input.into_iter().map(|bank| bank.highest_joltage(2)).sum()
}

fn solve_part2(input: &Input) -> u64 {
    input.into_iter().map(|bank| bank.highest_joltage(12)).sum()
}

fn main() {
    let input = from_str(&read(3).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111
";

    fn parsed_input() -> Input {
        Input::new([
            Bank::new([9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]),
            Bank::new([8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]),
            Bank::new([2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]),
            Bank::new([8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]),
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse(INPUT), Ok(("", parsed_input())));
    }

    #[test]
    fn test_highest_joltage_2() {
        assert_eq!(parsed_input().0[0].highest_joltage(2), 98);
        assert_eq!(parsed_input().0[1].highest_joltage(2), 89);
        assert_eq!(parsed_input().0[2].highest_joltage(2), 78);
        assert_eq!(parsed_input().0[3].highest_joltage(2), 92);
    }

    #[test]
    fn test_highest_joltage_12() {
        assert_eq!(parsed_input().0[0].highest_joltage(12), 987654321111);
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 357);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 3121910778619);
    }
}

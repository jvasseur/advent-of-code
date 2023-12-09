use advent_of_code_2023::{read, Parsable};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::IResult;
use nom::multi::{many0, separated_list0};
use nom::sequence::terminated;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    values: Vec<History>,
}

impl Input {
    fn new(values: impl Into<Vec<History>>) -> Self {
        Self { values: values.into() }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, values) = many0(terminated(History::parser, tag("\n")))(input)?;

        Ok((input, Input::new(values)))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct History {
    history: Vec<i32>,
}

impl History {
    fn new(history: impl Into<Vec<i32>>) -> Self {
        Self { history: history.into() }
    }

    fn predict_next(&self) -> i32 {
        let mut history = self.history.clone();
        let mut lasts = Vec::new();

        while !history.iter().all(|&value| value == 0) {
            lasts.push(*history.last().unwrap());
            history = history.iter().tuple_windows().map(|(a, b)| b - a).collect()
        }

        let mut value = 0;
        for last in lasts.iter().rev() {
            value = last + value;
        }

        value
    }

    fn predict_previous(&self) -> i32 {
        let mut history = self.history.clone();
        let mut firsts = Vec::new();

        while !history.iter().all(|&value| value == 0) {
            firsts.push(*history.first().unwrap());
            history = history.iter().tuple_windows().map(|(a, b)| b - a).collect()
        }

        let mut value = 0;
        for first in firsts.iter().rev() {
            value = first - value;
        }

        value
    }
}

impl Parsable for History {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, history) = separated_list0(tag(" "), i32)(input)?;

        Ok((input, History::new(history)))
    }
}

fn solve_part1(input: &Input) -> i32 {
    input.values.iter().map(History::predict_next).sum()
}

fn solve_part2(input: &Input) -> i32 {
    input.values.iter().map(History::predict_previous).sum()
}

fn main() {
    let input = read(9);

    let parsed = Input::parse(&input).unwrap();

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    fn parsed_input() -> Input {
        Input::new([
            History::new([0, 3, 6, 9, 12, 15]),
            History::new([1, 3, 6, 10, 15, 21]),
            History::new([10, 13, 16, 21, 30, 45]),
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(Input::parse(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_predict_next() {
        assert_eq!(History::new([0, 3, 6, 9, 12, 15]).predict_next(), 18);
        assert_eq!(History::new([1, 3, 6, 10, 15, 21]).predict_next(), 28);
        assert_eq!(History::new([10, 13, 16, 21, 30, 45]).predict_next(), 68);
    }

    #[test]
    fn test_predict_previous() {
        assert_eq!(History::new([10, 13, 16, 21, 30, 45]).predict_previous(), 5);
        assert_eq!(History::new([0, 3, 6, 9, 12, 15]).predict_previous(), -3);
        assert_eq!(History::new([1, 3, 6, 10, 15, 21]).predict_previous(), 0);
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 114);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 2);
    }
}

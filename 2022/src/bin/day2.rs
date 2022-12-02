use advent_of_code_2022::{read, parse};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::IResult;
use nom::multi::many1;
use nom::sequence::{terminated, separated_pair};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Result {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

type Input1 = Vec<(Play, Play)>;

fn parser1(input: &str) -> IResult<&str, Input1> {
    many1(terminated(separated_pair(
        alt((
            value(Play::Rock, tag("A")),
            value(Play::Paper, tag("B")),
            value(Play::Scissors, tag("C")),
        )),
        tag(" "),
        alt((
            value(Play::Rock, tag("X")),
            value(Play::Paper, tag("Y")),
            value(Play::Scissors, tag("Z")),
        )),
    ), tag("\n")))(input)
}

type Input2 = Vec<(Play, Result)>;

fn parser2(input: &str) -> IResult<&str, Input2> {
    many1(terminated(separated_pair(
        alt((
            value(Play::Rock, tag("A")),
            value(Play::Paper, tag("B")),
            value(Play::Scissors, tag("C")),
        )),
        tag(" "),
        alt((
            value(Result::Lose, tag("X")),
            value(Result::Draw, tag("Y")),
            value(Result::Win, tag("Z")),
        )),
    ), tag("\n")))(input)
}

fn solve_part1(input: &Input1) -> u32 {
    input.iter().map(|(adversary, play)| {
        let result = match adversary {
            Play::Rock => match play {
                Play::Rock => Result::Draw,
                Play::Paper => Result::Win,
                Play::Scissors => Result::Lose,
            },
            Play::Paper => match play {
                Play::Rock => Result::Lose,
                Play::Paper => Result::Draw,
                Play::Scissors => Result::Win,
            },
            Play::Scissors => match play {
                Play::Rock => Result::Win,
                Play::Paper => Result::Lose,
                Play::Scissors => Result::Draw,
            },
        };

        *play as u32 + result as u32
    }).sum()
}

fn solve_part2(input: &Input2) -> u32 {
    input.iter().map(|(adversary, result)| {
        let play = match adversary {
            Play::Rock => match result {
                Result::Draw => Play::Rock,
                Result::Lose => Play::Scissors,
                Result::Win => Play::Paper,
            },
            Play::Paper => match result {
                Result::Win => Play::Scissors,
                Result::Draw => Play::Paper,
                Result::Lose => Play::Rock,
            },
            Play::Scissors => match result {
                Result::Lose => Play::Paper,
                Result::Win => Play::Rock,
                Result::Draw => Play::Scissors,
            },
        };

        play as u32 + *result as u32
    }).sum()
}

fn main() {
    let input = read(2);

    println!("{}", solve_part1(&parse(parser1, &input)));
    println!("{}", solve_part2(&parse(parser2, &input)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser1() {
        assert_eq!(parser1("A Y\nB X\nC Z\n"), Ok(("", vec![
            (Play::Rock, Play::Paper),
            (Play::Paper, Play::Rock),
            (Play::Scissors, Play::Scissors),
        ])));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&vec![
            (Play::Rock, Play::Paper),
            (Play::Paper, Play::Rock),
            (Play::Scissors, Play::Scissors),
        ]), 15);
    }

    #[test]
    fn test_parser2() {
        assert_eq!(parser2("A Y\nB X\nC Z\n"), Ok(("", vec![
            (Play::Rock, Result::Draw),
            (Play::Paper, Result::Lose),
            (Play::Scissors, Result::Win),
        ])));
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&vec![
            (Play::Rock, Result::Draw),
            (Play::Paper, Result::Lose),
            (Play::Scissors, Result::Win),
        ]), 12);
    }
}

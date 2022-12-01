use advent_of_code_2022::{read, parse};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::u32;
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::terminated;

type Input = Vec<Vec<u32>>;

fn parser(input: &str) -> IResult<&str, Input> {
    separated_list1(tag("\n"), many1(terminated(u32, tag("\n"))))(input)
}

fn solve_part1(input: &Input) -> u32 {
    input.iter().map(|foods| foods.iter().sum()).max().unwrap()
}

fn solve_part2(input: &Input) -> u32 {
    input.iter().map(|foods| foods.iter().sum::<u32>()).sorted().rev().take(3).sum()
}

fn main() {
    let input = read(1);

    let parsed = parse(parser, &input);

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(parser("1000\n2000\n3000\n\n4000\n"), Ok(("", vec![
            vec![1000,2000,3000],
            vec![4000],
        ])));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ]), 24000);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ]), 45000);
    }
}

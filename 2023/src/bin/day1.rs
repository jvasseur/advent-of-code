use advent_of_code_2023::{read, parse};
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::IResult;
use nom::multi::many1;
use nom::sequence::terminated;

type Input<'a> = Vec<&'a str>;

fn parser(input: &str) -> IResult<&str, Input> {
    many1(terminated(alphanumeric1, tag("\n")))(input)
}

fn solve_part1(input: &Input) -> u32 {
    input.into_iter()
        .map(|row| {
            let first = row.chars().find(|char| char.is_numeric()).unwrap();
            let last = row.chars().rev().find(|char| char.is_numeric()).unwrap();

            format!("{}{}", first, last).parse::<u32>().unwrap()
        })
        .sum()
}

fn solve_part2(input: &Input) -> u32 {
    input.into_iter()
        .map(|row| {
            let first = [
                (row.find("1"), 1),
                (row.find("2"), 2),
                (row.find("3"), 3),
                (row.find("4"), 4),
                (row.find("5"), 5),
                (row.find("6"), 6),
                (row.find("7"), 7),
                (row.find("8"), 8),
                (row.find("9"), 9),
                (row.find("one"), 1),
                (row.find("two"), 2),
                (row.find("three"), 3),
                (row.find("four"), 4),
                (row.find("five"), 5),
                (row.find("six"), 6),
                (row.find("seven"), 7),
                (row.find("eight"), 8),
                (row.find("nine"), 9),
            ].into_iter()
                .filter_map(|(index, value)| index.map(|u| (u, value)))
                .min_by(|(a, _), (b, _)| a.cmp(b))
                .unwrap().1;

            let last = [
                (row.rfind("1"), 1),
                (row.rfind("2"), 2),
                (row.rfind("3"), 3),
                (row.rfind("4"), 4),
                (row.rfind("5"), 5),
                (row.rfind("6"), 6),
                (row.rfind("7"), 7),
                (row.rfind("8"), 8),
                (row.rfind("9"), 9),
                (row.rfind("one"), 1),
                (row.rfind("two"), 2),
                (row.rfind("three"), 3),
                (row.rfind("four"), 4),
                (row.rfind("five"), 5),
                (row.rfind("six"), 6),
                (row.rfind("seven"), 7),
                (row.rfind("eight"), 8),
                (row.rfind("nine"), 9),
            ].into_iter()
                .filter_map(|(index, value)| index.map(|u| (u, value)))
                .max_by(|(a, _), (b, _)| a.cmp(b))
                .unwrap().1;

            format!("{}{}", first, last).parse::<u32>().unwrap()
        })
        .sum()
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

    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

    fn parsed_input() -> Input<'static> {
        vec![
            "1abc2",
            "pqr3stu8vwx",
            "a1b2c3d4e5f",
            "treb7uchet",
        ]
    }


    fn parsed_input2() -> Input<'static> {
        vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ]
    }

    #[test]
    fn test_parser() {
        assert_eq!(parser(INPUT), Ok(("", parsed_input())));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 142);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input2()), 281);
    }
}

#![feature(int_abs_diff)]

use advent_of_code_2022::{read, parse};
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::u8;
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list0;
use nom::sequence::{delimited, pair, terminated};
use std::cmp::{max, Ordering};

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Integer(u8),
    List(Vec<Packet>),
}

fn compare_lists(a: &Vec<Packet>, b: &Vec<Packet>) -> Ordering {
    for i in 0..max(a.len(), b.len()) {
        if i >= a.len() {
            return Ordering::Less;
        }

        if i >= b.len() {
            return Ordering::Greater;
        }

        if a[i] < b[i] {
            return Ordering::Less;
        }

        if a[i] > b[i] {
            return Ordering::Greater;
        }
    }

    return Ordering::Equal;
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Integer(a), Packet::Integer(b)) => Some(a.cmp(b)),
            (Packet::List(a), Packet::List(b)) => Some(compare_lists(a, b)),
            (Packet::Integer(a), Packet::List(b)) => Some(compare_lists(&vec![Packet::Integer(*a)], b)),
            (Packet::List(a), Packet::Integer(b)) => Some(compare_lists(a, &vec![Packet::Integer(*b)])),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Integer(a), Packet::Integer(b)) => a.cmp(b),
            (Packet::List(a), Packet::List(b)) => compare_lists(a, b),
            (Packet::Integer(a), Packet::List(b)) => compare_lists(&vec![Packet::Integer(*a)], b),
            (Packet::List(a), Packet::Integer(b)) => compare_lists(a, &vec![Packet::Integer(*b)]),
        }
    }
}

type Input = Vec<(Packet, Packet)>;

fn packet_parser(input: &str) -> IResult<&str, Packet> {
    alt((
        map(delimited(tag("["), separated_list0(tag(","), packet_parser), tag("]")), |v| Packet::List(v)),
        map(u8, |v| Packet::Integer(v)),
    ))(input)
}

fn parser(input: &str) -> IResult<&str, Input> {
    separated_list0(tag("\n"), pair(terminated(packet_parser, tag("\n")), terminated(packet_parser, tag("\n"))))(input)
}

fn solve_part1(input: &Input) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(_, (a, b))| a < b)
        .map(|(i, _)| i + 1)
        .sum()
}

fn solve_part2(input: &Input) -> usize {
    let dividers = [
        Packet::List(vec![Packet::Integer(2)]),
        Packet::List(vec![Packet::Integer(6)]),
    ];

    input
        .iter()
        .map(|(a, b)| [a, b])
        .flatten()
        .chain(dividers.iter())
        .sorted()
        .enumerate()
        .filter(|(_, p)| dividers.contains(p))
        .map(|(i, _)| i + 1)
        .product()
}

fn main() {
    let input = read(13);

    let parsed = parse(parser, &input);

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[1,1,3,1,1]\n[1,1,5,1,1]\n\n[[1],[2,3,4]]\n[[1],4]\n\n[9]\n[[8,7,6]]\n\n[[4,4],4,4]\n[[4,4],4,4,4]\n\n[7,7,7,7]\n[7,7,7]\n\n[]\n[3]\n\n[[[]]]\n[[]]\n\n[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]\n";

    fn parsed_input() -> Input {
        parse(parser, INPUT)
    }

    #[test]
    fn test_parser() {
        assert_eq!(parser("[1,1,3,1,1]\n[1,1,5,1,1]\n\n[[1],[2,3,4]]\n[[1],4]\n"), Ok(("",
        vec![
            (
                Packet::List(vec![Packet::Integer(1),Packet::Integer(1),Packet::Integer(3),Packet::Integer(1),Packet::Integer(1)]),
                Packet::List(vec![Packet::Integer(1),Packet::Integer(1),Packet::Integer(5),Packet::Integer(1),Packet::Integer(1)]),
            ),
            (
                Packet::List(vec![Packet::List(vec![Packet::Integer(1)]),Packet::List(vec![Packet::Integer(2),Packet::Integer(3),Packet::Integer(4)])]),
                Packet::List(vec![Packet::List(vec![Packet::Integer(1)]),Packet::Integer(4)]),
            ),
        ])));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 13);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 140);
    }
}

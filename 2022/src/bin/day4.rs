use advent_of_code_2022::{read, parse};
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::character::complete::u32;
use nom::IResult;
use nom::multi::many1;
use nom::sequence::{separated_pair, terminated};

#[derive(Debug, PartialEq)]
struct Pair {
    first: Range,
    second: Range,
}

impl Pair {
    pub fn new(first: Range, second: Range) -> Self {
        Pair {
            first,
            second,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    pub fn new(start: u32, end: u32) -> Self {
        Range {
            start,
            end,
        }
    }

    pub fn includes(&self, other: &Range) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    pub fn overlap(&self, other: &Range) -> bool {
        !self.disjoint(other)
    }

    pub fn disjoint(&self, other: &Range) -> bool {
        other.end < self.start || self.end < other.start
    }
}

type Input = Vec<Pair>;

fn range_parser(input: &str) -> IResult<&str, Range> {
    map(separated_pair(u32, tag("-"), u32), |(start, end)| Range::new(start, end))(input)
}

fn pair_parser(input: &str) -> IResult<&str, Pair> {
    map(separated_pair(range_parser, tag(","), range_parser), |(first, second)| Pair::new(first, second))(input)
}

fn parser(input: &str) -> IResult<&str, Input> {
    many1(terminated(pair_parser, tag("\n")))(input)
}

fn solve_part1(input: &Input) -> usize {
    input.iter().filter(|pair| pair.first.includes(&pair.second) || pair.second.includes(&pair.first)).count()
}

fn solve_part2(input: &Input) -> usize {
    input.iter().filter(|pair| pair.first.overlap(&pair.second)).count()
}

fn main() {
    let input = read(4);

    let parsed = parse(parser, &input);

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(parser("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8\n"), Ok(("", vec![
            Pair::new(Range::new(2, 4), Range::new(6, 8)),
            Pair::new(Range::new(2, 3), Range::new(4, 5)),
            Pair::new(Range::new(5, 7), Range::new(7, 9)),
            Pair::new(Range::new(2, 8), Range::new(3, 7)),
            Pair::new(Range::new(6, 6), Range::new(4, 6)),
            Pair::new(Range::new(2, 6), Range::new(4, 8)),
        ])));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&vec![
            Pair::new(Range::new(2, 4), Range::new(6, 8)),
            Pair::new(Range::new(2, 3), Range::new(4, 5)),
            Pair::new(Range::new(5, 7), Range::new(7, 9)),
            Pair::new(Range::new(2, 8), Range::new(3, 7)),
            Pair::new(Range::new(6, 6), Range::new(4, 6)),
            Pair::new(Range::new(2, 6), Range::new(4, 8)),
        ]), 2);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&vec![
            Pair::new(Range::new(2, 4), Range::new(6, 8)),
            Pair::new(Range::new(2, 3), Range::new(4, 5)),
            Pair::new(Range::new(5, 7), Range::new(7, 9)),
            Pair::new(Range::new(2, 8), Range::new(3, 7)),
            Pair::new(Range::new(6, 6), Range::new(4, 6)),
            Pair::new(Range::new(2, 6), Range::new(4, 8)),
        ]), 4);
    }
}

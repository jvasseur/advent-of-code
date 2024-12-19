use std::collections::HashMap;

use advent_of_code_2024::{parser::*, read};
use nom::{branch::alt, bytes::complete::tag, combinator::value, multi::{many1, separated_list1}, sequence::terminated, IResult};

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
enum Stripe {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl Parsable for Stripe {
    fn parser(input: &str) -> IResult<&str, Self> {
        alt((
           value(Stripe::White, tag("w")),
           value(Stripe::Blue, tag("u")),
           value(Stripe::Black, tag("b")),
           value(Stripe::Red, tag("r")),
           value(Stripe::Green, tag("g")),
        ))(input)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Pattern {
    stripes: Vec<Stripe>
}

impl Pattern {
    fn new(stripes: Vec<Stripe>) -> Self {
        Self { stripes }
    }
}

impl Parsable for Pattern {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, stripes) = many1(Stripe::parser)(input)?;

        Ok((input, Pattern::new(stripes)))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    available: Vec<Pattern>,
    desired: Vec<Pattern>,
}

impl Input {
    fn new(available: Vec<Pattern>, desired: Vec<Pattern>) -> Self {
        Self { available, desired }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, available) = separated_list1(tag(", "), Pattern::parser)(input)?;
        let (input, _) = tag("\n\n")(input)?;
        let (input, desired) = many1(terminated(Pattern::parser, tag("\n")))(input)?;

        Ok((input, Input::new(available, desired)))
    }
}

fn is_valid(stripes: &[Stripe], available: &[Pattern]) -> bool
{
    if stripes.len() == 0 {
        return true;
    }

    for pattern in available {
        if stripes.starts_with(&pattern.stripes) && is_valid(&stripes[pattern.stripes.len()..], available) {
            return true
        }
    }

    return false;
}

fn count_valid<'a>(stripes: &'a[Stripe], available: &'a[Pattern], cache: &mut HashMap<&'a[Stripe], usize>) -> usize
{
    if stripes.len() == 0 {
        return 1;
    }

    if let Some(count) = cache.get(&stripes) {
        return *count;
    }

    let mut count = 0;

    for pattern in available {
        if stripes.starts_with(&pattern.stripes) {
            count += count_valid(&stripes[pattern.stripes.len()..], available, cache);
        }
    }

    cache.insert(stripes, count);

    return count;
}

fn solve_part1(input: &Input) -> usize {
    input.desired.iter().filter(|pattern| is_valid(&pattern.stripes, &input.available)).count()
}

fn solve_part2(input: &Input) -> usize {
    let mut cache = HashMap::new();

    input.desired.iter().map(|pattern| count_valid(&pattern.stripes, &input.available, &mut cache)).sum()
}

fn main() {
    let input = parse(&read(19).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    fn parsed_input() -> Input {
        Input::new(
            vec![
                Pattern::new(vec![Stripe::Red]),
                Pattern::new(vec![Stripe::White, Stripe::Red]),
                Pattern::new(vec![Stripe::Black]),
                Pattern::new(vec![Stripe::Green]),
                Pattern::new(vec![Stripe::Black, Stripe::White, Stripe::Blue]),
                Pattern::new(vec![Stripe::Red, Stripe::Black]),
                Pattern::new(vec![Stripe::Green, Stripe::Black]),
                Pattern::new(vec![Stripe::Black, Stripe::Red]),
            ],
            vec![
                Pattern::new(vec![Stripe::Black, Stripe::Red, Stripe::White, Stripe::Red, Stripe::Red]),
                Pattern::new(vec![Stripe::Black, Stripe::Green, Stripe::Green, Stripe::Red]),
                Pattern::new(vec![Stripe::Green, Stripe::Black, Stripe::Black, Stripe::Red]),
                Pattern::new(vec![Stripe::Red, Stripe::Red, Stripe::Black, Stripe::Green, Stripe::Black, Stripe::Red]),
                Pattern::new(vec![Stripe::Blue, Stripe::Black, Stripe::White, Stripe::Blue, ]),
                Pattern::new(vec![Stripe::Black, Stripe::White, Stripe::Blue, Stripe::Red, Stripe::Red, Stripe::Green]),
                Pattern::new(vec![Stripe::Black, Stripe::Red, Stripe::Green, Stripe::Red]),
                Pattern::new(vec![Stripe::Black, Stripe::Black, Stripe::Red, Stripe::Green, Stripe::White, Stripe::Black]),
            ],
        )
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse::<Input>(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_is_valid() {
        let available = parsed_input().available;

        assert_eq!(is_valid(&[Stripe::Black, Stripe::Red, Stripe::White, Stripe::Red, Stripe::Red], &available), true);
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 6);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 16);
    }
}

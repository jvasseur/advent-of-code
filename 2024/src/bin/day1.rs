use advent_of_code_2024::{parser::*, read};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::u32;
use nom::IResult;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    pairs: Vec<Pair>
}

impl Input {
    fn new(pairs: impl Into<Vec<Pair>>) -> Self {
        Self { pairs: pairs.into() }
    }

    fn left<'a>(&'a self) -> impl Iterator<Item = Id> + 'a {
        self.pairs.iter().map(|pair| pair.left)
    }

    fn right<'a>(&'a self) -> impl Iterator<Item = Id> + 'a {
        self.pairs.iter().map(|pair| pair.right)
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, pairs) = lines_parser(input)?;

        Ok((input, Self::new(pairs)))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Pair {
    left: Id,
    right: Id,
}

impl Pair {
    fn new(left: Id, right: Id) -> Self {
        Self { left, right }
    }
}

impl Parsable for Pair {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, left) = u32(input)?;
        let (input, _) = tag("   ")(input)?;
        let (input, right) = u32(input)?;

        Ok((input, Self::new(left, right)))
    }
}

type Id = u32;

fn solve_part1(input: &Input) -> u32 {
    input.left().sorted().zip(input.right().sorted()).map(|(left, right)| left.abs_diff(right)).sum()
}

fn solve_part2(input: &Input) -> u32 {
    let counts = input.right().counts();

    input.left().map(|left| left * counts.get(&left).map(|&left| left as u32).unwrap_or(0)).sum()
}

fn main() {
    let input = parse(&read(1).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";

    fn parsed_input() -> Input {
        Input::new([
            Pair::new(3, 4),
            Pair::new(4, 3),
            Pair::new(2, 5),
            Pair::new(1, 3),
            Pair::new(3, 9),
            Pair::new(3, 3),
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse::<Input>(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 11);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 31);
    }
}

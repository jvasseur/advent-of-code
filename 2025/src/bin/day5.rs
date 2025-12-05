use std::{cmp::{min, max}, collections::HashSet, ops::RangeInclusive};
use advent_of_code_2025::{parser::*, read};
use nom::{IResult, bytes::complete::tag, combinator::map, multi::many1, sequence::{separated_pair, terminated}};

type IngredientId = u64;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Input {
    ranges: Vec<RangeInclusive<IngredientId>>,
    ids: Vec<IngredientId>,
}

impl Input {
    fn new(ranges: impl Into<Vec<RangeInclusive<IngredientId>>>, ids: impl Into<Vec<IngredientId>>) -> Self {
        Self { ranges: ranges.into(), ids: ids.into() }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                many1(
                    terminated(
                        map(
                            separated_pair(
                                parse,
                                tag("-"),
                                parse,
                            ),
                            |(start, end)| start..=end
                        ),
                        tag("\n"))
                ),
                tag("\n"),
                many1(
                    terminated(
                        parse,
                        tag("\n"),
                    ),
                ),
            ),
            |(ranges, ids)| Input::new(ranges, ids),
        )(input)
    }
}

fn range_intersect(range_a: &RangeInclusive<IngredientId>, range_b: &RangeInclusive<IngredientId>) -> bool {
    range_a.contains(range_b.start()) || range_a.contains(range_b.end()) || range_b.contains(range_a.start()) || range_b.contains(range_a.end())
}

fn solve_part1(input: &Input) -> usize {
    input.ids.iter().filter(|id| input.ranges.iter().any(|range| range.contains(id))).count()
}

fn solve_part2(input: &Input) -> u64 {
    let mut ranges = input.ranges.iter().cloned().collect::<HashSet<_>>();

    'main: loop {
        for range_a in &ranges.clone() {
            for range_b in &ranges.clone() {
                if range_a == range_b {
                    continue;
                }

                if range_intersect(range_a, range_b) {
                    ranges.remove(&range_a);
                    ranges.remove(&range_b);

                    ranges.insert(*min(range_a.start(), range_b.start())..=*max(range_a.end(), range_b.end()));

                    continue 'main;
                }
            }
        }

        break;
    }

    ranges.iter().map(|range| range.end() - range.start() + 1).sum()
}

fn main() {
    let input = from_str(&read(5).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    fn parsed_input() -> Input {
        Input::new([
            3..=5,
            10..=14,
            16..=20,
            12..=18,
        ], [
            1,
            5,
            8,
            11,
            17,
            32,
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse(INPUT), Ok(("", parsed_input())));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 3);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 14);
    }
}

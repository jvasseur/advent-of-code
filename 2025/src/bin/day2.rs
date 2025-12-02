use std::ops::RangeInclusive;
use advent_of_code_2025::{parser::*, read};
use derive_more::IntoIterator;
use nom::{IResult, bytes::complete::tag, combinator::map, multi::separated_list1, sequence::{separated_pair, terminated}};

type Id = u64;

#[derive(Clone, Debug, PartialEq, Eq, IntoIterator)]
#[into_iterator(owned, ref, ref_mut)]
struct Input(Vec<RangeInclusive<Id>>);

impl Input {
    fn new(ranges: impl Into<Vec<RangeInclusive<Id>>>) -> Self {
        Self(ranges.into())
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        map(
            terminated(
                separated_list1(
                    tag(","),
                    map(
                        separated_pair(
                            parse,
                            tag("-"),
                            parse,
                        ),
                        |(start, end)| start..=end,
                    ),
                ),
                tag("\n"),
            ),
            Input::new,
        )(input)
    }
}

fn is_silly_1(value: Id) -> bool {
    let string_value = value.to_string();
    let split = string_value.len() / 2;

    string_value[0..split] == string_value[split..]
}

fn is_silly_2(value: Id) -> bool {
    let string_value = value.to_string();
    let string_length = string_value.len();

    for pattern_length in 1..=(string_length / 2) {
        let div = string_length.div_euclid(pattern_length);
        let rem = string_length.rem_euclid(pattern_length);

        if rem != 0 {
            continue;
        }

        let pattern = &string_value[0..pattern_length];

        if (1..div).all(|i| &string_value[i*pattern_length..(i+1)*pattern_length] == pattern) {
            return true;
        }
    }

    return false;
}

fn add_silly(input: &Input, is_silly: impl Fn(Id) -> bool) -> u64 {
    let mut result = 0;

    for range in input {
        for value in range.to_owned() {
            if is_silly(value) {
                result += value;
            }
        }
    }

    result
}

fn solve_part1(input: &Input) -> u64 {
    add_silly(input, is_silly_1)
}

fn solve_part2(input: &Input) -> u64 {
    add_silly(input, is_silly_2)
}

fn main() {
    let input = from_str(&read(2).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";

    fn parsed_input() -> Input {
        Input::new([
            11..=22,
            95..=115,
            998..=1012,
            1188511880..=1188511890,
            222220..=222224,
            1698522..=1698528,
            446443..=446449,
            38593856..=38593862,
            565653..=565659,
            824824821..=824824827,
            2121212118..=2121212124,
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse(INPUT), Ok(("", parsed_input())));
    }

    #[test]
    fn test_is_silly_1() {
        assert_eq!(is_silly_1(55), true, "55 is silly");
        assert_eq!(is_silly_1(6464), true);
        assert_eq!(is_silly_1(123123), true);
        assert_eq!(is_silly_1(101), false);
    }

    #[test]
    fn test_is_silly_2() {
        assert_eq!(is_silly_2(12341234), true, "12341234 is silly");
        assert_eq!(is_silly_2(123123123), true, "123123123 is silly");
        assert_eq!(is_silly_2(1212121212), true, "1212121212 is silly");
        assert_eq!(is_silly_2(1111111), true, "1111111 is silly");
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 1227775554);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 4174379265);
    }
}

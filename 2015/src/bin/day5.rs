use advent_of_code_2015::{parser::*, read};
use itertools::Itertools;
use nom::{character::complete::{alpha1, char}, combinator::map, multi::many1, sequence::terminated};

struct Input {
    strings: Vec<String>
}

impl Input {
    fn new(strings: Vec<String>) -> Self {
        Self { strings }
    }
}

impl Parsable for Input {
    fn parser<'a>(input: &'a str) -> ParserResult<'a, Self> {
        map(many1(terminated(map(alpha1, |str: &str| str.to_owned()), char('\n'))), Input::new)(input)
    }
}

fn is_nice(input: &str) -> bool {
    has_voyels(input) && has_double(input) && !has_badies(input)
}

fn has_voyels(input: &str) -> bool {
    let mut voyels = 0;

    for c in input.chars() {
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            voyels += 1;

            if voyels >= 3 {
                return true;
            }
        }
    }

    return false;
}

fn has_double(input: &str) -> bool {
    for (a, b) in input.chars().tuple_windows() {
        if a == b {
            return true;
        }
    }

    return false;
}

fn has_badies(input: &str) -> bool {
    input.contains("ab") || input.contains("cd") || input.contains("pq") || input.contains("xy")
}

fn is_nice2(input: &str) -> bool {
    has_repeated_double(input) && has_repeated_separated(input)
}

fn has_repeated_double(input: &str) -> bool {
    for i in 0..(input.len() - 2) {
        let double = &input[i..i+2];
        let rest = &input[i+2..];

        if rest.contains(&double) {
            return true;
        }
    }

    return false;
}

fn has_repeated_separated(input: &str) -> bool {
    for (a, _, b) in input.chars().tuple_windows() {
        if a == b {
            return true;
        }
    }

    return false;
}

fn solve_part1(input: &Input) -> usize {
    input.strings.iter().filter(|line| is_nice(&line)).count()
}

fn solve_part2(input: &Input) -> usize {
    input.strings.iter().filter(|line| is_nice2(&line)).count()
}

fn main() {
    let input = parse(&read(5).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_nice() {
        assert!(is_nice(&"ugknbfddgicrmopn"));
        assert!(is_nice(&"aaa"));
        assert!(!is_nice(&"jchzalrnumimnmhp"));
        assert!(!is_nice(&"haegwjzuvuyypxyu"));
        assert!(!is_nice(&"dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_is_nice_more() {
        assert!(is_nice("iuvrelxiapllaxbg"));
    }

    #[test]
    fn test_is_nice2() {
        assert!(!is_nice2(&"aaa"));

        assert!(is_nice2(&"qjhvhtzxzqqjkmpb"));
        assert!(is_nice2(&"xxyxx"));
        assert!(!is_nice2(&"uurcxstgmygtbstg"));
        assert!(!is_nice2(&"ieodomkazucvgmuy"));
    }
}

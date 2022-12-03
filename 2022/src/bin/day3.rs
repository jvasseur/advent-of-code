use advent_of_code_2022::{read, parse};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::combinator::map_res;
use nom::character::complete::alpha1;
use nom::IResult;
use nom::multi::{many1};
use nom::sequence::terminated;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
struct Rucksack {
    first: Vec<char>,
    second: Vec<char>,
}

impl Rucksack {
    fn item_set(&self) -> HashSet<char> {
        let mut set = HashSet::new();

        set.extend(self.first.clone());
        set.extend(self.second.clone());

        set
    }
}

impl std::str::FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_at(s.len() / 2);

        Ok(Self {
            first: first.chars().collect(),
            second: second.chars().collect(),
        })
    }
}

fn priority(item: char) -> u32 {
    let code = item as u32;

    if code >= 97 {
        return code - 96;
    }

    if code >= 65 {
        return code - 64 + 26;
    }

    panic!("(╯°□°)╯︵ ┻━┻");
}

type Input = Vec<Rucksack>;

fn parser(input: &str) -> IResult<&str, Input> {
    many1(terminated(map_res(alpha1, |s: &str| s.parse::<Rucksack>()), tag("\n")))(input)
}

fn solve_part1(input: &Input) -> u32 {
    input.iter().map(|sack| {
        let first: HashSet<char> = HashSet::from_iter(sack.first.to_owned().into_iter());
        let second: HashSet<char> = HashSet::from_iter(sack.second.to_owned().into_iter());

        priority(*first.intersection(&second).exactly_one().unwrap())
    }).sum()
}

fn solve_part2(input: &Input) -> u32 {
    input.iter().tuples::<(&Rucksack, &Rucksack, &Rucksack)>().map(|(one, two, three)| {
        let intersection: HashSet<char> = one.item_set().intersection(&two.item_set()).cloned().collect();
        let set = three.item_set();

        let badge = intersection.intersection(&set).exactly_one().unwrap();

        priority(*badge)
    }).sum()
}

fn main() {
    let input = read(3);

    let parsed = parse(parser, &input);

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(parser("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw\n"), Ok(("", vec![
            Rucksack {
                first: vec!['v', 'J', 'r', 'w', 'p', 'W', 't', 'w', 'J', 'g', 'W', 'r'],
                second: vec!['h', 'c', 's', 'F', 'M', 'M', 'f', 'F', 'F', 'h', 'F', 'p'],
            },
            Rucksack {
                first: vec!['j', 'q', 'H', 'R', 'N', 'q', 'R', 'j', 'q', 'z', 'j', 'G', 'D', 'L', 'G', 'L'],
                second: vec!['r', 's', 'F', 'M', 'f', 'F', 'Z', 'S', 'r', 'L', 'r', 'F', 'Z', 's', 'S', 'L'],
            },
            Rucksack {
                first: vec!['P', 'm', 'm', 'd', 'z', 'q', 'P', 'r', 'V'],
                second: vec!['v', 'P', 'w', 'w', 'T', 'W', 'B', 'w', 'g'],
            },
            Rucksack {
                first: vec!['w', 'M', 'q', 'v', 'L', 'M', 'Z', 'H', 'h', 'H', 'M', 'v', 'w', 'L', 'H'],
                second: vec!['j', 'b', 'v', 'c', 'j', 'n', 'n', 'S', 'B', 'n', 'v', 'T', 'Q', 'F', 'n'],
            },
            Rucksack {
                first: vec!['t', 't', 'g', 'J', 't', 'R', 'G', 'J'],
                second: vec!['Q', 'c', 't', 'T', 'Z', 't', 'Z', 'T'],
            },
            Rucksack {
                first: vec!['C', 'r', 'Z', 's', 'J', 's', 'P', 'P', 'Z', 's', 'G', 'z'],
                second: vec!['w', 'w', 's', 'L', 'w', 'L', 'm', 'p', 'w', 'M', 'D', 'w'],
            },
        ])));
    }

    #[test]
    fn test_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&vec![
            Rucksack {
                first: vec!['v', 'J', 'r', 'w', 'p', 'W', 't', 'w', 'J', 'g', 'W', 'r'],
                second: vec!['h', 'c', 's', 'F', 'M', 'M', 'f', 'F', 'F', 'h', 'F', 'p'],
            },
            Rucksack {
                first: vec!['j', 'q', 'H', 'R', 'N', 'q', 'R', 'j', 'q', 'z', 'j', 'G', 'D', 'L', 'G', 'L'],
                second: vec!['r', 's', 'F', 'M', 'f', 'F', 'Z', 'S', 'r', 'L', 'r', 'F', 'Z', 's', 'S', 'L'],
            },
            Rucksack {
                first: vec!['P', 'm', 'm', 'd', 'z', 'q', 'P', 'r', 'V'],
                second: vec!['v', 'P', 'w', 'w', 'T', 'W', 'B', 'w', 'g'],
            },
            Rucksack {
                first: vec!['w', 'M', 'q', 'v', 'L', 'M', 'Z', 'H', 'h', 'H', 'M', 'v', 'w', 'L', 'H'],
                second: vec!['j', 'b', 'v', 'c', 'j', 'n', 'n', 'S', 'B', 'n', 'v', 'T', 'Q', 'F', 'n'],
            },
            Rucksack {
                first: vec!['t', 't', 'g', 'J', 't', 'R', 'G', 'J'],
                second: vec!['Q', 'c', 't', 'T', 'Z', 't', 'Z', 'T'],
            },
            Rucksack {
                first: vec!['C', 'r', 'Z', 's', 'J', 's', 'P', 'P', 'Z', 's', 'G', 'z'],
                second: vec!['w', 'w', 's', 'L', 'w', 'L', 'm', 'p', 'w', 'M', 'D', 'w'],
            },
        ]), 157);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&vec![
            Rucksack {
                first: vec!['v', 'J', 'r', 'w', 'p', 'W', 't', 'w', 'J', 'g', 'W', 'r'],
                second: vec!['h', 'c', 's', 'F', 'M', 'M', 'f', 'F', 'F', 'h', 'F', 'p'],
            },
            Rucksack {
                first: vec!['j', 'q', 'H', 'R', 'N', 'q', 'R', 'j', 'q', 'z', 'j', 'G', 'D', 'L', 'G', 'L'],
                second: vec!['r', 's', 'F', 'M', 'f', 'F', 'Z', 'S', 'r', 'L', 'r', 'F', 'Z', 's', 'S', 'L'],
            },
            Rucksack {
                first: vec!['P', 'm', 'm', 'd', 'z', 'q', 'P', 'r', 'V'],
                second: vec!['v', 'P', 'w', 'w', 'T', 'W', 'B', 'w', 'g'],
            },
            Rucksack {
                first: vec!['w', 'M', 'q', 'v', 'L', 'M', 'Z', 'H', 'h', 'H', 'M', 'v', 'w', 'L', 'H'],
                second: vec!['j', 'b', 'v', 'c', 'j', 'n', 'n', 'S', 'B', 'n', 'v', 'T', 'Q', 'F', 'n'],
            },
            Rucksack {
                first: vec!['t', 't', 'g', 'J', 't', 'R', 'G', 'J'],
                second: vec!['Q', 'c', 't', 'T', 'Z', 't', 'Z', 'T'],
            },
            Rucksack {
                first: vec!['C', 'r', 'Z', 's', 'J', 's', 'P', 'P', 'Z', 's', 'G', 'z'],
                second: vec!['w', 'w', 's', 'L', 'w', 'L', 'm', 'p', 'w', 'M', 'D', 'w'],
            },
        ]), 70);
    }
}

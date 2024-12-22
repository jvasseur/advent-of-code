use std::collections::HashMap;
use advent_of_code_2024::{parser::*, read};
use itertools::Itertools;
use nom::{combinator::into, IResult};

type Number = u64;

fn mix(a: Number, b: Number) -> Number {
    a ^ b
}

fn prune(a: Number) -> Number {
    a % 16777216
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Monkey {
    value: Number,
}

impl From<Number> for Monkey {
    fn from(value: Number) -> Self {
        Monkey { value }
    }
}

impl Parsable for Monkey {
    fn parser(input: &str) -> IResult<&str, Self> {
        into(Number::parser)(input)
    }
}

impl Iterator for Monkey {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.value;

        self.value = prune(mix(self.value, self.value * 64));
        self.value = prune(mix(self.value, self.value / 32));
        self.value = prune(mix(self.value, self.value * 2048));

        Some(value)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    monkeys: Vec<Monkey>
}

impl Input {
    fn new(monkeys: Vec<Monkey>) -> Self {
        Self { monkeys }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, monkeys) = lines_parser(input)?;

        Ok((input, Input::new(monkeys)))
    }
}

fn solve_part1(input: &Input) -> Number {
    input.monkeys.iter().map(|monkey| monkey.clone().nth(2000).unwrap()).sum()
}

fn solve_part2(input: &Input) -> Number {
    let mut global_map = HashMap::new();

    for monkey in &input.monkeys {
        let prices = monkey.clone().take(2001).map(|a| a % 10);

        let diffs = prices.clone().tuple_windows().map(|(a, b)| (b as i32) - (a as i32));
        let diffs_seqs = diffs.clone().tuple_windows::<(i32, i32, i32, i32)>();

        let diffs_with_price = diffs_seqs.zip(prices.skip(4));

        let mut monkey_map = HashMap::new();
        for (diffs_seq, price) in diffs_with_price {
            if monkey_map.contains_key(&diffs_seq) {
                continue;
            }

            monkey_map.insert(diffs_seq, price);
        }

        for (diffs_seq, price) in monkey_map {
            *global_map.entry(diffs_seq).or_default() += price;
        }
    }

    *global_map.values().max().unwrap()
}

fn main() {
    let input = parse(&read(22).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1
10
100
2024
";

    fn parsed_input() -> Input {
        Input::new(vec![
            Monkey::from(1),
            Monkey::from(10),
            Monkey::from(100),
            Monkey::from(2024),
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse::<Input>(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_values() {
        let values = Monkey::from(123).take(11).collect::<Vec<_>>();

        assert_eq!(values, vec![
            123,
            15887950,
            16495136,
            527345,
            704524,
            1553684,
            12683156,
            11100544,
            12249484,
            7753432,
            5908254,
        ]);
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 37327623);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&Input::new(vec![
            Monkey::from(1),
            Monkey::from(2),
            Monkey::from(3),
            Monkey::from(2024),
        ])), 23);
    }
}

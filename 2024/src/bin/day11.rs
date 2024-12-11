use std::collections::HashMap;

use advent_of_code_2024::{parser::*, read};
use nom::{bytes::complete::tag, multi::separated_list1, sequence::terminated, IResult};

type Stone = u64;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    stones: Vec<Stone>
}

impl Input {
    fn new(stones: impl Into<Vec<Stone>>) -> Self {
        Self { stones: stones.into() }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, stones) = terminated(separated_list1(tag(" "), Stone::parser), tag("\n"))(input)?;

        Ok((input, Input::new(stones)))
    }
}

fn count_stones(stone: Stone, blinks: u8, cache: &mut HashMap<(Stone, u8), u64>) -> u64 {
    if blinks == 0 {
        return 1;
    }

    if let Some(count) = cache.get(&(stone, blinks)) {
        return *count;
    }

    let count = if stone == 0 {
        count_stones(1, blinks - 1, cache)
    } else {
        let stone_string = stone.to_string();
        if stone_string.len() % 2 == 0 {
            let (first, second) = stone_string.split_at(stone_string.len() / 2);

            count_stones(first.parse().unwrap(), blinks - 1, cache) + count_stones(second.parse().unwrap(), blinks - 1, cache)
        } else {
            count_stones(stone * 2024, blinks - 1, cache)
        }
    };

    cache.insert((stone, blinks), count);

    count
}

fn solve_part1(input: &Input) -> u64 {
    input.stones.iter().map(|&stone| count_stones(stone, 25, &mut HashMap::new())).sum()
}

fn solve_part2(input: &Input) -> u64 {
    input.stones.iter().map(|&stone| count_stones(stone, 75, &mut HashMap::new())).sum()
}

fn main() {
    let input = parse(&read(11).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0 1 10 99 999\n";

    fn parsed_input() -> Input {
        Input::new([0, 1, 10, 99, 999])
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse::<Input>(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&Input::new([125, 17])), 55312);
    }
}

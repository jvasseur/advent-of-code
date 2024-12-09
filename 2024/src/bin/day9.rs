use advent_of_code_2024::{parser::*, read};
use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete::{anychar, u8}, combinator::{map_parser, recognize}, multi::many1, sequence::terminated, IResult};

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    data: Vec<u8>,
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, data) = terminated(
            many1(
                map_parser(
                    recognize(anychar),
                     u8,
                ),
            ),
            tag("\n"),
        )(input)?;

        Ok((input, Input { data }))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Filesystem {
    blocks: Vec<Option<usize>>,
}

impl Filesystem {
    fn first_empty(&self) -> usize {
        self.blocks.iter().enumerate().find(|(_, value)| value.is_none()).unwrap().0
    }

    fn last_filled(&self) -> usize {
        self.blocks.iter().enumerate().rev().find(|(_, value)| value.is_some()).unwrap().0
    }

    fn checksum(&self) -> usize {
        self.blocks.iter().enumerate().map(|(index, value)| value.unwrap_or(0) * index).sum()
    }
}

impl From<&Input> for Filesystem {
    fn from(value: &Input) -> Self {
        let mut blocks = Vec::new();

        let mut id = 0;
        let mut file = true;
        for &size in &value.data {
            if file {
                for _ in 0..size {
                    blocks.push(Some(id));
                }

                id += 1;
                file = false;
            } else {
                for _ in 0..size {
                    blocks.push(None);
                }

                file = true;
            }
        }

        Self { blocks }
    }
}

fn solve_part1(input: &Input) -> usize {
    let mut filesystem = Filesystem::from(input);

    loop {
        let last_filled = filesystem.last_filled();
        let first_empty = filesystem.first_empty();

        if last_filled < first_empty {
            break;
        }

        filesystem.blocks.swap(last_filled, first_empty);
    }

    filesystem.checksum()
}

fn solve_part2(input: &Input) -> usize {
    0
}

fn main() {
    let input = parse(&read(9).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402\n";

    fn parsed_input() -> Input {
        Input { data: vec![2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2] }
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse::<Input>(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 1928);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 0);
    }
}

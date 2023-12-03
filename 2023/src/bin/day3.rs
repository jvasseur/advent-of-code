use advent_of_code_2023::{read, Parsable};
use std::collections::HashSet;
use std::ops::RangeInclusive;
use nom::bytes::complete::tag;
use nom::character::complete::satisfy;
use nom::IResult;
use nom::multi::many1;
use nom::sequence::terminated;

#[derive(PartialEq, Eq, Debug)]
struct Input {
    lines: Vec<Vec<char>>,
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, lines) = many1(terminated(many1(satisfy(|c| c != '\n')), tag("\n")))(input)?;

        Ok((input, Input { lines }))
    }
}

impl Input {
    fn width(&self) -> i32 {
        self.lines[0].len() as i32
    }

    fn height(&self) -> i32 {
        self.lines.len() as i32
    }

    fn get(&self, i: i32, j: i32) -> char {
        if i < 0 || i >= self.height() || j < 0 || j >= self.width() {
            return '.';
        }

        self.lines[i as usize][j as usize]
    }

    fn is_symbol(&self, i: i32, j: i32) -> bool {
        let char = self.get(i, j);

        !char.is_numeric() && char != '.'
    }

    fn is_gear_symbol(&self, i: i32, j: i32) -> bool {
        let char = self.get(i, j);

        char == '*'
    }

    fn get_number(&self, i: i32, j: i32) -> Option<(u32, RangeInclusive<i32>)> {
        if !self.get(i, j).is_numeric() {
            return None;
        }

        let mut start = j;
        while self.get(i, start - 1).is_numeric() {
            start -= 1;
        }

        let mut end = j;
        while self.get(i, end + 1).is_numeric() {
            end += 1;
        }

        let mut value = "".to_owned();
        for j2 in start..=end {
            value = format!("{}{}", value, self.get(i, j2));
        }

        Some((value.parse().unwrap(), start..=end))
    }

    fn has_symbol_aroud(&self, i: i32, j: RangeInclusive<i32>) -> bool {
        // Line above
        for j2 in j.start() - 1..=j.end() + 1 {
            if self.is_symbol(i - 1, j2) {
                return true;
            }
        }

        // Line under
        for j2 in j.start() - 1..=j.end() + 1 {
            if self.is_symbol(i + 1, j2) {
                return true;
            }
        }

        // Char before
        if self.is_symbol(i, j.start() - 1) {
            return true;
        }

        // Char after
        if self.is_symbol(i, j.end() + 1) {
            return true;
        }

        return false;
    }
}

fn solve_part1(input: &Input) -> u32 {
    let mut sum = 0;

    for i in 0..input.height() {
        let mut current = None;

        for j in 0..input.width() {
            let char = input.get(i, j);

            if char.is_numeric() {
                match current {
                    Some((start, value)) => {
                        current = Some((start, format!("{}{}", value, char)));
                    },
                    None => {
                        current = Some((j, format!("{}", char)));
                    },
                }
            } else {
                match &current {
                    Some((start, value)) => {
                        let end = j - 1;

                        if input.has_symbol_aroud(i, *start..=end) {
                            sum += value.parse::<u32>().unwrap();
                        }

                        current = None;
                    },
                    None => {},
                }
            }
        }

        match &current {
            Some((start, value)) => {
                let end = input.width() - 1;

                if input.has_symbol_aroud(i, *start..=end) {
                    sum += value.parse::<u32>().unwrap();
                }
            },
            None => {},
        }
    }

    sum
}

fn solve_part2(input: &Input) -> u32 {
    let mut sum = 0;

    for i in 0..input.height() {
        for j in 0..input.width() {
            if !input.is_gear_symbol(i, j) {
                continue;
            }

            let neighbours = [
                (i-1, j-1),
                (i-1, j),
                (i-1, j+1),
                (i, j-1),
                (i, j+1),
                (i+1, j-1),
                (i+1, j),
                (i+1, j+1),
            ];

            let mut numbers = HashSet::new();

            for (i2, j2) in neighbours {
                match input.get_number(i2, j2) {
                    Some(number) => {
                        numbers.insert(number);
                    },
                    None => {},
                }
            }

            match numbers.len() {
                0 => {},
                1 => {},
                2 => {
                    let numbers = numbers.into_iter().collect::<Vec<(u32, RangeInclusive<i32>)>>();

                    sum += numbers[0].0 * numbers[1].0;
                },
                _ => panic!("Found to many numbers"),
            }
        }
    }

    sum
}

fn main() {
    let input = read(3);

    let parsed = Input::parse(&input).unwrap();

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    fn parsed_input() -> Input {
        Input {
            lines: vec![
                "467..114..".chars().collect(),
                "...*......".chars().collect(),
                "..35..633.".chars().collect(),
                "......#...".chars().collect(),
                "617*......".chars().collect(),
                ".....+.58.".chars().collect(),
                "..592.....".chars().collect(),
                "......755.".chars().collect(),
                "...$.*....".chars().collect(),
                ".664.598..".chars().collect(),
            ],
        }
    }

    #[test]
    fn test_parser() {
        assert_eq!(Input::parse(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 4361);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 467835);
    }
}

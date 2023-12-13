use std::cmp::min;
use advent_of_code_2023::util::{Grid, Point};
use advent_of_code_2023::{read, Parsable};
use nom::bytes::complete::tag;
use nom::IResult;
use nom::combinator::value;
use nom::multi::{many1, separated_list1};
use nom::sequence::terminated;
use nom::branch::alt;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    patterns: Vec<Pattern>,
}

impl Input {
    fn new(patterns: impl Into<Vec<Pattern>>) -> Self {
        Input { patterns: patterns.into() }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, patterns) = separated_list1(tag("\n"), Pattern::parser)(input)?;

        Ok((input, Input::new(patterns)))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Pattern {
    grid: Grid<Element>,
}

impl Pattern {
    fn new(grid: impl Into<Grid<Element>>) -> Self {
        Pattern { grid: grid.into() }
    }

    fn reflections<'a>(&'a self) -> impl Iterator<Item=Reflection> + 'a {
        let vertical = (1..self.grid.cols())
            .filter_map(|col| {
                let size = min(col, self.grid.cols() - col);

                for i in 0..size {
                    let col_before = self.grid.get_col(col - 1 - i);
                    let col_after = self.grid.get_col(col + i);

                    if col_before != col_after {
                        return None;
                    }
                }

                return Some(Reflection::Vertical(col));
            });

        let horizontal = (1..self.grid.rows())
            .filter_map(|row| {
                let size = min(row, self.grid.rows() - row);

                for i in 0..size {
                    let row_before = self.grid.get_row(row - 1 - i);
                    let row_after = self.grid.get_row(row + i);

                    if row_before != row_after {
                        return None;
                    }
                }

                return Some(Reflection::Horizontal(row));
            });

        vertical.chain(horizontal)
    }
}

impl Parsable for Pattern {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, rows) = many1(terminated(many1(Element::parser), tag("\n")))(input)?;

        Ok((input, Pattern::new(rows)))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Element {
    Rock,
    Ash,
}

impl Parsable for Element {
    fn parser(input: &str) -> IResult<&str, Self> {
        alt((
            value(Element::Rock, tag("#")),
            value(Element::Ash, tag(".")),
        ))(input)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Reflection {
    Vertical(usize),
    Horizontal(usize),
}

fn solve_part1(input: &Input) -> usize {
    input.patterns.iter().map(|pattern| match pattern.reflections().next() {
        Some(Reflection::Vertical(col)) => col,
        Some(Reflection::Horizontal(row)) => row * 100,
        None => panic!("Found no reflection"),
    }).sum()
}

fn solve_part2(input: &Input) -> usize {
    input.patterns.iter().map(|pattern| {
        let original_reflection = match pattern.reflections().next() {
            Some(reflection) => reflection,
            None => panic!("Found no reflection"),
        };

        for row in 0..pattern.grid.rows() {
            for col in 0..pattern.grid.cols() {
                let mut smudged = pattern.clone();

                let point = Point { row, col };

                smudged.grid.set(&point, match smudged.grid.get(&point) {
                    Element::Ash => Element::Rock,
                    Element::Rock => Element::Ash,
                });

                let reflection = smudged.reflections()
                    .filter(|reflection| reflection != &original_reflection)
                    .next();

                match reflection {
                    Some(Reflection::Vertical(col)) => {
                        return col;
                    },
                    Some(Reflection::Horizontal(row)) => {
                        return row * 100;
                    },
                    None => {
                        continue;
                    },
                }
            }
        }

        panic!("Found no smudeged reflection")
    }).sum()
}

fn main() {
    let input = read(13);
    let parsed = Input::parse(&input).unwrap();

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Element::*;

    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    fn parsed_input() -> Input {
        Input::new(vec![
            Pattern::new(vec![
                vec![Rock, Ash, Rock, Rock, Ash, Ash, Rock, Rock, Ash],
                vec![Ash, Ash, Rock, Ash, Rock, Rock, Ash, Rock, Ash],
                vec![Rock, Rock, Ash, Ash, Ash, Ash, Ash, Ash, Rock],
                vec![Rock, Rock, Ash, Ash, Ash, Ash, Ash, Ash, Rock],
                vec![Ash, Ash, Rock, Ash, Rock, Rock, Ash, Rock, Ash],
                vec![Ash, Ash, Rock, Rock, Ash, Ash, Rock, Rock, Ash],
                vec![Rock, Ash, Rock, Ash, Rock, Rock, Ash, Rock, Ash],
            ]),
            Pattern::new(vec![
                vec![Rock, Ash, Ash, Ash, Rock, Rock, Ash, Ash, Rock],
                vec![Rock, Ash, Ash, Ash, Ash, Rock, Ash, Ash, Rock],
                vec![Ash, Ash, Rock, Rock, Ash, Ash, Rock, Rock, Rock],
                vec![Rock, Rock, Rock, Rock, Rock, Ash, Rock, Rock, Ash],
                vec![Rock, Rock, Rock, Rock, Rock, Ash, Rock, Rock, Ash],
                vec![Ash, Ash, Rock, Rock, Ash, Ash, Rock, Rock, Rock],
                vec![Rock, Ash, Ash, Ash, Ash, Rock, Ash, Ash, Rock],
            ]),
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(Input::parse(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 405);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 400);
    }
}

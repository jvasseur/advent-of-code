use advent_of_code_2024::{grid::{Grid, Point}, parser::*, read};
use nom::{branch::alt, bytes::complete::tag, combinator::value, multi::{many1, separated_list1}, sequence::terminated, IResult};

#[derive(Clone, PartialEq, Eq, Debug)]
struct Schematic {
    pins: Grid<bool>
}

impl Schematic {
    fn is_lock(&self) -> bool {
        (0..self.pins.cols()).all(|col| *self.pins.get(&Point { row: 0, col: col as i32 }))
    }

    fn is_key(&self) -> bool {
        (0..self.pins.cols()).all(|col| *self.pins.get(&Point { row: self.pins.rows() as i32 - 1, col: col as i32 }))
    }

    fn heights(&self) -> Vec<usize> {
        (0..self.pins.cols()).map(|col| (0..self.pins.rows()).filter(|&row| *self.pins.get(&Point { row: row as i32, col: col as i32 })).count() - 1).collect()
    }
}

impl Parsable for Schematic {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, pins) = many1(
            terminated(
                many1(
                    alt((
                        value(true, tag("#")),
                        value(false, tag(".")),
                    )),
                ),
                tag("\n"),
            )
        )(input)?;

        Ok((input, Schematic { pins: pins.into() }))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    schematics: Vec<Schematic>,
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, schematics) = separated_list1(tag("\n"), Schematic::parser)(input)?;

        Ok((input, Input { schematics }))
    }
}

fn solve_part1(input: &Input) -> usize {
    let schemantic_height = input.schematics[0].pins.rows() - 2;

    let keys_heights = input.schematics.iter().filter(|schematic| schematic.is_key()).map(|schematic| schematic.heights()).collect::<Vec<_>>();
    let locks_heights = input.schematics.iter().filter(|schematic| schematic.is_lock()).map(|schematic| schematic.heights()).collect::<Vec<_>>();

    let mut count = 0;

    for key_heights in &keys_heights {
        for lock_heights in &locks_heights {
            if key_heights.iter().zip(lock_heights).all(|(key_height, lock_height)| key_height + lock_height <= schemantic_height) {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let input = parse(&read(25).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";

    fn parsed_input() -> Input {
        parse::<Input>(INPUT).unwrap()
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse::<Input>(INPUT).unwrap().schematics.len(), 5);
    }

    #[test]
    fn test_is_lock() {
        let input = parse::<Input>(INPUT).unwrap();

        assert_eq!(input.schematics[0].is_lock(), true);
        assert_eq!(input.schematics[1].is_lock(), true);
        assert_eq!(input.schematics[2].is_lock(), false);
        assert_eq!(input.schematics[3].is_lock(), false);
        assert_eq!(input.schematics[4].is_lock(), false);
    }

    #[test]
    fn test_is_key() {
        let input = parse::<Input>(INPUT).unwrap();

        assert_eq!(input.schematics[0].is_key(), false);
        assert_eq!(input.schematics[1].is_key(), false);
        assert_eq!(input.schematics[2].is_key(), true);
        assert_eq!(input.schematics[3].is_key(), true);
        assert_eq!(input.schematics[4].is_key(), true);
    }

    #[test]
    fn test_heights() {
        let input = parse::<Input>(INPUT).unwrap();

        assert_eq!(input.schematics[0].heights(), vec![0, 5, 3, 4, 3]);
        assert_eq!(input.schematics[1].heights(), vec![1, 2, 0, 5, 3]);
        assert_eq!(input.schematics[2].heights(), vec![5, 0, 2, 1, 3]);
        assert_eq!(input.schematics[3].heights(), vec![4, 3, 4, 0, 2]);
        assert_eq!(input.schematics[4].heights(), vec![3, 0, 2, 0, 1]);
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 3);
    }
}

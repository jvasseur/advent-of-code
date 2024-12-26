use advent_of_code_2015::{parser::*, read};
use nom::{combinator::map, multi::many1, character::complete::{char, u32}, sequence::{terminated, tuple}};
use std::cmp::min;

#[derive(Clone,Debug,Eq,PartialEq)]
struct Gift {
    l: u32,
    h: u32,
    w: u32,
}

impl Gift {
    fn new(l: u32, h: u32, w:u32) -> Self {
        Self { l, h, w }
    }
}

impl Gift {
    fn wrapping(&self) -> u32 {
        let side0 = self.l * self.w;
        let side1 = self.w * self.h;
        let side2 = self.h * self.l;

        let min_side = min(side0, min(side1, side2));

        2 * (side0 + side1 + side2) + min_side
    }

    fn ribon(&self) -> u32 {
        let perimeter0 = 2 * self.l + 2 * self.w;
        let perimeter1 = 2 * self.w + 2 * self.h;
        let perimeter2 = 2 * self.h + 2 * self.l;

        let wrap = min(perimeter0, min(perimeter1, perimeter2));

        let bow = self.l * self.h * self.w;

        wrap + bow
    }
}

impl Parsable for Gift {
    fn parser<'a>(input: &'a str) -> ParserResult<'a, Self> {
        map(tuple((u32, char('x'), u32, char('x'), u32)), |(l, _, h, _, w)| Gift::new(l, h, w))(input)
    }
}

struct Input {
    gifts: Vec<Gift>,
}

impl Input {
    fn new(gifts: Vec<Gift>) -> Self {
        Self { gifts }
    }
}

impl Parsable for Input {
    fn parser<'a>(input: &'a str) -> ParserResult<'a, Self> {
        map(many1(terminated(Gift::parser, char('\n'))), Input::new)(input)
    }
}

fn solve_part1(input: &Input) -> u32 {
    input.gifts.iter().map(|gift| gift.wrapping()).sum()
}

fn solve_part2(input: &Input) -> u32 {
    input.gifts.iter().map(|gift| gift.ribon()).sum()
}

fn main() {
    let input = parse(&read(2).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(Gift::parser("4x3x2"), Ok(("", Gift::new(4, 3, 2))));
        assert_eq!(Gift::parser("1x2x3"), Ok(("", Gift::new(1, 2, 3))));
    }

    #[test]
    fn test_wrapping() {
        assert_eq!(Gift::new(2, 3, 4).wrapping(), 58);
        assert_eq!(Gift::new(1, 1, 10).wrapping(), 43);
    }

    #[test]
    fn test_ribon() {
        assert_eq!(Gift::new(2, 3, 4).ribon(), 34);
        assert_eq!(Gift::new(1, 1, 10).ribon(), 14);
    }
}

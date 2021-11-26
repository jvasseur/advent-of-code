use advent_of_code_2015::{read, parse_lines};
use nom::IResult;
use nom::character::complete::{char, u32};
use nom::sequence::tuple;
use std::cmp::min;

#[derive(Clone,Debug,Eq,PartialEq)]
struct Gift {
    l: u32,
    h: u32,
    w: u32,
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

fn parser(input: &str) -> IResult<&str, Gift> {
    let (rest, (l, _, h, _, w)) = tuple((u32, char('x'), u32, char('x'), u32))(input)?;

    Ok((rest, Gift { l, h, w }))
}

fn solve_part1(input: &[Gift]) -> u32 {
    input.iter().fold(0, |wrapping, gift: &Gift| wrapping + gift.wrapping())
}

fn solve_part2(input: &[Gift]) -> u32 {
    input.iter().fold(0, |wrapping, gift: &Gift| wrapping + gift.ribon())
}

fn main() {
    let input = read(2);

    let parsed_input = parse_lines(parser, &input);

    println!("{}", solve_part1(&parsed_input));
    println!("{}", solve_part2(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::Gift;
    use super::parser;
    use super::solve_part1;
    use super::solve_part2;

    #[test]
    fn test_parse() {
        assert_eq!(parser("4x3x2"), Ok(("", Gift { l: 4, h: 3, w: 2 })));
        assert_eq!(parser("1x2x3"), Ok(("", Gift { l: 1, h: 2, w: 3 })));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&vec![
            Gift { l: 2, h: 3, w: 4 },
        ]), 58);
        assert_eq!(solve_part1(&vec![
            Gift { l: 1, h: 1, w: 10 },
        ]), 43);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&vec![
            Gift { l: 2, h: 3, w: 4 },
        ]), 34);
        assert_eq!(solve_part2(&vec![
            Gift { l: 1, h: 1, w: 10 },
        ]), 14);
    }
}

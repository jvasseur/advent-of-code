use nom::IResult;
use nom::character::complete::char;
use nom::character::complete::u32;
use nom::character::complete::newline;
use nom::error::Error;
use nom::multi::many0;
use nom::sequence::terminated;
use nom::sequence::tuple;
use std::cmp::min;
use super::util::apply;

#[derive(Clone,Debug,Eq,PartialEq)]
pub struct Gift {
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

pub fn parse_line(input: &str) -> IResult<&str, Gift> {
    let (rest, (l, _, h, _, w)) = tuple((u32, char('x'), u32, char('x'), u32))(input)?;

    Ok((rest, Gift { l, h, w }))
}

pub fn parse(input: &str) -> Result<Vec<Gift>, Error<&str>> {
    apply(many0(terminated(parse_line, newline)), input)
}

pub fn solve_part1(input: &[Gift]) -> u32 {
    input.iter().fold(0, |wrapping, gift: &Gift| wrapping + gift.wrapping())
}

pub fn solve_part2(input: &[Gift]) -> u32 {
    input.iter().fold(0, |wrapping, gift: &Gift| wrapping + gift.ribon())
}

#[cfg(test)]
mod tests {
    use super::Gift;
    use super::parse;
    use super::solve_part1;
    use super::solve_part2;

    #[test]
    fn test_parse() {
        assert_eq!(parse("4x3x2\n1x2x3\n"), Ok(vec![
            Gift { l: 4, h: 3, w: 2 },
            Gift { l: 1, h: 2, w: 3 },
        ]));
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

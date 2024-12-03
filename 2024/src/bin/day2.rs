use advent_of_code_2024::{parser::*, read};
use itertools::Itertools;
use nom::{bytes::complete::tag, multi::separated_list1, IResult};
use nom::character::complete::u32;

type Level = u32;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Report {
    levels: Vec<Level>
}

impl Report {
    fn new(levels: impl Into<Vec<Level>>) -> Self {
        Self { levels: levels.into() }
    }

    fn is_safe(&self) -> bool {
        Report::are_levels_gradual(&self.levels)
    }

    fn is_safe_with_dampener(&self) -> bool {
        if Report::are_levels_gradual(&self.levels) {
            return true;
        }

        if self.levels.iter().cloned().combinations(self.levels.len() - 1).any(|levels| Report::are_levels_gradual(&levels)) {
            return true;
        }

        return false;
    }

    fn are_levels_gradual(levels: &[Level]) -> bool {
        if levels.iter().tuple_windows().all(|(&a, &b)| b > a && b <= a + 3) {
            return true;
        }

        if levels.iter().tuple_windows().all(|(&a, &b)| a > b && a <= b + 3) {
            return true;
        }

        return false;
    }
}

impl Parsable for Report {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, levels) = separated_list1(tag(" "), u32)(input)?;

        Ok((input, Report::new(levels)))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    reports: Vec<Report>
}

impl Input {
    fn new(reports: impl Into<Vec<Report>>) -> Self {
        Self { reports: reports.into() }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, reports) = lines_parser(input)?;

        Ok((input, Input::new(reports)))
    }
}

fn solve_part1(input: &Input) -> usize {
    input.reports.iter().filter(|report| report.is_safe()).count()
}

fn solve_part2(input: &Input) -> usize {
    input.reports.iter().filter(|report| report.is_safe_with_dampener()).count()
}

fn main() {
    let input = parse(&read(2).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    fn parsed_input() -> Input {
        Input::new([
            Report::new([7, 6, 4, 2, 1]),
            Report::new([1, 2, 7, 8, 9]),
            Report::new([9, 7, 6, 2, 1]),
            Report::new([1, 3, 2, 4, 5]),
            Report::new([8, 6, 4, 4, 1]),
            Report::new([1, 3, 6, 7, 9]),
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse::<Input>(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 2);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 4);
    }
}

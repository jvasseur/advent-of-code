use advent_of_code_2023::{read, Parsable};
use nom::bytes::complete::tag;
use nom::character::complete::u32;
use nom::IResult;
use nom::combinator::{map, value};
use nom::multi::{many0, separated_list0};
use nom::sequence::terminated;
use nom::branch::alt;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    rows: Vec<Row>,
}

impl Input {
    fn new(rows: impl Into<Vec<Row>>) -> Self {
        Input { rows: rows.into() }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, rows) = many0(terminated(Row::parser, tag("\n")))(input)?;

        Ok((input, Input::new(rows)))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Row {
    springs: Vec<Option<Spring>>,
    groups: Vec<u32>,
}

impl Row {
    fn new(springs: impl Into<Vec<Option<Spring>>>, groups: impl Into<Vec<u32>>) -> Self {
        Row { springs: springs.into(), groups: groups.into() }
    }

    fn combinaisons(&self) -> Vec<Vec<Spring>> {
        let mut combinaisons = vec![vec![]];

        for (row, value) in self.springs.iter().enumerate() {
            match value {
                Some(spring) => {
                    combinaisons.iter_mut().for_each(|combinaison| combinaison.push(*spring));
                },
                None => {
                    let mut operational = combinaisons.clone();
                    let mut damaged = combinaisons.clone();

                    operational.iter_mut().for_each(|combinaison| combinaison.push(Spring::Operational));
                    damaged.iter_mut().for_each(|combinaison| combinaison.push(Spring::Damaged));

                    combinaisons = [operational, damaged].concat();
                },
            }
        }

        combinaisons = combinaisons.into_iter()
            .filter(|combinaison| {
                let mut groups = Vec::new();
                let mut damaged = 0;

                for spring in combinaison {
                    match spring {
                        Spring::Operational => {
                            if damaged > 0 {
                                groups.push(damaged);
                            }

                            damaged = 0;
                        },
                        Spring::Damaged => {
                            damaged += 1;
                        },
                    }
                }

                if damaged > 0 {
                    groups.push(damaged);
                }

                groups == self.groups
            })
            .collect();

        combinaisons
    }

    fn unfold(&self) -> Self {
        Row {
            springs: [self.springs.clone(), vec![None], self.springs.clone(), vec![None], self.springs.clone(), vec![None], self.springs.clone(), vec![None], self.springs.clone()].concat(),
            groups: [self.groups.clone(), self.groups.clone(), self.groups.clone(), self.groups.clone(), self.groups.clone()].concat(),
        }
    }
}

impl Parsable for Row {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, springs) = many0(alt((
            value(Option::None, tag("?")),
            map(Spring::parser, |spring| Option::Some(spring)),
        )))(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, groups) = separated_list0(tag(","), u32)(input)?;

        Ok((input, Row::new(springs, groups)))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Spring {
    Operational,
    Damaged,
}

impl Parsable for Spring {
    fn parser(input: &str) -> IResult<&str, Self> {
        alt((
            value(Spring::Operational, tag(".")),
            value(Spring::Damaged, tag("#")),
        ))(input)
    }
}

fn solve_part1(input: &Input) -> usize {
    input.rows.iter().map(|row| row.combinaisons().len()).sum()
}

fn solve_part2(input: &Input) -> usize {
    input.rows.iter().map(|row| row.unfold().combinaisons().len()).sum()
}

fn main() {
    let input = read(12);
    let parsed = Input::parse(&input).unwrap();

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    fn parsed_input() -> Input {
        Input::new(vec![
            Row::new(vec![None, None, None, Some(Spring::Operational), Some(Spring::Damaged), Some(Spring::Damaged), Some(Spring::Damaged)], vec![1, 1, 3]),
            Row::new(vec![Some(Spring::Operational), None, None, Some(Spring::Operational), Some(Spring::Operational), None, None, Some(Spring::Operational), Some(Spring::Operational), Some(Spring::Operational), None, Some(Spring::Damaged), Some(Spring::Damaged), Some(Spring::Operational)], vec![1, 1, 3]),
            Row::new(vec![None, Some(Spring::Damaged), None, Some(Spring::Damaged), None, Some(Spring::Damaged), None, Some(Spring::Damaged), None, Some(Spring::Damaged), None, Some(Spring::Damaged), None, Some(Spring::Damaged), None], vec![1, 3, 1, 6]),
            Row::new(vec![None, None, None, None, Some(Spring::Operational), Some(Spring::Damaged), Some(Spring::Operational), Some(Spring::Operational), Some(Spring::Operational), Some(Spring::Damaged), Some(Spring::Operational), Some(Spring::Operational), Some(Spring::Operational)], vec![4, 1, 1]),
            Row::new(vec![None, None, None, None, Some(Spring::Operational), Some(Spring::Damaged), Some(Spring::Damaged), Some(Spring::Damaged), Some(Spring::Damaged), Some(Spring::Damaged), Some(Spring::Damaged), Some(Spring::Operational), Some(Spring::Operational), Some(Spring::Damaged), Some(Spring::Damaged), Some(Spring::Damaged), Some(Spring::Damaged), Some(Spring::Damaged), Some(Spring::Operational)], vec![1, 6, 5]),
            Row::new(vec![None, Some(Spring::Damaged), Some(Spring::Damaged), Some(Spring::Damaged), None, None, None, None, None, None, None, None], vec![3, 2, 1]),
        ])
    }
    #[test]
    fn test_parser() {
        assert_eq!(Input::parse(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 21);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 525152);
    }
}

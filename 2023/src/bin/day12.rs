use std::collections::HashMap;
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

    fn is_partial_group_valid(&self, group: &Vec<u32>) -> bool {
        if group.len() > self.groups.len() {
            return false;
        }

        if group[..] != self.groups[..group.len()] {
            return false;
        }

        return true
    }

    fn compute_possible_groups_count(&self, springs: &Vec<Option<Spring>>) -> Vec<(Vec<u32>, usize)> {
        let mut combinaisons = vec![(vec![], 0, 1)];

        for value in springs {
            match value {
                Some(Spring::Damaged) => {
                    combinaisons.iter_mut().for_each(|combinaison| {
                        combinaison.1 += 1;
                    });
                },
                Some(Spring::Operational) => {
                    combinaisons.iter_mut().for_each(|combinaison| {
                        if combinaison.1 > 0 {
                            combinaison.0.push(combinaison.1);
                        }

                        combinaison.1 = 0;
                    });
                },
                None => {
                    let mut operational = combinaisons.clone();
                    let mut damaged = combinaisons.clone();

                    operational.iter_mut().for_each(|combinaison| {
                        if combinaison.1 > 0 {
                            combinaison.0.push(combinaison.1);
                        }

                        combinaison.1 = 0;
                    });
                    damaged.iter_mut().for_each(|combinaison| {
                        combinaison.1 += 1;
                    });

                    combinaisons = [operational, damaged].concat();
                },
            }

            let mut map = HashMap::new();
            for (group, current, count) in combinaisons {
                if !self.is_partial_group_valid(&group) {
                    continue;
                }

                *map.entry((group, current)).or_insert(0) += count;
            }

            combinaisons = map
                .into_iter()
                .map(|((group, current), count)| (group, current, count))
                .collect()
        }

        combinaisons.iter_mut().for_each(|combinaison| {
            if combinaison.1 > 0 {
                combinaison.0.push(combinaison.1);
            }
        });

        combinaisons.into_iter().map(|(combinaison, _, count)| (combinaison, count)).collect()
    }

    fn combinaisons_count(&self) -> usize {
        self.compute_possible_groups_count(&self.springs)
            .into_iter()
            .filter(|(groups, _)| groups == &self.groups)
            .map(|(_, count)| count)
            .sum()
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
    input.rows.iter().map(|row| row.combinaisons_count()).sum()
}

fn solve_part2(input: &Input) -> usize {
    input.rows.iter().map(|row| row.unfold().combinaisons_count()).sum()
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

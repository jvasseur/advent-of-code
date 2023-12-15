use advent_of_code_2023::{read, Parsable};
use nom::{IResult, branch::alt, bytes::complete::tag, character::complete::{u32, alpha1}, combinator::{map, value}, multi::separated_list1, sequence::{terminated, preceded, pair}};

fn hash(value: &str) -> usize {
    let mut hash = 0;

    for char in value.chars() {
        hash += char as usize;
        hash *= 17;
        hash %= 256;
    }

    hash
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    steps: Vec<Step>,
}

impl Input {
    fn new(steps: impl Into<Vec<Step>>) -> Self {
        Self { steps: steps.into() }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        map(terminated(separated_list1(tag(","), Step::parser), tag("\n")), Self::new)(input)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Step {
    label: String,
    operation: Operation,
}

impl Step {
    fn new(label: impl Into<String>, operation: impl Into<Operation>) -> Self {
        Self { label: label.into(), operation: operation.into() }
    }
}

impl Parsable for Step {
    fn parser(input: &str) -> IResult<&str, Self> {
        map(pair(alpha1, Operation::parser), |(label, operation)| Self::new(label, operation))(input)
    }
}

impl std::fmt::Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)?;
        write!(f, "{}", self.operation)?;

        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Operation {
    Remove,
    Set(u32),
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Remove => {
                write!(f, "-")?;
            },
            Operation::Set(focal) => {
                write!(f, "={}", focal)?;
            },
        }

        Ok(())
    }
}

impl Parsable for Operation {
    fn parser(input: &str) -> IResult<&str, Self> {
        alt((
            value(Operation::Remove, tag("-")),
            map(preceded(tag("="), u32), |focal| Operation::Set(focal)),
        ))(input)
    }
}

fn solve_part1(input: &Input) -> usize {
    input.steps.iter().map(|step| hash(&step.to_string())).sum()
}

fn solve_part2(input: &Input) -> usize {
    const EMPTY_BOX: Vec::<(&str, u32)> = Vec::new();

    let mut boxes = [EMPTY_BOX; 256];

    for step in &input.steps {
        let label = &step.label;
        let current_box = &mut boxes[hash(label)];

        match step.operation {
            Operation::Remove => {
                if let Some(position) = current_box.iter().position(|lens| lens.0 == label) {
                    current_box.remove(position);
                }
            },
            Operation::Set(focal) => {
                if let Some(position) = current_box.iter().position(|lens| lens.0 == label) {
                    current_box.get_mut(position).unwrap().1 = focal;
                } else {
                    current_box.push((label, focal));
                }
            },
        }
    }

    boxes.into_iter()
        .enumerate()
        .map(|(box_number, current_box)| current_box.into_iter()
            .enumerate()
            .map(|(lens, (_, focus))| (box_number + 1) * (lens + 1) * (focus as usize))
            .sum::<usize>()
        )
        .sum()
}

fn main() {
    let input = read(15);
    let parsed = Input::parse(&input).unwrap();

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7\n";

    fn parsed_input() -> Input {
        Input::new(vec![
            Step::new("rn", Operation::Set(1)),
            Step::new("cm", Operation::Remove),
            Step::new("qp", Operation::Set(3)),
            Step::new("cm", Operation::Set(2)),
            Step::new("qp", Operation::Remove),
            Step::new("pc", Operation::Set(4)),
            Step::new("ot", Operation::Set(9)),
            Step::new("ab", Operation::Set(5)),
            Step::new("pc", Operation::Remove),
            Step::new("pc", Operation::Set(6)),
            Step::new("ot", Operation::Set(7)),
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(Input::parse(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 1320);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 145);
    }
}

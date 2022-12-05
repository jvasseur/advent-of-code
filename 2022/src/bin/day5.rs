use advent_of_code_2022::{read, parse};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, u32};
use nom::combinator::{map, value};
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, separated_pair, terminated, tuple};

#[derive(Copy, Clone, Debug, PartialEq)]
struct Crate {
    id: char,
}

impl Crate {
    pub fn new(id: char) -> Self {
        Self {
            id,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Stack {
    stack: Vec<Crate>,
}

impl Stack {
    pub fn new(stack: Vec<Crate>) -> Self {
        Self {
            stack,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Stacks {
    stacks: Vec<Stack>,
}

impl Stacks {
    pub fn new(stacks: Vec<Stack>) -> Self {
        Self {
            stacks,
        }
    }

    pub fn apply(&mut self, operation: Move) {
        for _ in 0..operation.count {
            let moving = self.stacks[(operation.from - 1) as usize].stack.pop().unwrap();

            self.stacks[(operation.to - 1) as usize].stack.push(moving);
        }
    }

    pub fn apply_9001(&mut self, operation: Move) {
        let start = self.stacks[(operation.from - 1) as usize].stack.len() - operation.count as usize;
        let mut moving: Vec<Crate> = self.stacks[(operation.from - 1) as usize].stack.drain(start..).collect();

        self.stacks[(operation.to - 1) as usize].stack.append(&mut moving);
    }
}

#[derive(Debug, PartialEq)]
struct Move {
    count: u32,
    from: u32,
    to: u32,
}

impl Move {
    pub fn new(count: u32, from: u32, to: u32) -> Self {
        Self {
            count,
            from,
            to,
        }
    }
}

type Input = (Stacks, Vec<Move>);

fn crate_parse(input: &str) -> IResult<&str, Option<Crate>> {
    alt((
        value(None, tag("   ")),
        map(delimited(tag("["), anychar, tag("]")), |id: char| Some(Crate::new(id))),
    ))(input)
}

fn stacks_parser(input: &str) -> IResult<&str, Stacks> {
    let (input, (crates, indexes)) = tuple((
        many1(
            terminated(
                separated_list1(
                    tag(" "),
                    crate_parse,
                ),
                tag("\n"),
            ),
        ),
        terminated(
            separated_list1(
                tag(" "),
                delimited(
                    tag(" "),
                    u32,
                    tag(" "),
                ),
            ),
            tag("\n"),
        )
    ))(input)?;

    let mut stackes = Vec::new();

    for (i, _) in indexes.iter().enumerate() {
        let mut stack = Vec::new();

        for (j, _) in crates.iter().enumerate().rev() {
            if let Some(current_crate) = crates[j][i] {
                stack.push(current_crate);
            }
        }

        stackes.push(Stack::new(stack));
    }

    Ok((input, Stacks::new(stackes)))
}

fn move_parser(input: &str) -> IResult<&str, Move> {
    map(tuple((
        tag("move "),
        u32,
        tag(" from "),
        u32,
        tag(" to "),
        u32,
        tag("\n"),
    )), |(_, count, _, from, _, to, _)| Move::new(count, from, to))(input)
}

fn moves_parser(input: &str) -> IResult<&str, Vec<Move>> {
    many1(move_parser)(input)
}

fn parser(input: &str) -> IResult<&str, Input> {
    separated_pair(stacks_parser, tag("\n"), moves_parser)(input)
}

fn solve_part1((mut stacks, moves): Input) -> String {
    for operation in moves {
        stacks.apply(operation);
    }

    stacks.stacks.iter().map(|stack| stack.stack.last().unwrap().id).collect()
}

fn solve_part2((mut stacks, moves): Input) -> String {
    for operation in moves {
        stacks.apply_9001(operation);
    }

    stacks.stacks.iter().map(|stack| stack.stack.last().unwrap().id).collect()
}

fn main() {
    let input = read(5);

    println!("{}", solve_part1(parse(parser, &input)));
    println!("{}", solve_part2(parse(parser, &input)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(parser("    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2\n"), Ok(("", (
            Stacks::new(vec![
                Stack::new(vec![
                    Crate::new('Z'),
                    Crate::new('N'),
                ]),
                Stack::new(vec![
                    Crate::new('M'),
                    Crate::new('C'),
                    Crate::new('D'),
                ]),
                Stack::new(vec![
                    Crate::new('P'),
                ]),
            ]),
            vec![
                Move::new(1, 2, 1),
                Move::new(3, 1, 3),
                Move::new(2, 2, 1),
                Move::new(1, 1, 2),
            ],
        ))));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1((
            Stacks::new(vec![
                Stack::new(vec![
                    Crate::new('Z'),
                    Crate::new('N'),
                ]),
                Stack::new(vec![
                    Crate::new('M'),
                    Crate::new('C'),
                    Crate::new('D'),
                ]),
                Stack::new(vec![
                    Crate::new('P'),
                ]),
            ]),
            vec![
                Move::new(1, 2, 1),
                Move::new(3, 1, 3),
                Move::new(2, 2, 1),
                Move::new(1, 1, 2),
            ],
        )), "CMZ");
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2((
            Stacks::new(vec![
                Stack::new(vec![
                    Crate::new('Z'),
                    Crate::new('N'),
                ]),
                Stack::new(vec![
                    Crate::new('M'),
                    Crate::new('C'),
                    Crate::new('D'),
                ]),
                Stack::new(vec![
                    Crate::new('P'),
                ]),
            ]),
            vec![
                Move::new(1, 2, 1),
                Move::new(3, 1, 3),
                Move::new(2, 2, 1),
                Move::new(1, 1, 2),
            ],
        )), "MCD");
    }
}

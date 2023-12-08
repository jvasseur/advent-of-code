use std::collections::HashMap;
use advent_of_code_2023::{read, Parsable};
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::branch::alt;
use nom::IResult;
use nom::combinator::value;
use nom::multi::{fill, many0};
use nom::sequence::{terminated, separated_pair, delimited};

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    instructions: Vec<Instruction>,
    nodes: HashMap<Node, (Node, Node)>,
}

impl Input {
    fn new(instructions: impl Into<Vec<Instruction>>, nodes: impl Into<HashMap<Node, (Node, Node)>>) -> Self {
        Input {
            instructions: instructions.into(),
            nodes: nodes.into(),
        }
    }

    fn travel(&self, start: Node, is_end: impl Fn(&Node) -> bool )-> u64 {
        let mut steps = 0;
        let mut position = start;

        for instruction in self.instructions.iter().cycle() {
            let &(left, right) = self.nodes.get(&position).unwrap();

            steps += 1;
            position = match instruction {
                Instruction::Left => left,
                Instruction::Right => right,
            };

            if is_end(&position) {
                return steps;
            }
        }

        panic!("To infinity and beyond!")
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, instructions) = many0(Instruction::parser)(input)?;
        let (input, _) = tag("\n\n")(input)?;
        let (input, nodes) = many0(terminated(separated_pair(
            Node::parser,
            tag(" = "),
            delimited(
                tag("("),
                separated_pair(
                    Node::parser,
                    tag(", "),
                    Node::parser,
                ),
                tag(")"),
            ),
        ), tag("\n")))(input)?;

        Ok((input, Input::new(instructions, nodes.into_iter().collect::<HashMap<Node, (Node, Node)>>())))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Instruction {
    Left,
    Right,
}

impl Parsable for Instruction {
    fn parser(input: &str) -> IResult<&str, Self> {
        alt((
            value(Instruction::Left, tag("L")),
            value(Instruction::Right, tag("R")),
        ))(input)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Node {
    id: [char; 3]
}

impl Node {
    const fn new(id: [char; 3]) -> Self {
        Node { id }
    }

    fn is_end(&self) -> bool {
        self.id == ['Z', 'Z', 'Z']
    }

    fn is_ghost_start(&self) -> bool {
        self.id[2] == 'A'
    }

    fn is_ghost_end(&self) -> bool {
        self.id[2] == 'Z'
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.id[0].fmt(f)?;
        self.id[1].fmt(f)?;
        self.id[2].fmt(f)?;

        Ok(())
    }
}

impl Parsable for Node {
    fn parser(input: &str) -> IResult<&str, Self> {
        let mut id = ['A'; 3];

        let (input, ()) = fill(anychar, &mut id)(input)?;

        Ok((input, Node { id }))
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;

    while a != b {
        if a < b {
            (a, b) = (b, a);
        }

        (a, b) = (a - b, b);
    }

    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

fn solve_part1(input: &Input) -> u64 {
    input.travel(Node::new(['A', 'A', 'A']), Node::is_end)
}

fn solve_part2(input: &Input) -> u64 {
    input.nodes.keys()
        .cloned()
        .filter(Node::is_ghost_start)
        .map(|start| input.travel(start, Node::is_ghost_end))
        .reduce(lcm)
        .unwrap()
}

fn main() {
    let input = read(8);

    let parsed = Input::parse(&input).unwrap();

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

    fn parsed_input_1() -> Input {
        Input::new(
            [Instruction::Right, Instruction::Left],
            [
                (Node::new(['A', 'A', 'A']), (Node::new(['B', 'B', 'B']), Node::new(['C', 'C', 'C']))),
                (Node::new(['B', 'B', 'B']), (Node::new(['D', 'D', 'D']), Node::new(['E', 'E', 'E']))),
                (Node::new(['C', 'C', 'C']), (Node::new(['Z', 'Z', 'Z']), Node::new(['G', 'G', 'G']))),
                (Node::new(['D', 'D', 'D']), (Node::new(['D', 'D', 'D']), Node::new(['D', 'D', 'D']))),
                (Node::new(['E', 'E', 'E']), (Node::new(['E', 'E', 'E']), Node::new(['E', 'E', 'E']))),
                (Node::new(['G', 'G', 'G']), (Node::new(['G', 'G', 'G']), Node::new(['G', 'G', 'G']))),
                (Node::new(['Z', 'Z', 'Z']), (Node::new(['Z', 'Z', 'Z']), Node::new(['Z', 'Z', 'Z']))),
            ]
        )
    }

    fn parsed_input_2() -> Input {
        Input::new(
            [Instruction::Left, Instruction::Right],
            [
                (Node::new(['1', '1', 'A']), (Node::new(['1', '1', 'B']), Node::new(['X', 'X', 'X']))),
                (Node::new(['1', '1', 'B']), (Node::new(['X', 'X', 'X']), Node::new(['1', '1', 'Z']))),
                (Node::new(['1', '1', 'Z']), (Node::new(['1', '1', 'B']), Node::new(['X', 'X', 'X']))),
                (Node::new(['2', '2', 'A']), (Node::new(['2', '2', 'B']), Node::new(['X', 'X', 'X']))),
                (Node::new(['2', '2', 'B']), (Node::new(['2', '2', 'C']), Node::new(['2', '2', 'C']))),
                (Node::new(['2', '2', 'C']), (Node::new(['2', '2', 'Z']), Node::new(['2', '2', 'Z']))),
                (Node::new(['2', '2', 'Z']), (Node::new(['2', '2', 'B']), Node::new(['2', '2', 'B']))),
                (Node::new(['X', 'X', 'X']), (Node::new(['Z', 'Z', 'Z']), Node::new(['Z', 'Z', 'Z']))),
            ]
        )
    }

    #[test]
    fn test_parser() {
        assert_eq!(Input::parse(INPUT), Ok(parsed_input_1()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input_1()), 2);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input_2()), 6);
    }
}

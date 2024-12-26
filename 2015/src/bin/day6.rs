use std::ops::RangeInclusive;
use advent_of_code_2015::{parser::*, read};
use nom::{branch::alt, bytes::complete::tag, character::complete::{char, u16}, combinator::{map, value}, multi::many1, sequence::terminated};

#[derive(Clone, PartialEq, Eq, Debug)]
enum InstructionKind {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Instruction {
    kind: InstructionKind,
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
}

impl Parsable for Instruction {
    fn parser<'a>(input: &'a str) -> ParserResult<'a, Self> {
        let (input, kind) = alt((
            value(InstructionKind::TurnOn, tag("turn on")),
            value(InstructionKind::TurnOff, tag("turn off")),
            value(InstructionKind::Toggle, tag("toggle")),
        ))(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, x_min) = u16(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, y_min) = u16(input)?;
        let (input, _) = tag(" through ")(input)?;
        let (input, x_max) = u16(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, y_max) = u16(input)?;

        Ok((input, Instruction { kind, x: (x_min as usize)..=(x_max as usize), y: (y_min as usize)..=(y_max as usize) }))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    instructions: Vec<Instruction>,
}

impl Input {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self { instructions }
    }
}

impl Parsable for Input {
    fn parser<'a>(input: &'a str) -> ParserResult<'a, Self> {
        map(many1(terminated(Instruction::parser, char('\n'))), Input::new)(input)
    }
}

fn solve_part1(input: &Input) -> usize {
    let mut grid = [[false; 1000]; 1000];

    for instruction in &input.instructions {
        for x in instruction.x.clone() {
            for y in instruction.y.clone() {
                grid[x][y] = match instruction.kind {
                    InstructionKind::TurnOn => true,
                    InstructionKind::TurnOff => false,
                    InstructionKind::Toggle => !grid[x][y],
                }
            }
        }
    }

    grid.into_iter().map(|row| row.into_iter().filter(|value| *value).count()).sum()
}

fn solve_part2(input: &Input) -> u32 {
    // Using a 1000*1000 array leads to stack overflows so we use a vec instead
    let mut grid = Vec::new();
    grid.resize(1000, [0; 1000]);

    for instruction in &input.instructions {
        for x in instruction.x.clone() {
            for y in instruction.y.clone() {
                grid[x][y] = match instruction.kind {
                    InstructionKind::TurnOn => grid[x][y] + 1,
                    InstructionKind::TurnOff => if grid[x][y] > 0 { grid[x][y] - 1 } else { grid[x][y] },
                    InstructionKind::Toggle => grid[x][y] + 2,
                }
            }
        }
    }

    grid.into_iter().map(|row| row.into_iter().sum::<u32>()).sum()
}

fn main() {
    let input = parse(&read(6).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

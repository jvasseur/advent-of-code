use advent_of_code_2024::{parser::*, read};
use itertools::Itertools;
use nom::{bytes::complete::tag, multi::separated_list1, IResult};

type Value = u64;

#[derive(Clone, PartialEq, Eq, Debug)]
enum Instruction {
    Adv(u8),
    Bxl(Value),
    Bst(u8),
    Jnz(usize),
    Bxc,
    Out(u8),
    Bdv(u8),
    Cdv(u8),
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    registry_a: Value,
    registry_b: Value,
    registry_c: Value,

    program: Vec<u8>,
}

impl Input {
    fn new(registry_a: Value, registry_b: Value, registry_c: Value, program: Vec<u8>) -> Self {
        Self { registry_a, registry_b, registry_c, program }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("Register A: ")(input)?;
        let (input, registry_a) = Value::parser(input)?;
        let (input, _) = tag("\n")(input)?;

        let (input, _) = tag("Register B: ")(input)?;
        let (input, registry_b) = Value::parser(input)?;
        let (input, _) = tag("\n")(input)?;

        let (input, _) = tag("Register C: ")(input)?;
        let (input, registry_c) = Value::parser(input)?;
        let (input, _) = tag("\n")(input)?;

        let (input, _) = tag("\n")(input)?;

        let (input, _) = tag("Program: ")(input)?;
        let (input, program) = separated_list1(tag(","), u8::parser)(input)?;
        let (input, _) = tag("\n")(input)?;

        Ok((input, Input::new(registry_a, registry_b, registry_c, program)))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Device {
    registry_a: Value,
    registry_b: Value,
    registry_c: Value,

    pointer: usize,

    program: Vec<Instruction>,
}

impl Device  {
    fn execute_instruction(&mut self, pointer: usize) -> (usize, Option<Value>) {
        let mut next_pointer = pointer + 1;
        let mut output = None;

        match self.program[pointer] {
            // 0
            Instruction::Adv(operand) => {
                self.registry_a = self.registry_a >> self.combo(operand);
            },
            // 1
            Instruction::Bxl(operand) => {
                self.registry_b = self.registry_b ^ operand;
            },
            // 2
            Instruction::Bst(operand) => {
                self.registry_b = self.combo(operand) % 8;
            },
            // 3
            Instruction::Jnz(operand) => {
                if self.registry_a != 0 {
                    next_pointer = operand;
                }
            },
            // 4
            Instruction::Bxc => {
                self.registry_b = self.registry_b ^ self.registry_c;
            },
            // 5
            Instruction::Out(operand) => {
                output = Some(self.combo(operand) % 8);
            },
            // 6
            Instruction::Bdv(operand) => {
                self.registry_b = self.registry_a >> self.combo(operand);
            },
            // 7
            Instruction::Cdv(operand) => {
                self.registry_c = self.registry_a >> self.combo(operand);
            },
        }

        (next_pointer, output)
    }

    fn combo(&self, operand: u8) -> Value {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.registry_a,
            5 => self.registry_b,
            6 => self.registry_c,
            7 => panic!("Reserved"),
            _ => panic!("Invalid operand"),
        }
    }
}

impl From<&Input> for Device {
    fn from(value: &Input) -> Self {
        let program = value.program.iter().tuples().map(|(instruction, operand)| match instruction {
            0 => Instruction::Adv(*operand),
            1 => Instruction::Bxl(*operand as Value),
            2 => Instruction::Bst(*operand),
            3 => Instruction::Jnz((*operand / 2) as usize),
            4 => Instruction::Bxc,
            5 => Instruction::Out(*operand),
            6 => Instruction::Bdv(*operand),
            7 => Instruction::Cdv(*operand),
            _ => panic!("Invalid instruction"),
        }).collect();

        Self {
            registry_a: value.registry_a,
            registry_b: value.registry_b,
            registry_c: value.registry_c,

            pointer: 0,

            program,
        }
    }
}

impl Iterator for Device {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if (self.pointer as usize) >= self.program.len() {
                return None;
            }

            let (next_pointer, output) = self.execute_instruction(self.pointer);

            self.pointer = next_pointer;

            if let Some(output) = output {
                return Some(output);
            }
        }
    }
}

fn solve_part1(input: &Input) -> Vec<Value> {
    Device::from(input).collect()
}

fn solve_part2(input: &Input) -> Value {
    let program = input.program.iter().map(|i| *i as u64).collect::<Vec<_>>();

    let mut possible_results = vec![0];

    for i in (0..program.len()).rev() {
        let mut next_possible_results = Vec::new();

        for possible_result in possible_results {
            for j in 0..8 {
                let tentative_result = possible_result | (j << (3 * i));

                let mut device = Device::from(input);

                device.registry_a = tentative_result >> (3 * i);

                if let Some(output) = device.next() {
                    if output == program[i] {
                        next_possible_results.push(tentative_result);
                    }
                }
            }
        }

        possible_results = next_possible_results;
    }

    possible_results[0]
}

fn main() {
    let input = parse(&read(17).unwrap()).unwrap();

    println!("{}", solve_part1(&input).into_iter().map(|o| o.to_string()).join(","));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    fn parsed_input() -> Input {
        Input::new(729, 0, 0, vec![0, 1, 5, 4, 3, 0])
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse::<Input>(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }


    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&Input::new(2024, 0, 0, vec![0, 3, 5, 4, 3, 0])), 117440);
    }
}

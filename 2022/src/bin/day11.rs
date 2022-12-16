use advent_of_code_2022::{read, parse};
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{u8, u64};
use nom::combinator::{map, value};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test_divisible_by: u64,
    test_on_true: usize,
    test_on_false: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Operation {
    operator: Operator,
    operand: Operand,
}

impl Operation {
    pub fn apply(&self, left: u64) -> u64 {
        let right = match self.operand {
            Operand::Old => left,
            Operand::Fixed(value) => value,
        };

        match self.operator {
            Operator::Add => left + right,
            Operator::Times => left * right,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Operator {
    Add,
    Times,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Operand {
    Old,
    Fixed(u64),
}

fn parser(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(
        tag("\n"),
        map(tuple((
            tag("Monkey "),
            u8,
            tag(":\n  Starting items: "),
            separated_list1(tag(", "), u64),
            tag("\n  Operation: new = old "),
            alt((
                value(Operator::Add, tag("+")),
                value(Operator::Times, tag("*")),
            )),
            tag(" "),
            alt((
                map(tag("old"), |_| Operand::Old),
                map(u64, |v| Operand::Fixed(v)),
            )),
            tag("\n  Test: divisible by "),
            u64,
            tag("\n    If true: throw to monkey "),
            map(u8, |v| v as usize),
            tag("\n    If false: throw to monkey "),
            map(u8, |v| v as usize),
            tag("\n")
        )), |(_, _, _, items, _, operator, _, operand, _, test_divisible_by, _, test_on_true, _, test_on_false, _)| Monkey {
            items: VecDeque::from(items),
            operation: Operation {
                operator,
                operand,
            },
            test_divisible_by,
            test_on_true,
            test_on_false,
        }),
    )(input)
}

fn solve_part1(input: &[Monkey]) -> u32 {
    let mut throws: Vec<u32> = input.iter().map(|_| 0).collect();
    let monkeys: Vec<RefCell<Monkey>> = input.iter().map(|monkey| RefCell::new(monkey.clone())).collect();

    for _ in 0..20 {
        for (i, monkey) in monkeys.iter().enumerate() {
            let operation = monkey.borrow().operation;
            let test_divisible_by = monkey.borrow().test_divisible_by;
            let test_on_true = monkey.borrow().test_on_true;
            let test_on_false = monkey.borrow().test_on_false;

            while let Some(item) = monkey.borrow_mut().items.pop_front() {
                let new_wory = operation.apply(item);
                let new_wory = new_wory / 3;
                let test = new_wory.rem_euclid(test_divisible_by) == 0;

                let monkey_to_throw = if test { test_on_true } else { test_on_false };

                monkeys[monkey_to_throw].borrow_mut().items.push_back(new_wory);

                throws[i] += 1;
            }
        }
    }

    throws.iter().sorted().rev().take(2).product()
}

fn solve_part2(input: &[Monkey]) -> u64 {
    let mut throws: Vec<u64> = input.iter().map(|_| 0).collect();
    let monkeys: Vec<RefCell<Monkey>> = input.iter().map(|monkey| RefCell::new(monkey.clone())).collect();
    let remnant: u64 = input.iter().map(|monkey| monkey.test_divisible_by).product();

    for _ in 0..10_000 {
        for (i, monkey) in monkeys.iter().enumerate() {
            let operation = monkey.borrow().operation;
            let test_divisible_by = monkey.borrow().test_divisible_by;
            let test_on_true = monkey.borrow().test_on_true;
            let test_on_false = monkey.borrow().test_on_false;

            while let Some(item) = monkey.borrow_mut().items.pop_front() {
                let new_wory = operation.apply(item);
                let new_wory = new_wory % remnant;
                let test = new_wory.rem_euclid(test_divisible_by) == 0;

                let monkey_to_throw = if test { test_on_true } else { test_on_false };

                monkeys[monkey_to_throw].borrow_mut().items.push_back(new_wory);

                throws[i] += 1;
            }
        }
    }

    throws.iter().sorted().rev().take(2).product()
}

fn main() {
    let input = read(11);

    let parsed = parse(parser, &input);

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn test_parser() {
        assert_eq!(parser(INPUT), Ok(("", vec![
            Monkey {
                items: VecDeque::from(vec![79, 98]),
                operation: Operation {
                    operator: Operator::Times,
                    operand: Operand::Fixed(19),
                },
                test_divisible_by: 23,
                test_on_true: 2,
                test_on_false: 3,
            },
            Monkey {
                items: VecDeque::from(vec![54, 65, 75, 74]),
                operation: Operation {
                    operator: Operator::Add,
                    operand: Operand::Fixed(6),
                },
                test_divisible_by: 19,
                test_on_true: 2,
                test_on_false: 0,
            },
            Monkey {
                items: VecDeque::from(vec![79, 60, 97]),
                operation: Operation {
                    operator: Operator::Times,
                    operand: Operand::Old,
                },
                test_divisible_by: 13,
                test_on_true: 1,
                test_on_false: 3,
            },
            Monkey {
                items: VecDeque::from(vec![74]),
                operation: Operation {
                    operator: Operator::Add,
                    operand: Operand::Fixed(3),
                },
                test_divisible_by: 17,
                test_on_true: 0,
                test_on_false: 1,
            },
        ])));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&vec![
            Monkey {
                items: VecDeque::from(vec![79, 98]),
                operation: Operation {
                    operator: Operator::Times,
                    operand: Operand::Fixed(19),
                },
                test_divisible_by: 23,
                test_on_true: 2,
                test_on_false: 3,
            },
            Monkey {
                items: VecDeque::from(vec![54, 65, 75, 74]),
                operation: Operation {
                    operator: Operator::Add,
                    operand: Operand::Fixed(6),
                },
                test_divisible_by: 19,
                test_on_true: 2,
                test_on_false: 0,
            },
            Monkey {
                items: VecDeque::from(vec![79, 60, 97]),
                operation: Operation {
                    operator: Operator::Times,
                    operand: Operand::Old,
                },
                test_divisible_by: 13,
                test_on_true: 1,
                test_on_false: 3,
            },
            Monkey {
                items: VecDeque::from(vec![74]),
                operation: Operation {
                    operator: Operator::Add,
                    operand: Operand::Fixed(3),
                },
                test_divisible_by: 17,
                test_on_true: 0,
                test_on_false: 1,
            },
        ]), 10605);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&vec![
            Monkey {
                items: VecDeque::from(vec![79, 98]),
                operation: Operation {
                    operator: Operator::Times,
                    operand: Operand::Fixed(19),
                },
                test_divisible_by: 23,
                test_on_true: 2,
                test_on_false: 3,
            },
            Monkey {
                items: VecDeque::from(vec![54, 65, 75, 74]),
                operation: Operation {
                    operator: Operator::Add,
                    operand: Operand::Fixed(6),
                },
                test_divisible_by: 19,
                test_on_true: 2,
                test_on_false: 0,
            },
            Monkey {
                items: VecDeque::from(vec![79, 60, 97]),
                operation: Operation {
                    operator: Operator::Times,
                    operand: Operand::Old,
                },
                test_divisible_by: 13,
                test_on_true: 1,
                test_on_false: 3,
            },
            Monkey {
                items: VecDeque::from(vec![74]),
                operation: Operation {
                    operator: Operator::Add,
                    operand: Operand::Fixed(3),
                },
                test_divisible_by: 17,
                test_on_true: 0,
                test_on_false: 1,
            },
        ]), 2713310158);
    }
}

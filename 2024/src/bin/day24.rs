use std::{collections::HashMap, convert::identity, fmt};

use advent_of_code_2024::{parser::*, read};
use itertools::Itertools;
use nom::{branch::alt, bytes::complete::tag, character::complete::{alphanumeric1, u8}, combinator::map, multi::many1, sequence::tuple, IResult};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
struct Id(String);

impl Id {
    fn x(i: usize) -> Self {
        Self(format!("x{:02}", i))
    }

    fn y(i: usize) -> Self {
        Self(format!("y{:02}", i))
    }

    fn z(i: usize) -> Self {
        Self(format!("z{:02}", i))
    }
}

impl Parsable for Id {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, value) = alphanumeric1(input)?;

        Ok((input, Id(value.to_owned())))
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Eq, Debug)]
enum Gate {
    Value(bool),
    And(Id, Id),
    Or(Id, Id),
    Xor(Id, Id),
}

impl PartialEq<Gate> for Gate {
    fn eq(&self, other: &Gate) -> bool {
        match (self, other) {
            (Gate::Value(a), Gate::Value(b)) => a == b,
            (Gate::And(a1, a2), Gate::And(b1, b2)) => (a1 == b1 && a2 == b2) || (a1 == b2 && a2 == b1),
            (Gate::Or(a1, a2), Gate::Or(b1, b2)) => (a1 == b1 && a2 == b2) || (a1 == b2 && a2 == b1),
            (Gate::Xor(a1, a2), Gate::Xor(b1, b2)) => (a1 == b1 && a2 == b2) || (a1 == b2 && a2 == b1),
            _ => false,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    gates: HashMap<Id, Gate>,
}

impl Input {
    fn find(&self, gate: &Gate) -> Option<Id> {
        self.find_by(|v| v == gate)
    }

    fn find_by(&self, predicate: impl Fn(&Gate) -> bool) -> Option<Id> {
        self.gates.iter()
            .find(|(_, gate)| predicate(gate))
            .map(|(id, _)| id.to_owned())
    }

    fn swap(&mut self, a: &Id, b: &Id) {
        let (a, a_gate) = self.gates.remove_entry(a).unwrap();
        let (b, b_gate) = self.gates.remove_entry(b).unwrap();

        self.gates.insert(a, b_gate);
        self.gates.insert(b, a_gate);
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, gates) = many1(alt((
            map(tag("\n"), |_| None),
            map(tuple((Id::parser, tag(": "), u8, tag("\n"))), |(id, _, value, _)| Some((id, Gate::Value(value == 1)))),
            map(tuple((Id::parser, tag(" AND "), Id::parser, tag(" -> "), Id::parser, tag("\n"))), |(left, _, right, _, id, _)| Some((id, Gate::And(left, right)))),
            map(tuple((Id::parser, tag(" OR "), Id::parser, tag(" -> "), Id::parser, tag("\n"))), |(left, _, right, _, id, _)| Some((id, Gate::Or(left, right)))),
            map(tuple((Id::parser, tag(" XOR "), Id::parser, tag(" -> "), Id::parser, tag("\n"))), |(left, _, right, _, id, _)| Some((id, Gate::Xor(left, right)))),
        )))(input)?;

        Ok((input, Input {
            gates: gates.into_iter().filter_map(identity).collect(),
        }))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Circuit {
    gates: HashMap<Id, Gate>,
    cache: HashMap<Id, bool>,
}

impl Circuit {
    fn get(&mut self, id: &Id) -> Option<bool> {
        if let Some(&value) = self.cache.get(id) {
            return Some(value);
        }

        let result = match self.gates.get(id).cloned() {
            None => {
                return None;
            },
            Some(Gate::Value(value)) => value,
            Some(Gate::And(left, right)) => self.get(&left).unwrap() & self.get(&right).unwrap(),
            Some(Gate::Or(left, right)) => self.get(&left).unwrap() | self.get(&right).unwrap(),
            Some(Gate::Xor(left, right)) => self.get(&left).unwrap() ^ self.get(&right).unwrap(),
        };

        self.cache.insert(id.to_owned(), result);

        return Some(result);
    }

    fn get_z(&mut self) -> u64 {
        let mut value = 0;

        for i in 0..100 {
            if let Some(bit) = self.get(&Id::z(i)) {
                if bit {
                    value += 1 << i;
                }
            } else {
                break;
            }
        }

        value
    }
}

impl From<&Input> for Circuit {
    fn from(value: &Input) -> Self {
        Self { gates: value.gates.to_owned(), cache: HashMap::new() }
    }
}

fn solve_part1(input: &Input) -> u64 {
    Circuit::from(input).get_z()
}

fn solve_part2(input: &Input) -> String {
    let mut input = input.clone();
    let mut swaps = Vec::new();

    'check: loop {
        // rem(O) is a special case
        let mut prev_rem = input
            .find(&Gate::And(Id::x(0), Id::y(0)))
            .expect("rem(0) not found");

        for i in 1..=44 {
            let xor = input
                .find(&Gate::Xor(Id::x(i), Id::y(i)))
                .expect(&format!("xor({}) not found", i));

            let and = input
                .find(&Gate::And(Id::x(i), Id::y(i)))
                .expect(&format!("and({}) not found", i));

            // Check that z(i) has the right opperands
            if let Some(Gate::Xor(a, b)) = input.gates.get(&Id::z(i)).cloned() {
                if a == xor && b != prev_rem {
                    input.swap(&b, &prev_rem);
                    swaps.push((b, prev_rem));

                    // Restart check with fixed input
                    continue 'check;
                }

                if a != xor && b == prev_rem {
                    input.swap(&a, &xor);
                    swaps.push((a, xor));

                    // Restart check with fixed input
                    continue 'check;
                }

                if b == xor && a != prev_rem {
                    input.swap(&a, &prev_rem);
                    swaps.push((a, prev_rem));

                    // Restart check with fixed input
                    continue 'check;
                }

                if b != xor && a == prev_rem {
                    input.swap(&b, &xor);
                    swaps.push((b, xor));

                    // Restart check with fixed input
                    continue 'check;
                }
            }

            // Rem
            let xor_and_prev_rem = input
                .find(&Gate::And(xor, prev_rem))
                .expect(&format!("xor_and_prev_rem{} not found", i));

            let rem = input.find(&Gate::Or(and.clone(), xor_and_prev_rem.clone()));

            if rem.is_none() {
                // We didn't find rem(i), this means either and(i) or xor_and_prev_rem(i) need to be swapped
                if let Some(swap) = input
                    .gates
                    .values()
                    .find_map(|gate| match gate {
                        Gate::Or(a, b) => {
                            if a == &and {
                                return Some(b.clone());
                            }

                            if b == &and {
                                return Some(a.clone());
                            }

                            return None;
                        },
                        _ => None,
                    }) {
                    input.swap(&xor_and_prev_rem, &swap);
                    swaps.push((xor_and_prev_rem, swap));

                    // Restart check with fixed input
                    continue 'check;
                }

                if let Some(swap) = input
                    .gates
                    .values()
                    .find_map(|gate| match gate {
                        Gate::Or(a, b) => {
                            if a == &xor_and_prev_rem {
                                return Some(b.clone());
                            }

                            if b == &xor_and_prev_rem {
                                return Some(a.clone());
                            }

                            return None;
                        },
                        _ => None,
                    }) {
                    input.swap(&and, &swap);
                    swaps.push((and, swap));

                    // Restart check with fixed input
                    continue 'check;
                }
            }

            prev_rem = rem.unwrap();
        }

        break 'check;
    }

    swaps
        .into_iter()
        .map(|(a, b)| [a, b])
        .flatten()
        .sorted()
        .join(",")
}

fn main() {
    let input = parse(&read(24).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
";

    fn parsed_input() -> Input {
        Input { gates: HashMap::from([
            (Id("x00".to_owned()), Gate::Value(true)),
            (Id("x01".to_owned()), Gate::Value(true)),
            (Id("x02".to_owned()), Gate::Value(true)),
            (Id("y00".to_owned()), Gate::Value(false)),
            (Id("y01".to_owned()), Gate::Value(true)),
            (Id("y02".to_owned()), Gate::Value(false)),
            (Id("z00".to_owned()), Gate::And(Id("x00".to_owned()), Id("y00".to_owned()))),
            (Id("z01".to_owned()), Gate::Xor(Id("x01".to_owned()), Id("y01".to_owned()))),
            (Id("z02".to_owned()), Gate::Or(Id("x02".to_owned()), Id("y02".to_owned()))),
        ])}
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse::<Input>(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parse::<Input>("x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
").unwrap()), 2024);
    }
}

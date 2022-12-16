#![feature(int_abs_diff)]

use advent_of_code_2022::{read, parse};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, u8};
use nom::combinator::map;
use nom::IResult;
use nom::multi::{separated_list1, many0};
use nom::sequence::{preceded, terminated};
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
struct Valve<'a> {
    flow_rate: u8,
    to: Vec<&'a str>,
}

impl<'a> Valve<'a> {
    pub fn new(flow_rate: u8, to: Vec<&'a str>) -> Self {
        Self {
            flow_rate,
            to,
        }
    }
}

type Input<'a> = HashMap<&'a str, Valve<'a>>;

fn valve_parser(input: &str) -> IResult<&str, (&str, Valve)> {
    let (input, _) = tag("Valve ")(input)?;
    let (input, id) = alpha1(input)?;
    let (input, _) = tag(" has flow rate=")(input)?;
    let (input, flow_rate) = u8(input)?;
    let (input, _) = tag("; ")(input)?;

    let (input, to) = alt((
        preceded(tag("tunnel leads to valve "), map(alpha1, |s| vec![s])),
        preceded(tag("tunnels lead to valves "), separated_list1(tag(", "), alpha1)),
    ))(input)?;

    Ok((input, (id, Valve {
        flow_rate,
        to,
    })))
}

fn parser(input: &str) -> IResult<&str, Input> {
    let (input, pairs) = many0(terminated(valve_parser, tag("\n")))(input)?;

    Ok((input, HashMap::from_iter(pairs)))
}

#[derive(Debug)]
struct World {
    valves: HashMap<String, u8>,
    distances: HashMap<String, HashMap<String, u8>>,
}

impl<'a> From<&Input<'a>> for World {
    fn from(input: &Input) -> Self {
        let valves: HashMap<String, u8> = input
            .iter()
            .map(|(id, valve)| ((*id).to_owned(), valve.flow_rate))
            .collect();

        let nodes: Vec<String> = input.keys().map(|k| (*k).to_owned()).collect();
        let mut distances: HashMap<String, HashMap<String, u8>> = HashMap::new();

        for (id, valve) in input.iter() {
            let id = (*id).to_owned();

            let mut map: HashMap<String, u8> = valve.to.iter().map(|id| ((*id).to_owned(), 1)).collect();

            map.insert(id.clone(), 0);

            distances.insert(id, map);
        }

        for k in nodes.iter() {
            for i in nodes.iter() {
                for j in nodes.iter() {
                    if let (Some(dist_i_k), Some(dist_k_j)) = (distances.get(i).unwrap().get(k), distances.get(k).unwrap().get(j)) {
                        let sum = dist_i_k + dist_k_j;
                        if let Some(dist_i_j) = distances.get(i).unwrap().get(j) {
                            if *dist_i_j > sum {
                                distances.get_mut(i).unwrap().insert(j.to_owned(), sum);
                            }
                        } else {
                            distances.get_mut(i).unwrap().insert(j.to_owned(), sum);
                        }
                    }
                }
            }
        }

        Self {
            valves,
            distances,
        }
    }
}

fn compute_flow(world: &World, open_valves: &HashSet<&str>, reamining_time: u8, valve_id: &str) -> u32 {
    if reamining_time == 0 {
        return 0;
    }

    let mut reamining_time = reamining_time;
    let mut flow: u32 = 0;
    let mut open_valves = open_valves.clone();

    let flow_rate = *world.valves.get(valve_id).unwrap();

    if flow_rate != 0 && !open_valves.contains(valve_id) {
        reamining_time -= 1;

        flow += flow_rate as u32 * reamining_time as u32;

        open_valves.insert(valve_id);
    }

    flow += world
        .distances
        .get(valve_id)
        .unwrap()
        .iter()
        .filter(|(to, distance)| {
            let to: String = (*to).to_owned();

            if to == valve_id {
                return false;
            }

            if *world.valves.get(&to).unwrap() == 0 {
                return false;
            }

            if open_valves.contains(&to as &str) {
                return false;
            }

            if **distance > reamining_time {
                return false;
            }

            return true;
        })
        .map(|(id, distance)| compute_flow(world, &open_valves, reamining_time - distance, id))
        .max()
        .unwrap_or(0);

    flow
}

fn solve_part1(input: &Input) -> u32 {
    let world = World::from(input);

    compute_flow(&world, &HashSet::new(), 30, "AA")
}

fn solve_part2(input: &Input) -> u32 {
    0
}

fn main() {
    let input = read(16);

    let parsed = parse(parser, &input);

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    fn parsed_input() -> Input<'static> {
        HashMap::from([
            ("AA", Valve::new(0, vec!["DD", "II", "BB"])),
            ("BB", Valve::new(13, vec!["CC", "AA"])),
            ("CC", Valve::new(2, vec!["DD", "BB"])),
            ("DD", Valve::new(20, vec!["CC", "AA", "EE"])),
            ("EE", Valve::new(3, vec!["FF", "DD"])),
            ("FF", Valve::new(0, vec!["EE", "GG"])),
            ("GG", Valve::new(0, vec!["FF", "HH"])),
            ("HH", Valve::new(22, vec!["GG"])),
            ("II", Valve::new(0, vec!["AA", "JJ"])),
            ("JJ", Valve::new(21, vec!["II"])),
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(parser(INPUT), Ok(("", parsed_input())));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 1651);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 0);
    }
}

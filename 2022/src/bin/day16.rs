use advent_of_code_2022::{read, parse};
use itertools::Itertools;
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

    Ok((input, (id, Valve::new(flow_rate, to))))
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

impl<'a> From<Input<'a>> for World {
    fn from(input: Input) -> Self {
        let valves: HashMap<String, u8> = input
            .iter()
            .filter(|(_, valve)| valve.flow_rate != 0)
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

        for i in nodes.iter() {
            if *i != "AA" && valves.get(i) == None {
                distances.remove(i);
            } else {
                let distances_to = distances.get_mut(i).unwrap();

                for j in nodes.iter() {
                    if i==j || valves.get(j) == None {
                        distances_to.remove(j);
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

fn compute_flow(world: &World, valves_to_check: &HashSet<&str>, remaining_time: u8, valve_id: &str) -> u16 {
    valves_to_check
        .iter()
        .filter_map(|to| {
            let time_needed = *world.distances.get(valve_id).unwrap().get(*to).unwrap() + 1;

            if time_needed > remaining_time {
                return None;
            }

            let remaining_time = remaining_time - time_needed;

            let flow_rate = *world.valves.get(*to).unwrap();
            let flow = flow_rate as u16 * remaining_time as u16;

            let mut valves_to_check = valves_to_check.clone();

            valves_to_check.remove(to);

            Some(flow + compute_flow(world, &valves_to_check, remaining_time, to))
        })
        .max()
        .unwrap_or(0)
}

fn solve_part1(world: &World) -> u16 {
    let valves: Vec<String> = world.valves.keys().cloned().collect();
    let valves_set: HashSet<&str> = valves.iter().map(|s| &s as &str).collect();

    compute_flow(&world, &valves_set, 30, "AA")
}

fn solve_part2(world: &World) -> u16 {
    let valves: Vec<String> = world.valves.keys().cloned().collect();
    let valves_set: HashSet<&str> = valves.iter().map(|s| &s as &str).collect();

    let mut best = 0;

    for valves_for_me in valves.iter().powerset() {
        let valves_for_me: HashSet<&str> = valves_for_me.iter().map(|s| &s as &str).collect();
        let valves_for_elephant: HashSet<&str> = valves_set.difference(&valves_for_me).cloned().collect();

        let current = compute_flow(&world, &valves_for_me, 26, "AA") + compute_flow(&world, &valves_for_elephant, 26, "AA");

        if current > best {
            best = current;
        }
    }

    best
}

fn main() {
    let input = read(16);

    let parsed = parse(parser, &input);
    let world = World::from(parsed);

    println!("{}", solve_part1(&world));
    println!("{}", solve_part2(&world));
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
        assert_eq!(solve_part1(&World::from(parsed_input())), 1651);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&World::from(parsed_input())), 1707);
    }
}

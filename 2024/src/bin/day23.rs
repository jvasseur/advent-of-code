use std::collections::{BTreeSet, HashSet};

use advent_of_code_2024::{parser::*, read};
use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete::alpha1, combinator::into, multi::many1, sequence::{separated_pair, terminated}, IResult};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
struct Id(String);

impl From<&str> for Id {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl Parsable for Id {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, value) = alpha1(input)?;

        Ok((input, Id(value.to_owned())))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    connections: Vec<(Id, Id)>,
}

impl From<Vec<(Id, Id)>> for Input {
    fn from(value: Vec<(Id, Id)>) -> Self {
        Self { connections: value }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        into(many1(terminated(separated_pair(Id::parser, tag("-"), Id::parser), tag("\n"))))(input)
    }
}

fn build_three_sets(connections: &[(Id, Id)]) -> HashSet<BTreeSet<&Id>> {
    let connections_set: HashSet<BTreeSet<&Id>> = connections.into_iter().map(|(a, b)| BTreeSet::from([a, b])).collect();
    let ids_set = connections_set.iter().flatten().collect::<HashSet<_>>();

    let mut sets = HashSet::new();

    for (a, b) in connections {
        for c in &ids_set {
            if connections_set.contains(&BTreeSet::from([b, c])) && connections_set.contains(&BTreeSet::from([a, c])) {
                sets.insert(BTreeSet::from([a, b, c]));
            }
        }
    }

    sets
}

fn build_full_sets(connections: &[(Id, Id)]) -> HashSet<BTreeSet<&Id>> {
    let connections_set: HashSet<BTreeSet<&Id>> = connections.into_iter().map(|(a, b)| BTreeSet::from([a, b])).collect();
    let ids_set = connections_set.iter().flatten().collect::<HashSet<_>>();

    let mut sets: HashSet<BTreeSet<&Id>> = connections_set
        .iter()
        .cloned()
        .collect();

    for &c in ids_set {
        for set in sets.clone() {
            if set.iter().all(|id| connections_set.contains(&BTreeSet::from([id, c]))) {
                let mut set = sets.take(&set).unwrap();

                set.insert(c);

                sets.insert(set);
            }
        }
    }

    sets
}

fn solve_part1(input: &Input) -> usize {
    build_three_sets(&input.connections).into_iter().filter(|set| set.iter().any(|id| id.0.starts_with("t"))).count()
}

fn solve_part2(input: &Input) -> String {
    build_full_sets(&input.connections).into_iter().max_by(|a, b| a.len().cmp(&b.len())).unwrap().into_iter().map(|id| id.0.clone()).sorted().join(",")
}

fn main() {
    let input = parse(&read(23).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

    fn parsed_input() -> Input {
        Input::from(vec![
            (Id::from("kh"), Id::from("tc")),
            (Id::from("qp"), Id::from("kh")),
            (Id::from("de"), Id::from("cg")),
            (Id::from("ka"), Id::from("co")),
            (Id::from("yn"), Id::from("aq")),
            (Id::from("qp"), Id::from("ub")),
            (Id::from("cg"), Id::from("tb")),
            (Id::from("vc"), Id::from("aq")),
            (Id::from("tb"), Id::from("ka")),
            (Id::from("wh"), Id::from("tc")),
            (Id::from("yn"), Id::from("cg")),
            (Id::from("kh"), Id::from("ub")),
            (Id::from("ta"), Id::from("co")),
            (Id::from("de"), Id::from("co")),
            (Id::from("tc"), Id::from("td")),
            (Id::from("tb"), Id::from("wq")),
            (Id::from("wh"), Id::from("td")),
            (Id::from("ta"), Id::from("ka")),
            (Id::from("td"), Id::from("qp")),
            (Id::from("aq"), Id::from("cg")),
            (Id::from("wq"), Id::from("ub")),
            (Id::from("ub"), Id::from("vc")),
            (Id::from("de"), Id::from("ta")),
            (Id::from("wq"), Id::from("aq")),
            (Id::from("wq"), Id::from("vc")),
            (Id::from("wh"), Id::from("yn")),
            (Id::from("ka"), Id::from("de")),
            (Id::from("kh"), Id::from("ta")),
            (Id::from("co"), Id::from("tc")),
            (Id::from("wh"), Id::from("qp")),
            (Id::from("tb"), Id::from("vc")),
            (Id::from("td"), Id::from("yn")),
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse::<Input>(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_build_three_sets() {
        for set in build_three_sets(&parsed_input().connections) {
            println!("{:?}", set);
        }

        assert_eq!(build_three_sets(&parsed_input().connections).len(), 12);
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 7);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), "co,de,ka,ta");
    }
}

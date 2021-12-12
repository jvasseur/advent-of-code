use advent_of_code_2021::{parse_lines, read};
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::HashMap;
use std::collections::HashSet;

fn line_parser(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alpha1, tag("-"), alpha1)(input)
}

struct Cave<'a> {
    connections: HashSet<&'a str>,
}

impl<'a> Cave<'a> {
    fn new() -> Self {
        Cave {
            connections: HashSet::new(),
        }
    }
}

struct Map<'a> {
    caves: HashMap<&'a str, Cave<'a>>,
}

impl<'a> Map<'a> {
    fn new(input: &[(&'a str, &'a str)]) -> Self {
        let mut caves: HashMap<&'a str, Cave<'a>> = HashMap::new();

        for (edge_a, edge_b) in input {
            if !caves.contains_key(edge_a) {
                caves.insert(edge_a, Cave::new());
            }
            if !caves.contains_key(edge_b) {
                caves.insert(edge_b, Cave::new());
            }

            caves.get_mut(edge_a).unwrap().connections.insert(edge_b);
            caves.get_mut(edge_b).unwrap().connections.insert(edge_a);
        }

        Self { caves }
    }
}

fn get_paths(map: &Map, start: &str, visited: &[&str], visited_twice: bool) -> u32 {
    let mut paths = 0;

    let mut new_visited = visited.to_owned();
    if !start.chars().all(char::is_uppercase) {
        new_visited.push(start);
    }

    for connection in map.caves.get(start).unwrap().connections.iter() {
        if connection == &"end" {
            paths += 1;

            continue;
        }

        if connection == &"start" {
            // We can't return to the start
            continue;
        }

        if !visited.contains(connection) {
            paths += get_paths(map, connection, &new_visited, visited_twice);

            continue;
        }

        if !visited_twice {
            paths += get_paths(map, connection, &new_visited, true);

            continue;
        }
    }

    paths
}

fn solve_part1<'a>(input: &[(&'a str, &'a str)]) -> u32 {
    let map = Map::new(input);

    get_paths(&map, &"start", &vec![], true)
}

fn solve_part2<'a>(input: &[(&'a str, &'a str)]) -> u32 {
    let map = Map::new(input);

    get_paths(&map, &"start", &vec![], false)
}

fn main() {
    let input = read(12);

    let parsed_input = parse_lines(line_parser, &input);

    println!("{}", solve_part1(&parsed_input));
    println!("{}", solve_part2(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::line_parser;
    use super::solve_part1;
    use super::solve_part2;

    #[test]
    fn test_line_parser() {
        assert_eq!(line_parser("HN-start"), Ok(("", ("HN", "start"))));
    }

    #[test]
    fn test_solve_part_1() {
        let input = vec![
            ("start", "A"),
            ("start", "b"),
            ("A", "c"),
            ("A", "b"),
            ("b", "d"),
            ("A", "end"),
            ("b", "end"),
        ];

        assert_eq!(solve_part1(&input), 10);
    }

    #[test]
    fn test_solve_part_2() {
        let input = vec![
            ("start", "A"),
            ("start", "b"),
            ("A", "c"),
            ("A", "b"),
            ("b", "d"),
            ("A", "end"),
            ("b", "end"),
        ];

        assert_eq!(solve_part2(&input), 36);
    }
}

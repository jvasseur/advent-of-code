use advent_of_code_2021::{parse, read};
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::character::complete::satisfy;
use nom::multi::many0;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashMap;
use std::collections::HashSet;

fn char_parser(input: &str) -> IResult<&str, char> {
    satisfy(|c| c.is_ascii_uppercase() && c.is_ascii_alphabetic())(input)
}

fn template_line_parser(input: &str) -> IResult<&str, Vec<char>> {
    terminated(many0(char_parser), newline)(input)
}

fn insertion_line_parser(input: &str) -> IResult<&str, ((char, char), char)> {
    let (rest, (first, second, _, inserted, _)) =
        tuple((char_parser, char_parser, tag(" -> "), char_parser, newline))(input)?;

    Ok((rest, ((first, second), inserted)))
}

fn parser(input: &str) -> IResult<&str, (Vec<char>, HashMap<(char, char), char>)> {
    let (input, template) = template_line_parser(input)?;
    let (input, _) = newline(input)?;
    let (input, insertions) = many0(insertion_line_parser)(input)?;

    Ok((input, (template, insertions.into_iter().collect())))
}

fn marge_counts(count_a: HashMap<char, u64>, count_b: HashMap<char, u64>) -> HashMap<char, u64> {
    let keys_a: HashSet<&char> = count_a.keys().collect();
    let keys_b: HashSet<&char> = count_b.keys().collect();

    keys_a
        .union(&keys_b)
        .into_iter()
        .map(|key| {
            (
                key.to_owned().to_owned(),
                count_a.get(key).or(Some(&0)).unwrap() + count_b.get(key).or(Some(&0)).unwrap(),
            )
        })
        .collect()
}

struct Rules {
    rules: HashMap<(char, char), char>,
    memoized: HashMap<((char, char), u8), HashMap<char, u64>>,
}

impl Rules {
    fn new(rules: HashMap<(char, char), char>) -> Self {
        Self {
            rules,
            memoized: HashMap::new(),
        }
    }

    fn polymerize_count_chain(&mut self, input: &[char], iterations: u8) -> HashMap<char, u64> {
        let mut counts = HashMap::new();

        for (&a, &b) in input.iter().tuple_windows() {
            counts = marge_counts(counts, self.polymerize_count((a, b), iterations).to_owned());
        }

        for inserted in input.iter().skip(1).dropping_back(1) {
            *counts.get_mut(&inserted).unwrap() -= 1;
        }

        counts
    }

    fn polymerize_count(&mut self, (a, b): (char, char), iterations: u8) -> &HashMap<char, u64> {
        if None == self.memoized.get(&((a, b), iterations)) {
            if iterations == 0 {
                self.memoized.insert(
                    ((a, b), iterations),
                    marge_counts(HashMap::from([(a, 1)]), HashMap::from([(b, 1)])),
                );
            } else {
                let insert = *self.rules.get(&(a, b)).unwrap();

                let mut counts = marge_counts(
                    self.polymerize_count((a, insert), iterations - 1)
                        .to_owned(),
                    self.polymerize_count((insert, b), iterations - 1)
                        .to_owned(),
                );

                *counts.get_mut(&insert).unwrap() -= 1;

                self.memoized.insert(((a, b), iterations), counts);
            }
        }

        self.memoized.get(&((a, b), iterations)).unwrap()
    }
}

fn solve_part1(input: &(Vec<char>, HashMap<(char, char), char>)) -> u64 {
    let mut rules = Rules::new(input.1.to_owned());

    let counts = rules.polymerize_count_chain(&input.0, 10);

    if let MinMax(min, max) = counts.values().minmax() {
        max - min
    } else {
        panic!("Here be dragons");
    }
}

fn solve_part2(input: &(Vec<char>, HashMap<(char, char), char>)) -> u64 {
    let mut rules = Rules::new(input.1.to_owned());

    let counts = rules.polymerize_count_chain(&input.0, 40);

    if let MinMax(min, max) = counts.values().minmax() {
        max - min
    } else {
        panic!("Here be dragons");
    }
}

fn main() {
    let input = read(14);

    let parsed_input = parse(parser, &input);

    println!("{}", solve_part1(&parsed_input));
    println!("{}", solve_part2(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::parser;
    use super::solve_part1;
    use super::Rules;
    use std::collections::HashMap;

    #[test]
    fn test_parser() {
        assert_eq!(
            parser("NNCB\n\nCH -> B\nHH -> N\nCB -> H\n"),
            Ok((
                "",
                (
                    vec!['N', 'N', 'C', 'B'],
                    HashMap::from([(('C', 'H'), 'B'), (('H', 'H'), 'N'), (('C', 'B'), 'H'),]),
                )
            ))
        );
    }

    #[test]
    fn test_polymerize_count_1() {
        let mut rules = Rules::new(HashMap::from([
            (('C', 'H'), 'B'),
            (('H', 'H'), 'N'),
            (('C', 'B'), 'H'),
            (('N', 'H'), 'C'),
            (('H', 'B'), 'C'),
            (('H', 'C'), 'B'),
            (('H', 'N'), 'C'),
            (('N', 'N'), 'C'),
            (('B', 'H'), 'H'),
            (('N', 'C'), 'B'),
            (('N', 'B'), 'B'),
            (('B', 'N'), 'B'),
            (('B', 'B'), 'N'),
            (('B', 'C'), 'B'),
            (('C', 'C'), 'N'),
            (('C', 'N'), 'C'),
        ]));

        let counts = rules.polymerize_count(('N', 'N'), 1);

        assert_eq!(counts, &HashMap::from([('C', 1), ('N', 2),]));
    }

    #[test]
    fn test_polymerize_count_1_a() {
        let mut rules = Rules::new(HashMap::from([
            (('C', 'H'), 'B'),
            (('H', 'H'), 'N'),
            (('C', 'B'), 'H'),
            (('N', 'H'), 'C'),
            (('H', 'B'), 'C'),
            (('H', 'C'), 'B'),
            (('H', 'N'), 'C'),
            (('N', 'N'), 'C'),
            (('B', 'H'), 'H'),
            (('N', 'C'), 'B'),
            (('N', 'B'), 'B'),
            (('B', 'N'), 'B'),
            (('B', 'B'), 'N'),
            (('B', 'C'), 'B'),
            (('C', 'C'), 'N'),
            (('C', 'N'), 'C'),
        ]));

        let counts = rules.polymerize_count(('N', 'C'), 1);

        assert_eq!(counts, &HashMap::from([('C', 1), ('B', 1), ('N', 1),]));
    }

    #[test]
    fn test_polymerize_count_1_b() {
        let mut rules = Rules::new(HashMap::from([
            (('C', 'H'), 'B'),
            (('H', 'H'), 'N'),
            (('C', 'B'), 'H'),
            (('N', 'H'), 'C'),
            (('H', 'B'), 'C'),
            (('H', 'C'), 'B'),
            (('H', 'N'), 'C'),
            (('N', 'N'), 'C'),
            (('B', 'H'), 'H'),
            (('N', 'C'), 'B'),
            (('N', 'B'), 'B'),
            (('B', 'N'), 'B'),
            (('B', 'B'), 'N'),
            (('B', 'C'), 'B'),
            (('C', 'C'), 'N'),
            (('C', 'N'), 'C'),
        ]));

        let counts = rules.polymerize_count(('C', 'N'), 1);

        assert_eq!(counts, &HashMap::from([('C', 2), ('N', 1),]));
    }

    #[test]
    fn test_polymerize_count_2() {
        let mut rules = Rules::new(HashMap::from([
            (('C', 'H'), 'B'),
            (('H', 'H'), 'N'),
            (('C', 'B'), 'H'),
            (('N', 'H'), 'C'),
            (('H', 'B'), 'C'),
            (('H', 'C'), 'B'),
            (('H', 'N'), 'C'),
            (('N', 'N'), 'C'),
            (('B', 'H'), 'H'),
            (('N', 'C'), 'B'),
            (('N', 'B'), 'B'),
            (('B', 'N'), 'B'),
            (('B', 'B'), 'N'),
            (('B', 'C'), 'B'),
            (('C', 'C'), 'N'),
            (('C', 'N'), 'C'),
        ]));

        let counts = rules.polymerize_count(('N', 'N'), 2);

        assert_eq!(counts, &HashMap::from([('B', 1), ('C', 2), ('N', 2),]));
    }

    #[test]
    fn test_polymerize_count_chain_1_a() {
        let mut rules = Rules::new(HashMap::from([
            (('C', 'H'), 'B'),
            (('H', 'H'), 'N'),
            (('C', 'B'), 'H'),
            (('N', 'H'), 'C'),
            (('H', 'B'), 'C'),
            (('H', 'C'), 'B'),
            (('H', 'N'), 'C'),
            (('N', 'N'), 'C'),
            (('B', 'H'), 'H'),
            (('N', 'C'), 'B'),
            (('N', 'B'), 'B'),
            (('B', 'N'), 'B'),
            (('B', 'B'), 'N'),
            (('B', 'C'), 'B'),
            (('C', 'C'), 'N'),
            (('C', 'N'), 'C'),
        ]));

        let counts = rules.polymerize_count_chain(&vec!['N', 'N', 'C'], 1);

        assert_eq!(counts, HashMap::from([('B', 1), ('C', 2), ('N', 2),]));
    }

    #[test]
    fn test_polymerize_count_chain_1() {
        let mut rules = Rules::new(HashMap::from([
            (('C', 'H'), 'B'),
            (('H', 'H'), 'N'),
            (('C', 'B'), 'H'),
            (('N', 'H'), 'C'),
            (('H', 'B'), 'C'),
            (('H', 'C'), 'B'),
            (('H', 'N'), 'C'),
            (('N', 'N'), 'C'),
            (('B', 'H'), 'H'),
            (('N', 'C'), 'B'),
            (('N', 'B'), 'B'),
            (('B', 'N'), 'B'),
            (('B', 'B'), 'N'),
            (('B', 'C'), 'B'),
            (('C', 'C'), 'N'),
            (('C', 'N'), 'C'),
        ]));

        let counts = rules.polymerize_count_chain(&vec!['N', 'N', 'C', 'B'], 1);

        assert_eq!(
            counts,
            HashMap::from([('B', 2), ('C', 2), ('H', 1), ('N', 2),])
        );
    }

    #[test]
    fn test_polymerize_count_chain_10() {
        let mut rules = Rules::new(HashMap::from([
            (('C', 'H'), 'B'),
            (('H', 'H'), 'N'),
            (('C', 'B'), 'H'),
            (('N', 'H'), 'C'),
            (('H', 'B'), 'C'),
            (('H', 'C'), 'B'),
            (('H', 'N'), 'C'),
            (('N', 'N'), 'C'),
            (('B', 'H'), 'H'),
            (('N', 'C'), 'B'),
            (('N', 'B'), 'B'),
            (('B', 'N'), 'B'),
            (('B', 'B'), 'N'),
            (('B', 'C'), 'B'),
            (('C', 'C'), 'N'),
            (('C', 'N'), 'C'),
        ]));

        let counts = rules.polymerize_count_chain(&vec!['N', 'N', 'C', 'B'], 10);

        assert_eq!(
            counts,
            HashMap::from([('B', 1749), ('C', 298), ('H', 161), ('N', 865),])
        );
    }

    #[test]
    fn test_solve_part1() {
        let input = (
            vec!['N', 'N', 'C', 'B'],
            HashMap::from([
                (('C', 'H'), 'B'),
                (('H', 'H'), 'N'),
                (('C', 'B'), 'H'),
                (('N', 'H'), 'C'),
                (('H', 'B'), 'C'),
                (('H', 'C'), 'B'),
                (('H', 'N'), 'C'),
                (('N', 'N'), 'C'),
                (('B', 'H'), 'H'),
                (('N', 'C'), 'B'),
                (('N', 'B'), 'B'),
                (('B', 'N'), 'B'),
                (('B', 'B'), 'N'),
                (('B', 'C'), 'B'),
                (('C', 'C'), 'N'),
                (('C', 'N'), 'C'),
            ]),
        );

        assert_eq!(solve_part1(&input), 1588);
    }
}

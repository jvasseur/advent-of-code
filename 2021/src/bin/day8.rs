use advent_of_code_2021::{read, parse_lines};
use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use std::collections::BTreeSet;
use std::collections::HashMap;

type Pattern<'a> = &'a str;
type Patterns<'a> = Vec<Pattern<'a>>;
type Output<'a> = Vec<Pattern<'a>>;

fn pattern_parser(input: &str) -> IResult<&str, Pattern> {
    alpha1(input)
}

fn patterns_parser(input: &str) -> IResult<&str, Patterns> {
    separated_list0(tag(" "), pattern_parser)(input)
}

fn output_parser(input: &str) -> IResult<&str, Output> {
    separated_list0(tag(" "), pattern_parser)(input)
}

fn line_parser(input: &str) -> IResult<&str, (Patterns, Output)> {
    separated_pair(patterns_parser, tag(" | "), output_parser)(input)
}

fn solve_part1(input: &[(Patterns, Output)]) -> u32 {
    let mut count = 0;

    for (_, outputs) in input {
        for output in outputs {
            if output.len() == 2 || output.len() == 3 || output.len() == 4 || output.len() == 7 {
                count += 1;
            }
        }
    }

    count
}

fn to_hash(input: &str) -> BTreeSet<char> {
    input.chars().collect()
}

fn solve_part2(input: &[(Patterns, Output)]) -> u32 {
    let mut sum = 0;

    for (patterns, outputs) in input {
        let hashes: Vec<BTreeSet<char>> = patterns.iter().map(|pattern| to_hash(pattern)).collect();

        let p1 = hashes.iter().find(|&pattern| pattern.len() == 2).unwrap();
        let p4 = hashes.iter().find(|&pattern| pattern.len() == 4).unwrap();
        let p7 = hashes.iter().find(|&pattern| pattern.len() == 3).unwrap();
        let p8 = hashes.iter().find(|&pattern| pattern.len() == 7).unwrap();

        let p6 = hashes.iter().find(|&pattern| pattern.len() == 6 && !pattern.is_superset(p1)).unwrap();
        let p9 = hashes.iter().find(|&pattern| pattern.len() == 6 && pattern.is_superset(p1) && pattern.is_superset(p4)).unwrap();
        let p0 = hashes.iter().find(|&pattern| pattern.len() == 6 && pattern.is_superset(p1) && !pattern.is_superset(p4)).unwrap();

        let p3 = hashes.iter().find(|&pattern| pattern.len() == 5 && pattern.is_superset(p1)).unwrap();
        let p5 = hashes.iter().find(|&pattern| pattern.len() == 5 && !pattern.is_superset(p1) && pattern.is_subset(p6)).unwrap();
        let p2 = hashes.iter().find(|&pattern| pattern.len() == 5 && !pattern.is_superset(p1) && !pattern.is_subset(p6)).unwrap();

        let map = HashMap::from([
            (p0, 0),
            (p1, 1),
            (p2, 2),
            (p3, 3),
            (p4, 4),
            (p5, 5),
            (p6, 6),
            (p7, 7),
            (p8, 8),
            (p9, 9),
        ]);

        let outputs_hashes: Vec<BTreeSet<char>> = outputs.iter().map(|output| to_hash(output)).collect();

        let mut num = 0;
        for (i, hash) in outputs_hashes.iter().rev().enumerate() {
            num += map.get(hash).unwrap() * 10_u32.pow(i as u32);
        }

        sum += num;
    }

    sum
}

fn main() {
    let input = read(8);

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
        assert_eq!(
            line_parser("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | dfeb fcadb cdfeb cdbaf"),
            Ok(("", (vec!["acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab"], vec!["dfeb", "fcadb", "cdfeb", "cdbaf"])))
        );
    }

    #[test]
    fn test_solve_part_1() {
        let input = vec![
            (vec!["be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb", "fabcd", "edb"], vec!["fdgacbe", "cefdb", "cefbgd", "gcbe"]),
            (vec!["edbfga", "begcd", "cbg", "gc", "gcadebf", "fbgde", "acbgfd", "abcde", "gfcbed", "gfec"], vec!["fcgedb", "cgb", "dgebacf", "gc"]),
            (vec!["fgaebd", "cg", "bdaec", "gdafb", "agbcfd", "gdcbef", "bgcad", "gfac", "gcb", "cdgabef"], vec!["cg", "cg", "fdcagb", "cbg"]),
            (vec!["fbegcd", "cbd", "adcefb", "dageb", "afcb", "bc", "aefdc", "ecdab", "fgdeca", "fcdbega"], vec!["efabcd", "cedba", "gadfec", "cb"]),
            (vec!["aecbfdg", "fbg", "gf", "bafeg", "dbefa", "fcge", "gcbea", "fcaegb", "dgceab", "fcbdga"], vec!["gecf", "egdcabf", "bgf", "bfgea"]),
            (vec!["fgeab", "ca", "afcebg", "bdacfeg", "cfaedg", "gcfdb", "baec", "bfadeg", "bafgc", "acf"], vec!["gebdcfa", "ecba", "ca", "fadegcb"]),
            (vec!["dbcfg", "fgd", "bdegcaf", "fgec", "aegbdf", "ecdfab", "fbedc", "dacgb", "gdcebf", "gf"], vec!["cefg", "dcbef", "fcge", "gbcadfe"]),
            (vec!["bdfegc", "cbegaf", "gecbf", "dfcage", "bdacg", "ed", "bedf", "ced", "adcbefg", "gebcd"], vec!["ed", "bcgafe", "cdgba", "cbgef"]),
            (vec!["egadfb", "cdbfeg", "cegd", "fecab", "cgb", "gbdefca", "cg", "fgcdab", "egfdb", "bfceg"], vec!["gbdfcae", "bgc", "cg", "cgb"]),
            (vec!["gcafb", "gcf", "dcaebfg", "ecagb", "gf", "abcdeg", "gaef", "cafbge", "fdbac", "fegbdc"], vec!["fgae", "cfgab", "fg", "bagce"]),
        ];

        assert_eq!(solve_part1(&input), 26);
    }

    #[test]
    fn test_solve_part_2() {
        let input = vec![
            (vec!["be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb", "fabcd", "edb"], vec!["fdgacbe", "cefdb", "cefbgd", "gcbe"]),
            (vec!["edbfga", "begcd", "cbg", "gc", "gcadebf", "fbgde", "acbgfd", "abcde", "gfcbed", "gfec"], vec!["fcgedb", "cgb", "dgebacf", "gc"]),
            (vec!["fgaebd", "cg", "bdaec", "gdafb", "agbcfd", "gdcbef", "bgcad", "gfac", "gcb", "cdgabef"], vec!["cg", "cg", "fdcagb", "cbg"]),
            (vec!["fbegcd", "cbd", "adcefb", "dageb", "afcb", "bc", "aefdc", "ecdab", "fgdeca", "fcdbega"], vec!["efabcd", "cedba", "gadfec", "cb"]),
            (vec!["aecbfdg", "fbg", "gf", "bafeg", "dbefa", "fcge", "gcbea", "fcaegb", "dgceab", "fcbdga"], vec!["gecf", "egdcabf", "bgf", "bfgea"]),
            (vec!["fgeab", "ca", "afcebg", "bdacfeg", "cfaedg", "gcfdb", "baec", "bfadeg", "bafgc", "acf"], vec!["gebdcfa", "ecba", "ca", "fadegcb"]),
            (vec!["dbcfg", "fgd", "bdegcaf", "fgec", "aegbdf", "ecdfab", "fbedc", "dacgb", "gdcebf", "gf"], vec!["cefg", "dcbef", "fcge", "gbcadfe"]),
            (vec!["bdfegc", "cbegaf", "gecbf", "dfcage", "bdacg", "ed", "bedf", "ced", "adcbefg", "gebcd"], vec!["ed", "bcgafe", "cdgba", "cbgef"]),
            (vec!["egadfb", "cdbfeg", "cegd", "fecab", "cgb", "gbdefca", "cg", "fgcdab", "egfdb", "bfceg"], vec!["gbdfcae", "bgc", "cg", "cgb"]),
            (vec!["gcafb", "gcf", "dcaebfg", "ecagb", "gf", "abcdeg", "gaef", "cafbge", "fdbac", "fegbdc"], vec!["fgae", "cfgab", "fg", "bagce"]),
        ];

        assert_eq!(solve_part2(&input), 61229);
    }
}

use advent_of_code_2021::{parse, read};
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::character::complete::satisfy;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::multi::many0;
use nom::IResult;
use std::collections::HashMap;
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;

fn char_parser(input: &str) -> IResult<&str, char> {
    satisfy(|c| c.is_ascii_uppercase() && c.is_ascii_alphabetic())(input)
}

fn template_line_parser(input: &str) -> IResult<&str, Vec<char>> {
    terminated(many0(char_parser), newline)(input)
}

fn insertion_line_parser(input: &str) -> IResult<&str, ((char, char), char)> {
    let (rest, (first, second, _, inserted, _)) = tuple((
        char_parser,
        char_parser,
        tag(" -> "),
        char_parser,
        newline,
    ))(input)?;

    Ok((rest, ((first, second), inserted)))
}

fn parser(input: &str) -> IResult<&str, (Vec<char>, HashMap<(char, char), char>)> {
    let (input, template) = template_line_parser(input)?;
    let (input, _) = newline(input)?;
    let (input, insertions) = many0(insertion_line_parser)(input)?;

    Ok((input, (template, insertions.into_iter().collect())))
}

fn polymerize(input: &[char], rules: &HashMap<(char, char), char>) -> Vec<char> {
    input.to_owned().into_iter().interleave(input.iter().tuple_windows().map(|(&a, &b)| rules.get(&(a, b)).unwrap().to_owned())).collect()
}

fn solve_part1(input: &(Vec<char>, HashMap<(char, char), char>)) -> usize {
    let mut chain = input.0.to_owned();

    for _ in 0..10 {
        chain = polymerize(&chain, &input.1);
    }

    if let MinMax(min, max) = chain.into_iter().counts().values().minmax() {
        max - min
    } else {
        panic!("Here be dragons");
    }
}

fn main() {
    let input = read(14);

    let parsed_input = parse(parser, &input);

    println!("{}", solve_part1(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::parser;
    use super::polymerize;
    use std::collections::HashMap;

    #[test]
    fn test_parser() {
        assert_eq!(parser("NNCB\n\nCH -> B\nHH -> N\nCB -> H\n"), Ok(("", (
            vec!['N', 'N', 'C', 'B'],
            HashMap::from([
                (('C', 'H'), 'B'),
                (('H', 'H'), 'N'),
                (('C', 'B'), 'H'),
            ]),
        ))));
    }

    #[test]
    fn test_polymerize() {
        let rules = HashMap::from([
            (('C', 'H'), 'B'),
            (('H', 'H'), 'N'),
            (('C', 'B'), 'H'),
            //NH -> C
            //HB -> C
            //HC -> B
            //HN -> C
            (('N', 'N'), 'C'),
            //BH -> H
            (('N', 'C'), 'B'),
            //NB -> B
            //BN -> B
            //BB -> N
            //BC -> B
            //CC -> N
            //CN -> C
        ]);

        assert_eq!(
            polymerize(&vec!['N', 'N', 'C', 'B'], &rules),
            vec!['N', 'C', 'N', 'B', 'C', 'H', 'B'],
        );
    }
}

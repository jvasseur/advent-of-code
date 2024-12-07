use advent_of_code_2024::{parser::*, read};
use nom::{bytes::complete::tag, multi::separated_list1, sequence::separated_pair, IResult};

type Page = u32;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Rule {
    first: Page,
    second: Page,
}

impl Rule {
    fn new(first: Page, second: Page) -> Self {
        Self { first, second }
    }

    fn validate(&self, pages: &[Page]) -> bool {
        match self.find(pages) {
            (None, None) => true,
            (None, Some(_)) => true,
            (Some(_), None) => true,
            (Some(first), Some(second)) => first < second,
        }
    }

    fn find(&self, pages: &[Page]) -> (Option<usize>, Option<usize>) {
        let mut first_index = None;
        let mut second_index = None;

        for (index, page) in pages.iter().enumerate() {
            if page == &self.first {
                first_index = Some(index);
            }

            if page == &self.second {
                second_index = Some(index);
            }
        }

        (first_index, second_index)
    }
}

impl Parsable for Rule {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (first, second)) = separated_pair(Page::parser, tag("|"), Page::parser)(input)?;

        Ok((input, Self::new(first, second)))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Update {
    pages: Vec<Page>,
}

impl Update {
    fn new(pages: impl Into<Vec<Page>>) -> Self {
        Self { pages: pages.into() }
    }
}

impl Parsable for Update {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, pages) =  separated_list1(tag(","), Page::parser)(input)?;

        Ok((input, Self::new(pages)))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    rules: Vec<Rule>,
    updates: Vec<Update>,
}

impl Input {
    fn new(rules: impl Into<Vec<Rule>>, updates: impl Into<Vec<Update>>) -> Self {
        Self { rules: rules.into(), updates: updates.into() }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, rules) = lines_parser(input)?;
        let (input, _) = tag("\n")(input)?;
        let (input, updates) = lines_parser(input)?;

        Ok((input, Input::new(rules, updates)))
    }
}

fn solve_part1(input: &Input) -> u32 {
    input.updates.iter()
        .filter(|update| input.rules.iter().all(|rule| rule.validate(&update.pages)))
        .map(|update| update.pages[(update.pages.len() - 1) / 2])
        .sum()
}

fn solve_part2(input: &Input) -> u32 {
    input.updates.iter()
        .filter(|update| input.rules.iter().any(|rule| !rule.validate(&update.pages)))
        .map(|update| {
            let applicable_rules: Vec<&Rule> = input.rules.iter().filter(|rule| match rule.find(&update.pages) {
                (Some(_), Some(_)) => true,
                _ => false,
            }).collect();

            let mut pages = update.pages.to_owned();

            loop {
                let mut changed = false;

                for rule in &applicable_rules {
                    match rule.find(&pages) {
                        (Some(first), Some(second)) => {
                            if first > second {
                                pages.swap(first, second);
                                changed = true;
                            }
                        },
                        _ => panic!("Shouldn't append"),
                    }
                }

                if !changed {
                    break;
                }
            }

            pages[(pages.len() - 1) / 2]
        })
        .sum()
}

fn main() {
    let input = parse(&read(5).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    fn parsed_input() -> Input {
        Input::new([
            Rule::new(47,53),
            Rule::new(97,13),
            Rule::new(97,61),
            Rule::new(97,47),
            Rule::new(75,29),
            Rule::new(61,13),
            Rule::new(75,53),
            Rule::new(29,13),
            Rule::new(97,29),
            Rule::new(53,29),
            Rule::new(61,53),
            Rule::new(97,53),
            Rule::new(61,29),
            Rule::new(47,13),
            Rule::new(75,47),
            Rule::new(97,75),
            Rule::new(47,61),
            Rule::new(75,61),
            Rule::new(47,29),
            Rule::new(75,13),
            Rule::new(53,13),
        ], [
            Update::new([75,47,61,53,29]),
            Update::new([97,61,53,29,13]),
            Update::new([75,29,13]),
            Update::new([75,97,47,61,53]),
            Update::new([61,13,29]),
            Update::new([97,13,75,29,47]),
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse::<Input>(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 143);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 123);
    }
}

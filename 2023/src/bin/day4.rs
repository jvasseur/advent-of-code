use std::collections::{HashSet, VecDeque};
use std::cmp::min;
use advent_of_code_2023::{read, Parsable};
use nom::bytes::complete::tag;
use nom::character::complete::{u32, space1};
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::terminated;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    cards: Vec<Card>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Card {
    id: u32,
    winning: HashSet<u32>,
    have: HashSet<u32>,
}

impl Card {
    fn new(id: u32, winning: impl Into<HashSet<u32>>, have: impl Into<HashSet<u32>>) -> Self {
        Self { id, winning: winning.into(), have: have.into() }
    }

    fn wins(&self) -> u32 {
        self.winning.intersection(&self.have).count() as u32
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, cards) = many1(terminated(Card::parser, tag("\n")))(input)?;

        Ok((input, Input { cards }))
    }
}

impl Parsable for Card {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("Card")(input)?;
        let (input, _) = space1(input)?;
        let (input, id) = u32(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, _) = space1(input)?;
        let (input, winning) = separated_list1(space1, u32)(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = tag("|")(input)?;
        let (input, _) = space1(input)?;
        let (input, have) = separated_list1(space1, u32)(input)?;

        Ok((input, Card {
            id,
            winning: winning.into_iter().collect(),
            have: have.into_iter().collect(),
        }))
    }
}

fn solve_part1(input: &Input) -> u32 {
    input.cards.iter().map(|card| {
        let wins = card.wins();

        if wins > 0 {
            2_u32.pow(wins - 1)
        } else {
            0
        }
    }).sum()
}

fn solve_part2(input: &Input) -> u32 {
    let mut cards: VecDeque<Card> = input.cards.iter().cloned().collect();
    let mut count = input.cards.len() as u32;

    let max = input.cards.len() - 1;

    while let Some(card) = cards.pop_front() {
        let id = card.id;
        let wins = card.wins();

        if wins == 0 {
            continue;
        }

        let cards_to_add = &input.cards[min(id as usize, max)..=min((id + wins - 1) as usize, max)];

        for card in cards_to_add {
            count += 1;
            cards.push_back(card.clone());
        };
    }

    count
}

fn main() {
    let input = read(4);

    let parsed = Input::parse(&input).unwrap();

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    fn parsed_input() -> Input {
        Input {
            cards: vec![
                Card::new(1, [41, 48, 83, 86, 17], [83, 86, 6, 31, 17, 9, 48, 53]),
                Card::new(2, [13, 32, 20, 16, 61], [61, 30, 68, 82, 17, 32, 24, 19]),
                Card::new(3, [1, 21, 53, 59, 44], [69, 82, 63, 72, 16, 21, 14, 1]),
                Card::new(4, [41, 92, 73, 84, 69], [59, 84, 76, 51, 58, 5, 54, 83]),
                Card::new(5, [87, 83, 26, 28, 32], [88, 30, 70, 12, 93, 22, 82, 36]),
                Card::new(6, [31, 18, 13, 56, 72], [74, 77, 10, 23, 35, 67, 36, 11]),
            ],
        }
    }

    #[test]
    fn test_parser() {
        assert_eq!(Input::parse(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 13);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 30);
    }
}

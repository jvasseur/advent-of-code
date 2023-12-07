use advent_of_code_2023::{read, Parsable};
use nom::bytes::complete::tag;
use nom::character::complete::u32;
use nom::branch::alt;
use nom::IResult;
use nom::combinator::value;
use nom::multi::{fill, many0};
use nom::sequence::terminated;
use itertools::Itertools;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    hands: Vec<Hand>
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Input {
    fn new(hands: impl Into<Vec<Hand>>) -> Self {
        Input { hands: hands.into() }
    }
}

impl Hand {
    fn new(cards: [Card; 5], bid: u32) -> Self {
        Hand { cards, bid }
    }

    fn get_type_part1(&self) -> HandType {
        let count_by_type = self.cards.iter().counts();

        if count_by_type.values().any(|&count| count == 5) {
            return HandType::FiveOfAKind;
        }

        if count_by_type.values().any(|&count| count == 4) {
            return HandType::FourOfAKind;
        }

        if count_by_type.values().any(|&count| count == 3) && count_by_type.values().any(|&count| count == 2) {
            return HandType::FullHouse;
        }

        if count_by_type.values().any(|&count| count == 3) {
            return HandType::ThreeOfAKind;
        }

        if count_by_type.values().filter(|&&count| count == 2).count() == 2 {
            return HandType::TwoPairs;
        }

        if count_by_type.values().any(|&count| count == 2) {
            return HandType::OnePair;
        }

        return HandType::HighCard;
    }

    fn get_type_part2(&self) -> HandType {
        let (jockers, others): (Vec<Card>, Vec<Card>) = self.cards.iter()
            .partition(|&&card| card == Card::Jack);
        let jocker_count = jockers.len();

        if jocker_count == 0 {
            return self.get_type_part1();
        }

        if jocker_count == 5 {
            return HandType::FiveOfAKind;
        }

        let count_by_type = others.iter().counts();

        if count_by_type.values().any(|&count| count + jocker_count == 5) {
            return HandType::FiveOfAKind;
        }

        if count_by_type.values().any(|&count| count + jocker_count == 4) {
            return HandType::FourOfAKind;
        }

        // At that point there is at most 2 jockers or else the previous if would have matched
        match jocker_count {
            2 => {
                if count_by_type.values().any(|&count| count == 3 || count == 2) {
                    return HandType::FullHouse;
                }
            },
            1 => {
                if count_by_type.values().filter(|&&count| count == 2).count() == 2 {
                    return HandType::FullHouse;
                }
            },
            _ => panic!("Invalid jocker count: {}", jocker_count),
        }

        if count_by_type.values().any(|&count| count + jocker_count == 3) {
            return HandType::ThreeOfAKind;
        }

        // At that point there is at most 1 jockers or else the previous if would have matched
        match jocker_count {
            1 => {
                if count_by_type.values().any(|&count| count == 2) {
                    return HandType::TwoPairs;
                }
            },
            _ => panic!("Invalid jocker count: {}", jocker_count),
        }

        if count_by_type.values().any(|&count| count + jocker_count == 2) {
            return HandType::OnePair;
        }

        return HandType::HighCard;
    }

    fn cmp_part1(&self, other: &Self) -> std::cmp::Ordering {
        self.get_type_part1().cmp(&other.get_type_part1())
            .then(self.cards[0].cmp_part1(&other.cards[0]))
            .then(self.cards[1].cmp_part1(&other.cards[1]))
            .then(self.cards[2].cmp_part1(&other.cards[2]))
            .then(self.cards[3].cmp_part1(&other.cards[3]))
            .then(self.cards[4].cmp_part1(&other.cards[4]))
    }

    fn cmp_part2(&self, other: &Self) -> std::cmp::Ordering {
        self.get_type_part2().cmp(&other.get_type_part2())
            .then(self.cards[0].cmp_part2(&other.cards[0]))
            .then(self.cards[1].cmp_part2(&other.cards[1]))
            .then(self.cards[2].cmp_part2(&other.cards[2]))
            .then(self.cards[3].cmp_part2(&other.cards[3]))
            .then(self.cards[4].cmp_part2(&other.cards[4]))
    }
}

impl Card {
    fn cmp_part1(&self, other: &Self) -> std::cmp::Ordering {
        self.cmp(other)
    }

    fn cmp_part2(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            return std::cmp::Ordering::Equal;
        }

        if self == &Card::Jack {
            return std::cmp::Ordering::Less;
        }

        if other == &Card::Jack {
            return std::cmp::Ordering::Greater;
        }

        self.cmp(other)
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, hands) = many0(terminated(Hand::parser, tag("\n")))(input)?;

        Ok((input, Input::new(hands)))
    }
}

impl Parsable for Hand {
    fn parser(input: &str) -> IResult<&str, Self> {
        let mut cards = [Card::Two; 5];
        let (input, ()) = fill(Card::parser, &mut cards)(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, bid) = u32(input)?;

        Ok((input, Hand::new(cards, bid)))
    }
}

impl Parsable for Card {
    fn parser(input: &str) -> IResult<&str, Self> {
        alt((
            value(Card::Two, tag("2")),
            value(Card::Three, tag("3")),
            value(Card::Four, tag("4")),
            value(Card::Five, tag("5")),
            value(Card::Six, tag("6")),
            value(Card::Seven, tag("7")),
            value(Card::Eight, tag("8")),
            value(Card::Nine, tag("9")),
            value(Card::Ten, tag("T")),
            value(Card::Jack, tag("J")),
            value(Card::Queen, tag("Q")),
            value(Card::King, tag("K")),
            value(Card::Ace, tag("A")),
        ))(input)
    }
}
fn solve_part1(input: &Input) -> u32 {
    input.hands.iter()
        .cloned()
        .sorted_by(Hand::cmp_part1)
        .enumerate()
        .map(|(index, hand)| (index as u32 + 1) * hand.bid)
        .sum()
}

fn solve_part2(input: &Input) -> u32 {
    input.hands.iter()
        .cloned()
        .sorted_by(Hand::cmp_part2)
        .enumerate()
        .map(|(index, hand)| (index as u32 + 1) * hand.bid)
        .sum()
}

fn main() {
    let input = read(7);

    let parsed = Input::parse(&input).unwrap();

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    fn parsed_input() -> Input {
        Input::new([
            Hand::new([Card::Three, Card::Two, Card::Ten, Card::Three, Card::King], 765),
            Hand::new([Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five], 684),
            Hand::new([Card::King, Card::King, Card::Six, Card::Seven, Card::Seven], 28),
            Hand::new([Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten], 220),
            Hand::new([Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace], 483),
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(Input::parse(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 6440);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 5905);
    }
}

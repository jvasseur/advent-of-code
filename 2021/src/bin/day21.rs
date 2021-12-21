use advent_of_code_2021::{parse, read};
use nom::bytes::complete::tag;
use nom::character::complete::u8;
use nom::sequence::tuple;
use nom::IResult;

fn parser(input: &str) -> IResult<&str, (u8, u8)> {
    let (input, (_, p1, _, p2, _)) = tuple((
        tag("Player 1 starting position: "),
        u8,
        tag("\nPlayer 2 starting position: "),
        u8,
        tag("\n"),
    ))(input)?;

    Ok((input, (p1, p2)))
}

struct PracticeDice {
    rolled: u32,
}

impl PracticeDice {
    fn new() -> Self {
        PracticeDice {
            rolled: 0,
        }
    }

    fn roll(&mut self) -> u32 {
        self.rolled += 1;

        (self.rolled - 1).rem_euclid(100) + 1
    }
}

struct Pawn {
    position: u32,
}

impl Pawn {
    fn new(position: u8) -> Self {
        Pawn {
            position: position as u32,
        }
    }

    fn advance(&mut self, count: u32) {
        self.position = (self.position + count - 1).rem_euclid(10) + 1
    }
}

fn solve_part_1(input: (u8, u8)) -> u32 {
    let mut p1_pawn = Pawn::new(input.0);
    let mut p2_pawn = Pawn::new(input.1);

    let mut p1_score = 0;
    let mut p2_score = 0;

    let mut dice = PracticeDice::new();

    loop {
        p1_pawn.advance(dice.roll() + dice.roll() + dice.roll());

        p1_score += p1_pawn.position;

        if p1_score >= 1000 {
            return p2_score * dice.rolled;
        }

        p2_pawn.advance(dice.roll() + dice.roll() + dice.roll());

        p2_score += p2_pawn.position;

        if p2_score >= 1000 {
            return p1_score * dice.rolled;
        }
    }
}

fn main() {
    let input = read(21);

    let parsed_input = parse(parser, &input);

    println!("{}", solve_part_1(parsed_input));
}

#[cfg(test)]
mod tests {
    use super::parser;
    use super::PracticeDice;
    use super::Pawn;
    use super::solve_part_1;

    static TEXT: &str = "Player 1 starting position: 4\nPlayer 2 starting position: 8\n";

    #[test]
    fn test_parser() {
        assert_eq!(parser(TEXT), Ok(("", (4, 8))));
    }

    #[test]
    fn test_dice_1() {
        let mut dice = PracticeDice::new();

        assert_eq!(dice.roll(), 1);
        assert_eq!(dice.roll(), 2);
        assert_eq!(dice.roll(), 3);

        assert_eq!(dice.rolled, 3);
    }

    #[test]
    fn test_dice_2() {
        let mut dice = PracticeDice::new();

        for _ in 0..99 {
            dice.roll();
        }

        assert_eq!(dice.roll(), 100);
        assert_eq!(dice.roll(), 1);

        assert_eq!(dice.rolled, 101);
    }

    #[test]
    fn test_pawn() {
        let mut pawn = Pawn::new(4);

        pawn.advance(6);

        assert_eq!(pawn.position, 10);

        pawn.advance(1);

        assert_eq!(pawn.position, 1);

        pawn.advance(30);

        assert_eq!(pawn.position, 1);

        pawn.advance(29);

        assert_eq!(pawn.position, 10);
    }

    #[test]
    fn test_solve_part_1() {
        assert_eq!(solve_part_1((4, 8)), 739785);
    }
}

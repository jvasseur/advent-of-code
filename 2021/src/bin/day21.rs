use advent_of_code_2021::{parse, read};
use nom::bytes::complete::tag;
use nom::character::complete::u8;
use nom::sequence::tuple;
use nom::IResult;
use std::cmp::max;

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

#[derive(Clone, Copy)]
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

fn play((p1_pawn, p2_pawn): (Pawn, Pawn), (p1_score, p2_score): (u32, u32)) -> (u64, u64) {
    let wins: Vec<(u64, u64)> = (3..=9).map(|advance| {
        let mut p1_pawn_u = p1_pawn.clone();
        let mut p1_score_u = p1_score.clone();

        p1_pawn_u.advance(advance);
        p1_score_u += p1_pawn_u.position;

        if p1_score_u >= 21 {
            (1, 0)
        } else {
            let (p2_score_r, p1_score_r) = play((p2_pawn, p1_pawn_u), (p2_score, p1_score_u));

            (p1_score_r, p2_score_r)
        }
    }).collect();

    (
        wins[0].0 + wins[1].0 * 3 + wins[2].0 * 6 + wins[3].0 * 7 + wins[4].0 * 6 + wins[5].0 * 3 + wins[6].0,
        wins[0].1 + wins[1].1 * 3 + wins[2].1 * 6 + wins[3].1 * 7 + wins[4].1 * 6 + wins[5].1 * 3 + wins[6].1,
    )
}

fn solve_part_2(input: (u8, u8)) -> u64 {
    let (wins_p1, wins_p2) = play((Pawn::new(input.0), Pawn::new(input.1)), (0, 0));

    max(wins_p1, wins_p2)
}

fn main() {
    let input = read(21);

    let parsed_input = parse(parser, &input);

    println!("{}", solve_part_1(parsed_input));
    println!("{}", solve_part_2(parsed_input));
}

#[cfg(test)]
mod tests {
    use super::parser;
    use super::PracticeDice;
    use super::Pawn;
    use super::solve_part_1;
    use super::play;

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

    #[test]
    fn test_play_p1() {
        assert_eq!(play((Pawn::new(4), Pawn::new(8)), (0, 0)), (444356092776315, 341960390180808));
    }
}

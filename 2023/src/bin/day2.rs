use std::collections::HashMap;

use advent_of_code_2023::{read, Parsable};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::u32;
use nom::IResult;
use nom::combinator::value;
use nom::multi::{separated_list1, many1};
use nom::sequence::separated_pair;

#[derive(PartialEq, Eq, Debug)]
struct Input {
    games: Vec<Game>,
}

#[derive(PartialEq, Eq, Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

#[derive(PartialEq, Eq, Debug)]
struct Draw {
    balls: HashMap<Color, u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, games) = many1(Game::parser)(input)?;

        Ok((input, Input { games }))
    }
}

impl Parsable for Game {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("Game ")(input)?;
        let (input, id) = u32(input)?;
        let (input, _) = tag(": ")(input)?;
        let (input, draws) = separated_list1(tag("; "), Draw::parser)(input)?;
        let (input, _) = tag("\n")(input)?;


        Ok((input, Game { id, draws }))
    }
}

impl Parsable for Draw {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, balls) = separated_list1(tag(", "), separated_pair(u32, tag(" "), Color::parser))(input)?;

        Ok((input, Draw {
            balls: balls.into_iter().map(|(count, color)| (color, count)).collect(),
        }))
    }
}

impl Parsable for Color {
    fn parser(input: &str) -> IResult<&str, Self> {
        alt((
            value(Color::Red, tag("red")),
            value(Color::Green, tag("green")),
            value(Color::Blue, tag("blue")),
        ))(input)
    }
}

impl Game {
    fn is_possible(&self) -> bool {
        self.draws.iter().all(|draw| draw.get(&Color::Red) <= 12 && draw.get(&Color::Green) <= 13 && draw.get(&Color::Blue) <= 14)
    }

    fn power(&self) -> u32 {
        let red = self.draws.iter().map(|draw| draw.get(&Color::Red)).max().unwrap();
        let green = self.draws.iter().map(|draw| draw.get(&Color::Green)).max().unwrap();
        let blue = self.draws.iter().map(|draw| draw.get(&Color::Blue)).max().unwrap();

        red * green * blue
    }
}

impl Draw {
    fn get(&self, color: &Color) -> u32 {
        match self.balls.get(color) {
            Some(value) => *value,
            None => 0,
        }
    }
}

fn solve_part1(input: &Input) -> u32 {
    input.games.iter().filter(|game| game.is_possible()).map(|game| game.id).sum()
}

fn solve_part2(input: &Input) -> u32 {
    input.games.iter().map(|game| game.power()).sum()
}

fn main() {
    let input = read(2);

    let parsed = Input::parse(&input).unwrap();

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    fn parsed_input() -> Input {
        Input {
            games: vec![
                Game {
                    id: 1,
                    draws: vec![
                        Draw {
                            balls: HashMap::from([
                                (Color::Blue, 3),
                                (Color::Red, 4),
                            ]),
                        },
                        Draw {
                            balls: HashMap::from([
                                (Color::Red, 1),
                                (Color::Green, 2),
                                (Color::Blue, 6),
                            ]),
                        },
                        Draw {
                            balls: HashMap::from([
                                (Color::Green, 2),
                            ]),
                        },
                    ]
                },
                Game {
                    id: 2,
                    draws: vec![
                        Draw {
                            balls: HashMap::from([
                                (Color::Blue, 1),
                                (Color::Green, 2),
                            ]),
                        },
                        Draw {
                            balls: HashMap::from([
                                (Color::Green, 3),
                                (Color::Blue, 4),
                                (Color::Red, 1),
                            ]),
                        },
                        Draw {
                            balls: HashMap::from([
                                (Color::Green, 1),
                                (Color::Blue, 1),
                            ]),
                        },
                    ]
                },
                Game {
                    id: 3,
                    draws: vec![
                        Draw {
                            balls: HashMap::from([
                                (Color::Green, 8),
                                (Color::Blue, 6),
                                (Color::Red, 20),
                            ]),
                        },
                        Draw {
                            balls: HashMap::from([
                                (Color::Blue, 5),
                                (Color::Red, 4),
                                (Color::Green, 13),
                            ]),
                        },
                        Draw {
                            balls: HashMap::from([
                                (Color::Green, 5),
                                (Color::Red, 1),
                            ]),
                        },
                    ]
                },
                Game {
                    id: 4,
                    draws: vec![
                        Draw {
                            balls: HashMap::from([
                                (Color::Green, 1),
                                (Color::Red, 3),
                                (Color::Blue, 6),
                            ]),
                        },
                        Draw {
                            balls: HashMap::from([
                                (Color::Green, 3),
                                (Color::Red, 6),
                            ]),
                        },
                        Draw {
                            balls: HashMap::from([
                                (Color::Green, 3),
                                (Color::Blue, 15),
                                (Color::Red, 14),
                            ]),
                        },
                    ]
                },
                Game {
                    id: 5,
                    draws: vec![
                        Draw {
                            balls: HashMap::from([
                                (Color::Red, 6),
                                (Color::Blue, 1),
                                (Color::Green, 3),
                            ]),
                        },
                        Draw {
                            balls: HashMap::from([
                                (Color::Blue, 2),
                                (Color::Red, 1),
                                (Color::Green, 2),
                            ]),
                        },
                    ]
                },
            ],
        }
    }

    #[test]
    fn test_parser() {
        assert_eq!(Input::parse(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 8);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 2286);
    }
}

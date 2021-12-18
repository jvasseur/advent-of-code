use advent_of_code_2021::{parse_lines, read};
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::u8;
use nom::combinator::map;
use nom::combinator::value;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::Finish;
use nom::IResult;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Pair {
    left: Value,
    right: Value,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Value {
    Literal(u8),
    Pair(Box<Pair>),
}

impl Pair {
    fn magnitude(&self) -> u32 {
        self.left.magnitude() * 3 + self.right.magnitude() * 2
    }
}

impl Value {
    fn magnitude(&self) -> u32 {
        match self {
            Value::Literal(value) => *value as u32,
            Value::Pair(pair) => pair.magnitude(),
        }
    }
}

fn value_parser(input: &str) -> IResult<&str, Value> {
    alt((
        map(u8, |value| Value::Literal(value)),
        map(pair_parser, |value| Value::Pair(Box::new(value))),
    ))(input)
}

fn pair_parser(input: &str) -> IResult<&str, Pair> {
    let (input, (_, left, _, right, _)) =
        tuple((tag("["), value_parser, tag(","), value_parser, tag("]")))(input)?;

    Ok((input, Pair { left, right }))
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Token {
    Left,
    Right,
    Comma,
    Value(u8),
}

impl Token {
    fn unwrap_value(&self) -> u8 {
        match self {
            Token::Value(value) => *value,
            _ => panic!("Invalid token found"),
        }
    }
}

fn line_parser(input: &str) -> IResult<&str, Vec<Token>> {
    many0(alt((
        value(Token::Left, tag("[")),
        value(Token::Right, tag("]")),
        value(Token::Comma, tag(",")),
        map(u8, |value| Token::Value(value)),
    )))(input)
}

fn reduce(input: &[Token]) -> Vec<Token> {
    let mut tokens = input.to_owned();

    loop {
        if try_explode(&mut tokens) {
            continue;
        }

        if try_split(&mut tokens) {
            continue;
        }

        break;
    }

    tokens
}

fn try_explode(tokens: &mut Vec<Token>) -> bool {
    let mut depth = 0;

    for (i, token) in tokens.into_iter().enumerate() {
        match token {
            Token::Left => depth += 1,
            Token::Right => depth -= 1,
            _ => (),
        }

        if depth == 5 {
            // Let's explode !
            let (_, left_token, _, right_token, _) = tokens
                .splice(i..i + 5, [Token::Value(0)])
                .collect_tuple()
                .unwrap();

            let left = left_token.unwrap_value();
            let right = right_token.unwrap_value();

            for j in (0..i).rev() {
                if let Token::Value(value) = tokens[j] {
                    tokens[j] = Token::Value(value + left);

                    break;
                }
            }

            for j in i + 1..tokens.len() {
                if let Token::Value(value) = tokens[j] {
                    tokens[j] = Token::Value(value + right);

                    break;
                }
            }

            return true;
        }
    }

    false
}

fn try_split(tokens: &mut Vec<Token>) -> bool {
    for (i, token) in tokens.into_iter().enumerate() {
        if let Token::Value(value) = token {
            let value = *value;
            if value > 9 {
                tokens.splice(
                    i..i + 1,
                    [
                        Token::Left,
                        Token::Value(((value as f32) / 2.).floor() as u8),
                        Token::Comma,
                        Token::Value(((value as f32) / 2.).ceil() as u8),
                        Token::Right,
                    ],
                );

                return true;
            }
        }
    }

    false
}

fn add(left: &[Token], rigth: &[Token]) -> Vec<Token> {
    reduce(
        &[
            &[Token::Left],
            left,
            &[Token::Comma],
            rigth,
            &[Token::Right],
        ]
        .concat(),
    )
}

fn stringify(tokens: &[Token]) -> String {
    tokens
        .iter()
        .map(|token| match token {
            Token::Left => "[".to_string(),
            Token::Right => "]".to_string(),
            Token::Comma => ",".to_string(),
            Token::Value(value) => value.to_string(),
        })
        .collect()
}

fn magnitude(tokens: &[Token]) -> u32 {
    let (_, pair) = pair_parser(&stringify(tokens)).finish().unwrap();

    pair.magnitude()
}

fn solve_part1(input: &[Vec<Token>]) -> u32 {
    let result = input
        .to_owned()
        .into_iter()
        .reduce(|left, right| add(&left, &right))
        .unwrap();

    magnitude(&result)
}

fn solve_part2(input: &[Vec<Token>]) -> u32 {
    input
        .iter()
        .cartesian_product(input)
        .map(|(left, rigth)| magnitude(&add(left, rigth)))
        .max()
        .unwrap()
}

fn main() {
    let input = read(18);

    let parsed_input = parse_lines(line_parser, &input);

    println!("{}", solve_part1(&parsed_input));
    println!("{}", solve_part2(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::add;
    use super::line_parser;
    use super::magnitude;
    use super::pair_parser;
    use super::reduce;
    use super::stringify;
    use super::Pair;
    use super::Value;
    use nom::Finish;

    #[test]
    fn test_pair_parser() {
        assert_eq!(
            pair_parser("[9,[8,7]]"),
            Ok((
                "",
                Pair {
                    left: Value::Literal(9),
                    right: Value::Pair(Box::new(Pair {
                        left: Value::Literal(8),
                        right: Value::Literal(7),
                    }))
                }
            ))
        );
    }

    #[test]
    fn test_reduce_1() {
        let (_, input) = line_parser("[[[[[9,8],1],2],3],4]").finish().unwrap();
        let (_, expected) = line_parser("[[[[0,9],2],3],4]").finish().unwrap();

        assert_eq!(reduce(&input), expected);
    }

    #[test]
    fn test_reduce_2() {
        let (_, input) = line_parser("[11,0]").finish().unwrap();
        let (_, expected) = line_parser("[[5,6],0]").finish().unwrap();

        assert_eq!(reduce(&input), expected);
    }

    #[test]
    fn test_add() {
        let (_, left) = line_parser("[[[[4,3],4],4],[7,[[8,4],9]]]")
            .finish()
            .unwrap();
        let (_, rigth) = line_parser("[1,1]").finish().unwrap();
        let (_, expected) = line_parser("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
            .finish()
            .unwrap();

        assert_eq!(add(&left, &rigth), expected);
    }

    #[test]
    fn test_stringify() {
        let (_, input) = line_parser("[[[[4,3],4],4],[7,[[8,4],9]]]")
            .finish()
            .unwrap();

        assert_eq!(stringify(&input), "[[[[4,3],4],4],[7,[[8,4],9]]]");
    }

    #[test]
    fn test_magnitude() {
        let (_, input) = line_parser("[[1,2],[[3,4],5]]").finish().unwrap();

        assert_eq!(magnitude(&input), 143);
    }
}

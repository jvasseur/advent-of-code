use advent_of_code_2021::{parse_lines, read};
use nom::branch::alt;
use nom::combinator::value;
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::IResult;

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
enum Side {
    Open,
    Close,
}

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
struct Token {
    token: char,
    side: Side,
}

fn line_parser(input: &str) -> IResult<&str, Vec<Token>> {
    many0(alt((
        value(Token { token: '(', side: Side::Open }, tag("(")),
        value(Token { token: '(', side: Side::Close }, tag(")")),
        value(Token { token: '[', side: Side::Open }, tag("[")),
        value(Token { token: '[', side: Side::Close }, tag("]")),
        value(Token { token: '{', side: Side::Open }, tag("{")),
        value(Token { token: '{', side: Side::Close }, tag("}")),
        value(Token { token: '<', side: Side::Open }, tag("<")),
        value(Token { token: '<', side: Side::Close }, tag(">")),
    )))(input)
}

#[derive(Clone,Debug,Eq,PartialEq)]
enum ParseError {
    InvalidToken(Token),
    UnexpectedEnd,
    ExpectedTokens(Vec<Token>),
}

fn get_token(input: &[Token], index: usize) -> Result<Token, ParseError> {
    if index < input.len() {
        Ok(input[index])
    } else {
        Err(ParseError::UnexpectedEnd)
    }
}

fn expect_token(input: &[Token], index: usize, expected: Token) -> Result<(), ParseError> {
    if index < input.len() {
        let token = input[index];

        if token == expected {
            Ok(())
        } else {
            Err(ParseError::InvalidToken(token))
        }
    } else {
        Err(ParseError::ExpectedTokens(vec![expected]))
    }
}

fn parse_chunk(input: &[Token]) -> Result<usize, ParseError> {
    let open = get_token(&input, 0)?;

    if open.side != Side::Open {
        return Err(ParseError::InvalidToken(open));
    }

    let next = get_token(&input, 1);

    if Err(ParseError::UnexpectedEnd) == next {
        return Err(ParseError::ExpectedTokens(vec![Token {
            token: open.token,
            side: Side::Close,
        }]));
    }

    let consumed = if next?.side == Side::Open {
        let result = parse_expression(&input[1..]);

        if let Err(ParseError::ExpectedTokens(tokens)) = result {
            let mut copy = tokens.to_vec();

            copy.push(Token {
                token: open.token,
                side: Side::Close,
            });

            return Err(ParseError::ExpectedTokens(copy));
        } else {
            result?
        }
    } else {
        0
    };

    expect_token(&input, consumed + 1, Token {
        token: open.token,
        side: Side::Close,
    })?;

    Ok(consumed + 2)
}

fn parse_expression(input: &[Token]) -> Result<usize, ParseError> {
    let mut consumed = 0;

    while consumed < input.len() && input[consumed].side == Side::Open {
        consumed += parse_chunk(&input[consumed..])?
    }

    Ok(consumed)
}

fn token_value(token: Token) -> u32 {
    match token.token {
        '(' => 3,
        '[' => 57,
        '{' => 1197,
        '<' => 25137,
        _ => panic!("Here be dragons."),
    }
}

fn completion_token_value(token: &Token) -> u64 {
    match token.token {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("Here be dragons."),
    }
}

fn completion_value(tokens: &[Token]) -> u64 {
    let mut score = 0;

    for token in tokens {
        score = score * 5 + completion_token_value(token);
    }

    score
}

fn solve_part1(input: &[Vec<Token>]) -> u32 {
    let mut score = 0;

    for line in input {
        if let Err(error) = parse_expression(&line) {
            if let ParseError::InvalidToken(token) = error {
                score += token_value(token);
            }
        }
    }

    score
}

fn solve_part2(input: &[Vec<Token>]) -> u64 {
    let mut scores = Vec::new();

    for line in input {
        if let Err(error) = parse_expression(&line) {
            if let ParseError::ExpectedTokens(tokens) = error {
                scores.push(completion_value(&tokens));
            }
        }
    }

    scores.sort();

    scores[(scores.len() - 1) / 2]
}

fn main() {
    let input = read(10);

    let parsed_input = parse_lines(line_parser, &input);

    println!("{}", solve_part1(&parsed_input));
    println!("{}", solve_part2(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::Side;
    use super::Token;
    use super::line_parser;
    use super::ParseError;
    use super::parse_expression;
    use super::completion_value;

    #[test]
    fn test_line_parser() {
        assert_eq!(
            line_parser("{()()()>"),
            Ok(("", vec![
                Token { token: '{', side: Side::Open },
                Token { token: '(', side: Side::Open },
                Token { token: '(', side: Side::Close },
                Token { token: '(', side: Side::Open },
                Token { token: '(', side: Side::Close },
                Token { token: '(', side: Side::Open },
                Token { token: '(', side: Side::Close },
                Token { token: '<', side: Side::Close },
            ]))
        );
    }

    #[test]
    fn test_parse_expression_1() {
        let line = vec![
            Token { token: '(', side: Side::Open },
            Token { token: '(', side: Side::Close },
        ];

        assert_eq!(parse_expression(&line), Ok(2));
    }

    #[test]
    fn test_parse_expression_2() {
        let line = vec![
            Token { token: '(', side: Side::Open },
            Token { token: '(', side: Side::Close },
            Token { token: '(', side: Side::Open },
            Token { token: '(', side: Side::Close },
        ];

        assert_eq!(parse_expression(&line), Ok(4));
    }

    #[test]
    fn test_parse_expression_3() {
        let line = vec![
            Token { token: '(', side: Side::Open },
            Token { token: '(', side: Side::Close },
            Token { token: '(', side: Side::Open },
            Token { token: '{', side: Side::Close },
        ];

        assert_eq!(parse_expression(&line), Err(ParseError::InvalidToken(Token {
            token: '{',
            side: Side::Close,
        })));
    }

    #[test]
    fn test_parse_expression_4() {
        let line = vec![
            Token { token: '{', side: Side::Open },
            Token { token: '(', side: Side::Open },
            Token { token: '(', side: Side::Close },
            Token { token: '(', side: Side::Open },
            Token { token: '(', side: Side::Close },
            Token { token: '(', side: Side::Open },
            Token { token: '(', side: Side::Close },
            Token { token: '<', side: Side::Close },
        ];

        assert_eq!(parse_expression(&line), Err(ParseError::InvalidToken(Token {
            token: '<',
            side: Side::Close,
        })));
    }

    #[test]
    fn test_parse_expression_5() {
        let (_, line) = line_parser("[({(<(())[]>[[{[]{<()<>>").unwrap();

        assert_eq!(parse_expression(&line), Err(ParseError::ExpectedTokens(vec![
            Token { token: '{', side: Side::Close },
            Token { token: '{', side: Side::Close },
            Token { token: '[', side: Side::Close },
            Token { token: '[', side: Side::Close },
            Token { token: '(', side: Side::Close },
            Token { token: '{', side: Side::Close },
            Token { token: '(', side: Side::Close },
            Token { token: '[', side: Side::Close },
        ])));
    }

    #[test]
    fn test_parse_expression_6() {
        let (_, line) = line_parser("[(()[<>])]({[<{<<[]>>(").unwrap();

        assert_eq!(parse_expression(&line), Err(ParseError::ExpectedTokens(vec![
            Token { token: '(', side: Side::Close },
            Token { token: '{', side: Side::Close },
            Token { token: '<', side: Side::Close },
            Token { token: '[', side: Side::Close },
            Token { token: '{', side: Side::Close },
            Token { token: '(', side: Side::Close },
        ])));
    }

    #[test]
    fn test_parse_expression_7() {
        let (_, line) = line_parser("(((({<>}<{<{<>}{[]{[]{}").unwrap();

        assert_eq!(parse_expression(&line), Err(ParseError::ExpectedTokens(vec![
            Token { token: '{', side: Side::Close },
            Token { token: '{', side: Side::Close },
            Token { token: '<', side: Side::Close },
            Token { token: '{', side: Side::Close },
            Token { token: '<', side: Side::Close },
            Token { token: '(', side: Side::Close },
            Token { token: '(', side: Side::Close },
            Token { token: '(', side: Side::Close },
            Token { token: '(', side: Side::Close },
        ])));
    }

    #[test]
    fn test_parse_expression_8() {
        let (_, line) = line_parser("{<[[]]>}<{[{[{[]{()[[[]").unwrap();

        assert_eq!(parse_expression(&line), Err(ParseError::ExpectedTokens(vec![
            Token { token: '[', side: Side::Close },
            Token { token: '[', side: Side::Close },
            Token { token: '{', side: Side::Close },
            Token { token: '{', side: Side::Close },
            Token { token: '[', side: Side::Close },
            Token { token: '{', side: Side::Close },
            Token { token: '[', side: Side::Close },
            Token { token: '{', side: Side::Close },
            Token { token: '<', side: Side::Close },
        ])));
    }

    #[test]
    fn test_completion_value_1() {
        assert_eq!(completion_value(&vec![
            Token { token: '[', side: Side::Close },
            Token { token: '(', side: Side::Close },
            Token { token: '{', side: Side::Close },
            Token { token: '<', side: Side::Close },
        ]), 294);
    }

    #[test]
    fn test_completion_value_2() {
        assert_eq!(completion_value(&vec![
            Token { token: '{', side: Side::Close },
            Token { token: '{', side: Side::Close },
            Token { token: '[', side: Side::Close },
            Token { token: '[', side: Side::Close },
            Token { token: '(', side: Side::Close },
            Token { token: '{', side: Side::Close },
            Token { token: '(', side: Side::Close },
            Token { token: '[', side: Side::Close },
        ]), 288957);
    }
}

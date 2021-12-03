use advent_of_code_2021::{read, parse_lines};
use nom::IResult;
use nom::multi::many0;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;

fn parser(input: &str) -> IResult<&str, Vec<u8>> {
    many0(alt((
        value(0, tag("0")),
        value(1, tag("1")),
    )))(input)
}

fn solve_part1(input: &[Vec<u8>]) -> usize {
    let size = input[0].len();

    let mut gama = 0;
    let mut epsilon = 0;

    for position in 0..size {
        let mut zeros = 0;
        let mut ones = 0;

        for number in input {
            if number[size - position - 1] == 0 {
                zeros += 1
            } else {
                ones += 1
            }
        }

        if ones >= zeros {
            gama += usize::pow(2, position.try_into().unwrap());
        }

        if zeros >= ones {
            epsilon += usize::pow(2, position.try_into().unwrap());
        }
    }

    gama * epsilon
}

fn to_number(input: &[u8]) -> u32 {
    input.iter().rev().enumerate().map(|(i, &bit)| {
        if bit == 1 {
            2_u32.pow(i.try_into().unwrap())
        } else {
            0
        }
    }).sum()
}

fn oxygen(input: &[Vec<u8>]) -> u32 {
    let size = input[0].len();

    let mut list: Vec<Vec<u8>> = input.to_vec();

    for position in 0..size {
        let mut zeros = 0;
        let mut ones = 0;

        for number in &list {
            if number[position] == 0 {
                zeros += 1
            } else {
                ones += 1
            }
        }

        let searched = if ones >= zeros { 1 } else { 0 };

        list.retain(|value| value[position] == searched);

        if list.len() == 1 {
            return to_number(&list[0]);
        }
    }

    panic!("Here be dragons");
}

fn co2(input: &[Vec<u8>]) -> u32 {
    let size = input[0].len();

    let mut list: Vec<Vec<u8>> = input.to_vec();

    for position in 0..size {
        let mut zeros = 0;
        let mut ones = 0;

        for number in &list {
            if number[position] == 0 {
                zeros += 1
            } else {
                ones += 1
            }
        }

        let searched = if ones >= zeros { 0 } else { 1 };

        list.retain(|value| value[position] == searched);

        if list.len() == 1 {
            return to_number(&list[0]);
        }
    }

    panic!("Here be dragons");
}

fn solve_part2(input: &[Vec<u8>]) -> u32 {
    oxygen(input) * co2(input)
}

fn main() {
    let input = read(3);

    let parsed_input = parse_lines(parser, &input);

    println!("{}", solve_part1(&parsed_input));
    println!("{}", solve_part2(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::parser;
    use super::solve_part1;
    use super::to_number;
    use super::oxygen;
    use super::co2;

    #[test]
    fn test_instruction_parser() {
        assert_eq!(parser("00100"), Ok(("", vec![0, 0, 1, 0, 0])));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&vec![
            vec![0,0,1,0,0],
            vec![1,1,1,1,0],
            vec![1,0,1,1,0],
            vec![1,0,1,1,1],
            vec![1,0,1,0,1],
            vec![0,1,1,1,1],
            vec![0,0,1,1,1],
            vec![1,1,1,0,0],
            vec![1,0,0,0,0],
            vec![1,1,0,0,1],
            vec![0,0,0,1,0],
            vec![0,1,0,1,0],
        ]), 198);
    }

    #[test]
    fn test_to_number() {
        assert_eq!(to_number(&vec![1,0,1,1,1]), 23);
        assert_eq!(to_number(&vec![0,1,0,1,0]), 10);
    }

    #[test]
    fn test_oxygen() {
        assert_eq!(oxygen(&vec![
            vec![0,0,1,0,0],
            vec![1,1,1,1,0],
            vec![1,0,1,1,0],
            vec![1,0,1,1,1],
            vec![1,0,1,0,1],
            vec![0,1,1,1,1],
            vec![0,0,1,1,1],
            vec![1,1,1,0,0],
            vec![1,0,0,0,0],
            vec![1,1,0,0,1],
            vec![0,0,0,1,0],
            vec![0,1,0,1,0],
        ]), 23);
    }

    #[test]
    fn test_co2() {
        assert_eq!(co2(&vec![
            vec![0,0,1,0,0],
            vec![1,1,1,1,0],
            vec![1,0,1,1,0],
            vec![1,0,1,1,1],
            vec![1,0,1,0,1],
            vec![0,1,1,1,1],
            vec![0,0,1,1,1],
            vec![1,1,1,0,0],
            vec![1,0,0,0,0],
            vec![1,1,0,0,1],
            vec![0,0,0,1,0],
            vec![0,1,0,1,0],
        ]), 10);
    }
}

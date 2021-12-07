use advent_of_code_2021::{read, parse};
use advent_of_code_2021::util::abs_diff;
use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::character::complete::u32;
use nom::multi::separated_list0;
use nom::sequence::terminated;

fn parser(input: &str) -> IResult<&str, Vec<u32>> {
    terminated(separated_list0(tag(","), u32), newline)(input)
}

fn mean(input: &[u32]) -> u32 {
    let mut numbers = input.to_owned();

    numbers.sort();

    let mid = numbers.len() / 2;

    numbers[mid]
}

fn solve_part1(input: &[u32]) -> u32 {
    let mean = mean(input);

    input.iter().map(|&x| abs_diff(x, mean)).sum()
}

fn calc_part2(input: &[u32], pos: u32) -> u32 {
    input.iter().map(|&x| {
        let n = abs_diff(x, pos);

        n * (n + 1) / 2
    }).sum()
}

fn solve_part2(input: &[u32]) -> u32 {
    let mut pos = mean(input);

    loop {
        let center = calc_part2(input, pos);
        let left = calc_part2(input, pos - 1);
        let right = calc_part2(input, pos + 1);

        if left < center {
            pos -= 1
        } else if right < center {
            pos += 1
        } else {
            return center;
        }
    }
}

fn main() {
    let input = read(7);

    let parsed_input = parse(parser, &input);

    println!("{}", solve_part1(&parsed_input));
    println!("{}", solve_part2(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::parser;
    use super::solve_part1;
    use super::solve_part2;

    #[test]
    fn test_parser() {
        assert_eq!(parser("16,1,2,0,4,2,7,1,2,14\n"), Ok(("", vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14])));
    }

    #[test]
    fn test_solve_part_1() {
        let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        assert_eq!(solve_part1(&input), 37);
    }

    #[test]
    fn test_solve_part_2() {
        let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        assert_eq!(solve_part2(&input), 168);
    }
}

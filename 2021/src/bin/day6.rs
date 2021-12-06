use advent_of_code_2021::{read, parse};
use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::character::complete::u8;
use nom::multi::separated_list0;
use nom::sequence::terminated;

fn parser(input: &str) -> IResult<&str, Vec<u8>> {
    terminated(separated_list0(tag(","), u8), newline)(input)
}

fn solve_cycles(input: &[u8], cycles: u32) -> u64 {
    let mut fishes = [0; 9];

    for fish in input {
        let indice: usize = (*fish).into();

        fishes[indice] += 1;
    }

    for _ in 0..cycles {
        let first = fishes[0];

        // Shift array
        fishes = [fishes[1], fishes[2], fishes[3], fishes[4], fishes[5], fishes[6], fishes[7], fishes[8], 0];

        fishes[6] += first;
        fishes[8] += first;
    }

    fishes.iter().sum()
}

fn solve_part1(input: &[u8]) -> u64 {
    solve_cycles(input, 80)
}

fn solve_part2(input: &[u8]) -> u64 {
    solve_cycles(input, 256)
}

fn main() {
    let input = read(6);

    let parsed_input = parse(parser, &input);

    println!("{}", solve_part1(&parsed_input));
    println!("{}", solve_part2(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::parser;
    use super::solve_part1;

    #[test]
    fn test_parser() {
        assert_eq!(parser("3,4,3,1,2\n"), Ok(("", vec![3, 4, 3, 1, 2])));
    }

    #[test]
    fn test_solve_part_1() {
        let input = vec![3, 4, 3, 1, 2];

        assert_eq!(solve_part1(&input), 5934);
    }
}

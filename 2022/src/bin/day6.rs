use advent_of_code_2022::{read, parse};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::IResult;
use nom::sequence::terminated;

fn parser(input: &str) -> IResult<&str, &str> {
    terminated(alpha1, tag("\n"))(input)
}

fn solve_part1(input: &str) -> usize {
    input
        .chars()
        .tuple_windows()
        .enumerate()
        .filter(|(_, (a, b, c, d))| a != b && a != c && a != d && b != c && b != d && c != d)
        .next().unwrap().0 + 4
}

fn solve_part2(input: &str) -> usize {
    let chars: Vec<char> = input.chars().collect();

    chars
        .windows(14)
        .enumerate()
        .filter(|(_, window)| window.iter().unique().count() == 14)
        .next().unwrap().0 + 14
}

fn main() {
    let input = read(6);

    let parsed = parse(parser, &input);

    println!("{}", solve_part1(parsed));
    println!("{}", solve_part2(parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(parser("mjqjpqmgbljsphdztnvjfqwrcgsmlb\n"), Ok(("", "mjqjpqmgbljsphdztnvjfqwrcgsmlb")));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(solve_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(solve_part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(solve_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(solve_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(solve_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(solve_part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(solve_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(solve_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}

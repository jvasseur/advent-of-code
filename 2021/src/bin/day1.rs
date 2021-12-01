use advent_of_code_2021::{read, parse_lines};
use itertools::Itertools;
use nom::character::complete::u32;

fn solve_part1(input: &[u32]) -> usize {
    input.iter().tuple_windows().filter(|(a, b)| b > a).count()
}

fn solve_part2(input: &[u32]) -> usize {
    input.iter().tuple_windows().filter(|(a, _, _, b)| b > a).count()
}

fn main() {
    let input = read(1);

    let parsed_input = parse_lines(u32, &input);

    println!("{}", solve_part1(&parsed_input));
    println!("{}", solve_part2(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::solve_part1;
    use super::solve_part2;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&vec![
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263,
        ]), 7);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&vec![
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263,
        ]), 5);
    }
}

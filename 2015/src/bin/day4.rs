use advent_of_code_2015::read;
use md5::compute;

fn solve_part1(input: &str) -> u32 {
    let mut number = 0;

    loop {
        number += 1;

        let hash = compute(format!("{}{}", input, number));

        if format!("{:x}", hash).starts_with("00000") {
            return number;
        }
    }
}

fn solve_part2(input: &str) -> u32 {
    let mut number = 0;

    loop {
        number += 1;

        let hash = compute(format!("{}{}", input, number));

        if format!("{:x}", hash).starts_with("000000") {
            return number;
        }
    }
}

fn main() {
    let input = read(4);

    let parsed_input = input.trim_end_matches('\n');

    println!("{}", solve_part1(&parsed_input));
    println!("{}", solve_part2(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::solve_part1;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1("abcdef"), 609043);
        assert_eq!(solve_part1("pqrstuv"), 1048970);
    }
}

use advent_of_code_2015::read;
use md5::compute;

fn hash(input: &str, salt: u32) -> [u8; 16] {
    compute(format!("{}{}", input, salt)).into()
}

fn solve_part1(input: &str) -> u32 {
    let mut number = 0;

    loop {
        number += 1;

        let hash = hash(input, number);

        if hash[0] == 0 && hash[1] == 0 && hash[2] < 16 {
            return number;
        }
    }
}

fn solve_part2(input: &str) -> u32 {
    let mut number = 0;

    loop {
        number += 1;

        let hash = hash(input, number);

        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            return number;
        }
    }
}

fn main() {
    let input = read(4).unwrap();

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

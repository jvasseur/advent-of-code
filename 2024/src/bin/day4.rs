use advent_of_code_2024::{math::{Direction, Grid}, parser::*, read};
use nom::{bytes::complete::tag, character::complete::none_of, multi::many1, sequence::terminated, IResult};

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    grid: Grid<char>,
}

impl Input {
    fn new(grid: impl Into<Grid<char>>) -> Self {
        Self { grid: grid.into() }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, grid) = many1(terminated(many1(none_of("\n")), tag("\n")))(input)?;

        Ok((input, Input::new(grid)))
    }
}

fn solve_part1(input: &Input) -> usize {
    input.grid.points().map(|point| {
        if input.grid.get(&point) != Some(&'X') {
            return 0;
        }

        Direction::VALUES.into_iter().filter(|direction| {
            input.grid.get(&(point + direction * 1)) == Some(&'M') &&
            input.grid.get(&(point + direction * 2)) == Some(&'A') &&
            input.grid.get(&(point + direction * 3)) == Some(&'S')
        }).count()
    }).sum()
}

fn solve_part2(input: &Input) -> usize {
    input.grid.points().filter(|point| {
        input.grid.get(&point) == Some(&'A')
            &&
        (
            input.grid.get(&(*point + Direction::UpLeft * 1)) == Some(&'M') && input.grid.get(&(*point + Direction::DownRight * 1)) == Some(&'S')
                ||
            input.grid.get(&(*point + Direction::UpLeft * 1)) == Some(&'S') && input.grid.get(&(*point + Direction::DownRight * 1)) == Some(&'M')
        )
            &&

        (
            input.grid.get(&(*point + Direction::UpRight * 1)) == Some(&'M') && input.grid.get(&(*point + Direction::DownLeft * 1)) == Some(&'S')
                ||
            input.grid.get(&(*point + Direction::UpRight * 1)) == Some(&'S') && input.grid.get(&(*point + Direction::DownLeft * 1)) == Some(&'M')
        )
    }).count()
}

fn main() {
    let input = parse(&read(4).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    fn parsed_input() -> Input {
        Input::new(vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse::<Input>(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 18);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 9);
    }
}

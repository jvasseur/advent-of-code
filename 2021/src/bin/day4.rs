use advent_of_code_2021::{read, parse};
use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::u8;
use nom::character::complete::newline;
use nom::multi::separated_list0;
use nom::sequence::terminated;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::multi::fill;
use nom::sequence::tuple;
use nom::sequence::separated_pair;

type GridNumbers = [[u8; 5]; 5];

struct Grid {
    numbers: GridNumbers,
    ticked: [[bool; 5]; 5],
}

impl Grid {
    fn new(numbers: GridNumbers) -> Self {
        Grid {
            numbers,
            ticked: [[false; 5]; 5]
        }
    }

    fn tick(&mut self, number: u8) {
        for i in 0..5 {
            for j in 0..5 {
                if self.numbers[i][j] == number {
                    self.ticked[i][j] = true;
                }
            }
        }
    }

    fn is_complete(&self) -> bool {
        for line in 0..5 {
            let mut complete = true;

            for column in 0..5 {
                complete = complete && self.ticked[line][column];
            }

            if complete {
                return true;
            }
        }

        for column in 0..5 {
            let mut complete = true;

            for line in 0..5 {
                complete = complete && self.ticked[line][column];
            }

            if complete {
                return true;
            }
        }

        return false;
    }

    fn unmarked_sum(&self) -> u32 {
        let mut sum = 0_u32;

        for i in 0..5 {
            for j in 0..5 {
                if !self.ticked[i][j] {
                    let number: u32 = self.numbers[i][j].into();

                    sum += number;
                }
            }
        }

        return sum;
    }
}

fn pick_parser(input: &str) -> IResult<&str, Vec<u8>> {
    terminated(separated_list0(tag(","), u8), newline)(input)
}

fn grid_line_parser(input: &str) -> IResult<&str, [u8; 5]> {
    let (rest, (_, a, _, b, _, c, _, d, _, e, _)) = tuple((space0, u8, space1, u8, space1, u8, space1, u8, space1, u8, newline))(input)?;

    Ok((rest, [a, b, c, d, e]))
}

fn grid_parser(input: &str) -> IResult<&str, GridNumbers> {
    let mut grid: [[u8; 5]; 5] = [[0; 5]; 5];
    let (rest, ()) = fill(grid_line_parser, &mut grid)(input)?;

    Ok((rest, grid))
}

fn parser(input: &str) -> IResult<&str, (Vec<u8>, Vec<GridNumbers>)> {
    separated_pair(pick_parser, newline, separated_list0(newline, grid_parser))(input)
}

fn solve_part1(input: &(Vec<u8>, Vec<GridNumbers>)) -> u32 {
    let mut grids: Vec<Grid> = input.1.iter().map(|numbers| Grid::new(*numbers)).collect();

    for number in &input.0 {
        for grid in grids.iter_mut() {
            grid.tick(*number);

            if grid.is_complete() {
                let num: u32 = (*number).into();

                return grid.unmarked_sum() * num;
            }
        }
    }

    panic!("Here be dragons");
}

fn solve_part2(input: &(Vec<u8>, Vec<GridNumbers>)) -> u32 {
    let mut grids: Vec<Grid> = input.1.iter().map(|numbers| Grid::new(*numbers)).collect();

    let mut last = 0;

    for number in &input.0 {
        for grid in grids.iter_mut() {
            if !grid.is_complete() {
                grid.tick(*number);

                if grid.is_complete() {
                    let num: u32 = (*number).into();

                    last = grid.unmarked_sum() * num;
                }
            }
        }
    }

    last
}

fn main() {
    let input = read(4);

    let parsed_input = parse(parser, &input);

    println!("{}", solve_part1(&parsed_input));
    println!("{}", solve_part2(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::pick_parser;
    use super::grid_line_parser;
    use super::grid_parser;
    use super::Grid;

    #[test]
    fn test_pick_parser() {
        assert_eq!(pick_parser("7,4,9,5,11,17,23,2,0,14,21\n"), Ok(("", vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21])));
    }

    #[test]
    fn test_grid_line_parser() {
        assert_eq!(grid_line_parser("22 13 17 11  0\n"), Ok(("", [22, 13, 17, 11, 0])));
    }

    #[test]
    fn test_grid_parser() {
        assert_eq!(grid_parser("22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19\n"), Ok(("", [
            [22, 13, 17, 11, 0],
            [8, 2, 23, 4, 24],
            [21, 9, 14, 16, 7],
            [6, 10, 3, 18, 5],
            [1, 12, 20, 15, 19],
        ])));
    }

    #[test]
    fn test_tick() {
        let mut grid = Grid::new([
            [22, 13, 17, 11, 0],
            [8, 2, 23, 4, 24],
            [21, 9, 14, 16, 7],
            [6, 10, 3, 18, 5],
            [1, 12, 20, 15, 19],
        ]);

        grid.tick(22);
        grid.tick(2);
        grid.tick(24);
        grid.tick(19);

        assert_eq!(grid.ticked[0][0], true, "0,0 -> 22");
        assert_eq!(grid.ticked[1][1], true, "1,1 -> 2");
        assert_eq!(grid.ticked[1][4], true, "1,4 -> 24");
        assert_eq!(grid.ticked[4][4], true, "4,4 -> 19");
        assert_eq!(grid.ticked[1][0], false, "1,0");
    }

    #[test]
    fn test_is_complete_1() {
        let mut grid = Grid::new([
            [22, 13, 17, 11, 0],
            [8, 2, 23, 4, 24],
            [21, 9, 14, 16, 7],
            [6, 10, 3, 18, 5],
            [1, 12, 20, 15, 19],
        ]);

        grid.tick(22);
        grid.tick(13);

        assert_eq!(grid.is_complete(), false);
    }

    #[test]
    fn test_is_complete_2() {
        let mut grid = Grid::new([
            [22, 13, 17, 11, 0],
            [8, 2, 23, 4, 24],
            [21, 9, 14, 16, 7],
            [6, 10, 3, 18, 5],
            [1, 12, 20, 15, 19],
        ]);

        grid.tick(22);
        grid.tick(13);
        grid.tick(17);
        grid.tick(11);
        grid.tick(0);

        assert_eq!(grid.is_complete(), true);
    }

    #[test]
    fn test_is_complete_3() {
        let mut grid = Grid::new([
            [22, 13, 17, 11, 0],
            [8, 2, 23, 4, 24],
            [21, 9, 14, 16, 7],
            [6, 10, 3, 18, 5],
            [1, 12, 20, 15, 19],
        ]);

        grid.tick(22);
        grid.tick(8);
        grid.tick(21);
        grid.tick(6);
        grid.tick(1);

        assert_eq!(grid.is_complete(), true);
    }
}

use advent_of_code_2021::{parse_lines, read};
use nom::bytes::complete::take;
use nom::character::complete::u8;
use nom::combinator::map_parser;
use nom::multi::many0;
use nom::IResult;

fn line_parser(input: &str) -> IResult<&str, Vec<u8>> {
    many0(map_parser(take(1_u8), u8))(input)
}

fn new_vec_size<T: std::clone::Clone>(height: usize, width: usize, value: T) -> Vec<Vec<T>> {
    let mut line_prototype = Vec::new();
    line_prototype.resize(width, value);

    let mut vec = Vec::new();
    vec.resize(height, line_prototype);

    vec
}

type Point = (usize, usize);

struct Map {
    points: Vec<Vec<u8>>,
    height: usize,
    width: usize,
}

impl Map {
    fn new(input: &[Vec<u8>]) -> Self {
        Self {
            points: input.to_owned(),
            height: input.len(),
            width: input[0].len(),
        }
    }

    fn neighbours(&self, (i, j): Point) -> Vec<Point> {
        let mut neighbours = Vec::new();

        if i != self.height - 1 {
            neighbours.push((i + 1, j))
        }

        if j != self.width - 1 {
            neighbours.push((i, j + 1))
        }

        if i != 0 {
            neighbours.push((i - 1, j))
        }

        if j != 0 {
            neighbours.push((i, j - 1))
        }

        neighbours
    }

    fn build_risk(&self) -> Vec<Vec<Option<u32>>> {
        let mut risk = new_vec_size(self.height, self.width, None);

        risk[self.height - 1][self.width - 1] = Some(0);

        loop {
            let mut changed = false;

            for i in (0..self.height).rev() {
                for j in (0..self.width).rev() {
                    if i == self.height - 1 && j == self.width - 1 {
                        continue;
                    }

                    let local_risk = self
                        .neighbours((i, j))
                        .into_iter()
                        .filter_map(|(k, l)| risk[k][l].map(|local| local + self.points[k][l] as u32))
                        .min();

                    if local_risk != risk[i][j] {
                        risk[i][j] = local_risk;
                        changed = true;
                    }
                }
            }

            if !changed {
                return risk
            }
        }
    }
}

fn solve_part1(input: &[Vec<u8>]) -> u32 {
    let map = Map::new(input);

    map.build_risk()[0][0].unwrap()
}

fn solve_part2(input: &[Vec<u8>]) -> u32 {
    let height = input.len();
    let width = input[0].len();

    let mut points = new_vec_size(height * 5, width * 5, 0);

    for k in 0..5_usize {
        for l in 0..5_usize {
            for i in 0..height {
                for j in 0..width {
                    let mut value = (input[i][j] as usize + k + l) as u8;

                    while value > 9 {
                        value = value - 9;
                    }

                    points[k * height + i][l * width + j] = value
                }
            }
        }
    }

    let map = Map::new(&points[..]);

    map.build_risk()[0][0].unwrap()
}

fn main() {
    let input = read(15);

    let parsed_input = parse_lines(line_parser, &input);

    println!("{}", solve_part1(&parsed_input));
    println!("{}", solve_part2(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::line_parser;
    use super::solve_part1;
    use super::solve_part2;

    #[test]
    fn test_line_parser() {
        assert_eq!(
            line_parser("1163751742"),
            Ok(("", (vec![1, 1, 6, 3, 7, 5, 1, 7, 4, 2])))
        );
    }

    #[test]
    fn test_solve_part_1() {
        let input = vec![
            vec![1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
            vec![1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
            vec![2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
            vec![3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
            vec![7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
            vec![1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
            vec![1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
            vec![3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
            vec![1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
            vec![2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
        ];

        assert_eq!(solve_part1(&input), 40);
    }

    #[test]
    fn test_solve_part_2() {
        let input = vec![
            vec![1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
            vec![1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
            vec![2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
            vec![3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
            vec![7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
            vec![1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
            vec![1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
            vec![3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
            vec![1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
            vec![2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
        ];

        assert_eq!(solve_part2(&input), 315);
    }
}

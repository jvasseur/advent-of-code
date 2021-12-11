use advent_of_code_2021::{parse_lines, read};
use nom::bytes::complete::take;
use nom::character::complete::u8;
use nom::combinator::map_parser;
use nom::multi::many0;
use nom::IResult;

fn line_parser(input: &str) -> IResult<&str, Vec<u8>> {
    many0(map_parser(take(1_u8), u8))(input)
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
        /*
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        */

        let mut neighbours = Vec::new();

        if i != self.height - 1 {
            neighbours.push((i + 1, j))
        }

        if i != self.height - 1 && j != self.width - 1 {
            neighbours.push((i + 1, j + 1))
        }

        if j != self.width - 1 {
            neighbours.push((i, j + 1))
        }

        if i != 0 && j != self.width - 1 {
            neighbours.push((i - 1, j + 1))
        }

        if i != 0 {
            neighbours.push((i - 1, j))
        }

        if i != 0 && j != 0 {
            neighbours.push((i - 1, j - 1))
        }

        if j != 0 {
            neighbours.push((i, j - 1))
        }

        if i != self.height - 1 && j != 0 {
            neighbours.push((i + 1, j - 1))
        }

        neighbours
    }

    fn cycle(&mut self) -> u32 {
        let mut flashes = 0;

        for i in 0..self.height {
            for j in 0..self.width {
                flashes += self.inc((i, j));
            }
        }

        for i in 0..self.height {
            for j in 0..self.width {
                if self.points[i][j] > 9 {
                    self.points[i][j] = 0;
                }
            }
        }

        return flashes;
    }

    fn inc(&mut self, point: Point) -> u32 {
        let mut flashes = 0;

        self.points[point.0][point.1] += 1;

        if self.points[point.0][point.1] == 10 {
            flashes += 1;

            for neighbour in self.neighbours(point) {
                flashes += self.inc(neighbour);
            }
        }

        flashes
    }
}

fn solve_part1(input: &[Vec<u8>]) -> u32 {
    let mut map = Map::new(input);
    let mut flashes = 0;

    for _ in 0..100 {
        flashes += map.cycle();
    }

    flashes
}

fn solve_part2(input: &[Vec<u8>]) -> u32 {
    let mut map = Map::new(input);

    let count = (map.height * map.width) as u32;
    let mut cycle = 0;

    loop {
        let flashes = map.cycle();

        cycle += 1;

        if flashes == count {
            return cycle;
        }
    }
}

fn main() {
    let input = read(11);

    let parsed_input = parse_lines(line_parser, &input);

    println!("{}", solve_part1(&parsed_input));
    println!("{}", solve_part2(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::line_parser;
    use super::solve_part1;
    use super::solve_part2;
    use super::Map;

    #[test]
    fn test_line_parser() {
        assert_eq!(
            line_parser("5483143223"),
            Ok(("", (vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3])))
        );
    }

    #[test]
    fn test_cycle_1() {
        let input = vec![
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ];

        let mut map = Map::new(&input);

        map.cycle();

        assert_eq!(
            map.points,
            vec![
                vec![6, 5, 9, 4, 2, 5, 4, 3, 3, 4],
                vec![3, 8, 5, 6, 9, 6, 5, 8, 2, 2],
                vec![6, 3, 7, 5, 6, 6, 7, 2, 8, 4],
                vec![7, 2, 5, 2, 4, 4, 7, 2, 5, 7],
                vec![7, 4, 6, 8, 4, 9, 6, 5, 8, 9],
                vec![5, 2, 7, 8, 6, 3, 5, 7, 5, 6],
                vec![3, 2, 8, 7, 9, 5, 2, 8, 3, 2],
                vec![7, 9, 9, 3, 9, 9, 2, 2, 4, 5],
                vec![5, 9, 5, 7, 9, 5, 9, 6, 6, 5],
                vec![6, 3, 9, 4, 8, 6, 2, 6, 3, 7],
            ]
        );
    }

    #[test]
    fn test_solve_part_1() {
        let input = vec![
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ];

        assert_eq!(solve_part1(&input), 1656);
    }

    #[test]
    fn test_solve_part_2() {
        let input = vec![
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ];

        assert_eq!(solve_part2(&input), 195);
    }
}

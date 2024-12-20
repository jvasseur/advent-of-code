use advent_of_code_2024::{dijkstra, grid::{Direction, Grid, Point}, parser::*, read};
use nom::{bytes::complete::tag, character::complete::none_of, multi::many1, sequence::terminated, IResult};

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    map: Grid<bool>,
    start: Point,
    end: Point,
}

impl Input {
    fn new(map: Grid<bool>, start: Point, end: Point) -> Self {
        Self { map, start, end }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, grid) = many1(terminated(many1(none_of("\n")), tag("\n")))(input)?;

        let mut map = Grid::new_fill(grid.len(), grid[0].len(), false);
        let mut start = None;
        let mut end = None;

        for point in map.points() {
            match grid[point.row as usize][point.col as usize] {
                '#' => {
                    map.set(&point, false);
                },
                '.' => {
                    map.set(&point, true);
                },
                'S' => {
                    start = Some(point);
                    map.set(&point, true);
                },
                'E' => {
                    end = Some(point);
                    map.set(&point, true);
                },
                _ => {
                    panic!("Invalid map data");
                }
            }
        }

        Ok((input, Input::new(map, start.unwrap(), end.unwrap())))
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

fn get_path(input: &Input) -> Vec<Point> {
    dijkstra::get_paths(
        [input.start],
        |point| {
            let mut edges = Vec::new();

            for direction in DIRECTIONS {
                let next = point + direction * 1;

                if *input.map.get(&next) {
                    edges.push(dijkstra::Edge {
                        node: next,
                        cost: 1,
                    });
                }
            }

            edges
        },
        |point| point == &input.end,
    )[0].clone()
}

fn get_cheats(input: &Input, cheat_max_len: u32) -> Vec<usize> {
    let path = get_path(&input);

    let mut cheats = Vec::new();

    for (start_index, start) in path.iter().enumerate() {
        for (end_index, end) in path.iter().enumerate() {
            if end_index <= start_index + 1 {
                // No need to cheat to a point that is before, the same point or ne next point
                continue;
            }

            // I tried doing a second dijkstra here, but that's dumb, the best distance in a space without obstacles
            // is the Manhattan distance between them
            let best_path_len = start.row.abs_diff(end.row) +  start.col.abs_diff(end.col);

            if best_path_len <= cheat_max_len {
                if end_index > start_index + (best_path_len as usize) {
                    cheats.push(end_index - start_index - (best_path_len as usize));
                }
            }
        }
    }

    cheats
}

fn solve_part1(input: &Input) -> usize {
    get_cheats(&input, 2).into_iter().filter(|&cost| cost >= 100).count()
}

fn solve_part2(input: &Input) -> usize {
    get_cheats(&input, 20).into_iter().filter(|&cost| cost >= 100).count()
}

fn main() {
    let input = parse(&read(20).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    fn parsed_input() -> Input {
        parse::<Input>(INPUT).unwrap()
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse::<Input>(INPUT).unwrap().start, Point::new(3, 1));
        assert_eq!(parse::<Input>(INPUT).unwrap().end, Point::new(7, 5));
    }

    #[test]
    fn test_get_cheats_1() {
        let cheats = get_cheats(&parsed_input(), 2);

        assert_eq!(cheats.iter().filter(|&&len| len == 2).count(), 14);
        assert_eq!(cheats.iter().filter(|&&len| len == 4).count(), 14);
    }

    #[test]
    fn test_get_cheats_2() {
        let cheats = get_cheats(&parsed_input(), 20);

        assert_eq!(cheats.iter().filter(|&&len| len == 50).count(), 32);
        assert_eq!(cheats.iter().filter(|&&len| len == 52).count(), 31);
    }
}

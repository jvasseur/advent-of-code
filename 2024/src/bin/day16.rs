use advent_of_code_2024::{dijkstra::{self, Edge}, grid::{Direction, Grid, Point}, parser::*, read};
use nom::IResult;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    map: Grid<bool>,
    start: (Point, Direction),
    end: Point,
}

impl Input {
    fn new(map: impl Into<Grid<bool>>, start: (Point, Direction), end: Point) -> Self {
        Self { map: map.into(), start, end }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, grid) = grid_parser::<char>(input)?;

        let mut map = Grid::new_fill(grid.rows(), grid.cols(), false);
        let mut start = None;
        let mut end = None;

        for point in grid.points() {
            match grid.get(&point) {
                '#' => map.set(&point, true),
                '.' => map.set(&point, false),
                'S' => start = Some((point, Direction::Right)),
                'E' => end = Some(point),
                _ => panic!("Invalid map"),
            }
        }

        Ok((input, Input::new(map, start.unwrap(), end.unwrap())))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct Position<'a> {
    grid: &'a Grid<bool>,
    position: (Point, Direction),
}

impl<'a> dijkstra::Node for Position<'a> {
    fn edges(&self) -> Vec<dijkstra::Edge<Self>> {
        let mut edges = Vec::new();

        let advanced = self.position.0 + self.position.1 * 1;

        if !self.grid.get(&advanced) {
            edges.push(Edge {
                node: Position { grid: self.grid, position: (advanced, self.position.1) },
                cost: 1,
            });
        }

        match self.position.1 {
            Direction::Up | Direction::Down => {
                for direction in [Direction::Left, Direction::Right] {
                    edges.push(Edge {
                        node: Position { grid: self.grid, position: (self.position.0, direction) },
                        cost: 1000,
                    });
                }
            },
            Direction::Left |Direction::Right => {
                for direction in [Direction::Up, Direction::Down] {
                    edges.push(Edge {
                        node: Position { grid: self.grid, position: (self.position.0, direction) },
                        cost: 1000,
                    });
                }
            },
            _ => panic!("Here be dragons"),

        }

        edges
    }
}

fn solve_part1(input: &Input) -> u32 {
    dijkstra::shortest_path([Position {
        grid: &input.map,
        position: input.start.clone(),
    }], |position| position.position.0 == input.end).unwrap()
}

fn solve_part2(input: &Input) -> usize {
    0
}

fn main() {
    let input = parse(&read(16).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    fn parsed_input() -> Input {
        parse(INPUT).unwrap()
    }

    #[test]
    fn test_parser() {
        let parsed = parsed_input();

        assert_eq!(parsed.start, (Point::new(13, 1), Direction::Right));
        assert_eq!(parsed.end, Point::new(1, 13));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 7036);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 0);
    }
}

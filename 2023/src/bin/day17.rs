use advent_of_code_2023::{read, Parsable, util::{Grid, Point, dijkstra}};
use nom::{IResult, multi::many0, combinator::{map, map_parser}, sequence::terminated, bytes::complete::{tag, take}, character::complete::u32};

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    grid: Grid<u32>,
}

impl Input {
    fn new(grid: impl Into<Grid<u32>>) -> Self {
        Self { grid: grid.into() }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        map(many0(terminated(many0(map_parser(take(1_usize), u32)), tag("\n"))), Input::new)(input)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Node<'a> {
    grid: &'a Grid<u32>,
    point: Point,
    direction: Direction,
    step: u8,
}

impl<'a> dijkstra::Node for Node<'a> {
    fn edges(&self) -> Vec<dijkstra::Edge<Self>> {
        let mut directions = match self.direction {
            Direction::Up | Direction::Down => vec![(Direction::Left, 1), (Direction::Right, 1)],
            Direction::Left | Direction::Right => vec![(Direction::Up, 1), (Direction::Down, 1)],
        };

        if self.step < 3 {
            directions.push((self.direction, self.step + 1));
        }

        directions
            .into_iter()
            .filter_map(|(direction, step)| match direction {
                Direction::Up => if self.point.row > 0 {
                    Some(Node { grid: self.grid, point: self.point.up(1), direction, step })
                } else {
                    None
                },
                Direction::Down => if self.point.row < self.grid.rows() - 1 {
                    Some(Node { grid: self.grid, point: self.point.down(1), direction, step })
                } else {
                    None
                },
                Direction::Left => if self.point.col > 0 {
                    Some(Node { grid: self.grid, point: self.point.left(1), direction, step })
                } else {
                    None
                },
                Direction::Right => if self.point.col < self.grid.cols() - 1 {
                    Some(Node { grid: self.grid, point: self.point.right(1), direction, step })
                } else {
                    None
                },
            })
            .map(|node| dijkstra::Edge { node, cost: *self.grid.get(&node.point) })
            .collect()
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct UltraNode<'a> {
    grid: &'a Grid<u32>,
    point: Point,
    direction: Direction,
    step: u8,
}

impl<'a> dijkstra::Node for UltraNode<'a> {
    fn edges(&self) -> Vec<dijkstra::Edge<Self>> {
        let mut directions = Vec::new();

        if self.step >= 4 {
            match self.direction {
                Direction::Up | Direction::Down => {
                    directions.push((Direction::Left, 1));
                    directions.push((Direction::Right, 1));
                },
                Direction::Left | Direction::Right => {
                    directions.push((Direction::Up, 1));
                    directions.push((Direction::Down, 1));
                },
            };
        }

        if self.step < 10 {
            directions.push((self.direction, self.step + 1));
        }

        directions
            .into_iter()
            .filter_map(|(direction, step)| match direction {
                Direction::Up => if self.point.row > 0 {
                    Some(UltraNode { grid: self.grid, point: self.point.up(1), direction, step })
                } else {
                    None
                },
                Direction::Down => if self.point.row < self.grid.rows() - 1 {
                    Some(UltraNode { grid: self.grid, point: self.point.down(1), direction, step })
                } else {
                    None
                },
                Direction::Left => if self.point.col > 0 {
                    Some(UltraNode { grid: self.grid, point: self.point.left(1), direction, step })
                } else {
                    None
                },
                Direction::Right => if self.point.col < self.grid.cols() - 1 {
                    Some(UltraNode { grid: self.grid, point: self.point.right(1), direction, step })
                } else {
                    None
                },
            })
            .map(|node| dijkstra::Edge { node, cost: *self.grid.get(&node.point) })
            .collect()
    }
}

fn solve_part1(input: &Input) -> u32 {
    let start = Point { row: 0, col: 0 };
    let destination = Point { row: input.grid.rows() - 1, col: input.grid.cols() - 1 };

    dijkstra::shortest_path([
        Node { grid: &input.grid, point: start, direction: Direction::Right, step: 0 },
        Node { grid: &input.grid, point: start, direction: Direction::Down, step: 0 },
    ], |node| node.point == destination).unwrap()
}

fn solve_part2(input: &Input) -> u32 {
    let start = Point { row: 0, col: 0 };
    let destination = Point { row: input.grid.rows() - 1, col: input.grid.cols() - 1 };

    dijkstra::shortest_path([
        UltraNode { grid: &input.grid, point: start, direction: Direction::Right, step: 0 },
        UltraNode { grid: &input.grid, point: start, direction: Direction::Down, step: 0 },
    ], |node| node.point == destination && node.step >= 4).unwrap()
}

fn main() {
    let input = read(17);
    let parsed = Input::parse(&input).unwrap();

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

    fn parsed_input() -> Input {
        Input::new(vec![
            vec![2, 4, 1, 3, 4, 3, 2, 3, 1, 1, 3, 2, 3],
            vec![3, 2, 1, 5, 4, 5, 3, 5, 3, 5, 6, 2, 3],
            vec![3, 2, 5, 5, 2, 4, 5, 6, 5, 4, 2, 5, 4],
            vec![3, 4, 4, 6, 5, 8, 5, 8, 4, 5, 4, 5, 2],
            vec![4, 5, 4, 6, 6, 5, 7, 8, 6, 7, 5, 3, 6],
            vec![1, 4, 3, 8, 5, 9, 8, 7, 9, 8, 4, 5, 4],
            vec![4, 4, 5, 7, 8, 7, 6, 9, 8, 7, 7, 6, 6],
            vec![3, 6, 3, 7, 8, 7, 7, 9, 7, 9, 6, 5, 3],
            vec![4, 6, 5, 4, 9, 6, 7, 9, 8, 6, 8, 8, 7],
            vec![4, 5, 6, 4, 6, 7, 9, 9, 8, 6, 4, 5, 3],
            vec![1, 2, 2, 4, 6, 8, 6, 8, 6, 5, 5, 6, 3],
            vec![2, 5, 4, 6, 5, 4, 8, 8, 8, 7, 7, 3, 5],
            vec![4, 3, 2, 2, 6, 7, 4, 6, 5, 5, 5, 3, 3],
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(Input::parse(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 102);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 94);
    }
}

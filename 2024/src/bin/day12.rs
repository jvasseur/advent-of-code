use advent_of_code_2024::{grid::{Direction, Grid, Point}, parser::*, read};
use std::collections::HashSet;
use nom::IResult;

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Left,
    Direction::Down,
    Direction::Right,
];

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    grid: Grid<char>,
}

impl Input {
    fn new(grid: impl Into<Grid<char>>) -> Self {
        Self { grid: grid.into() }
    }

    fn get_group(&self) -> Vec<(char, HashSet<Point>)> {
        let mut groups = Vec::new();

        let mut remaining = self.grid.points().collect::<HashSet<_>>();

        while let Some(start) = remaining.iter().next().cloned() {
            remaining.remove(&start);

            let mut points_to_check = vec![start];

            let char = self.grid.get(&start);
            let mut points = HashSet::new();

            while let Some(point) = points_to_check.pop() {
                for direction in DIRECTIONS {
                    let point_to_check = point + direction * 1;

                    if self.grid.get(&point_to_check) != char || !remaining.contains(&point_to_check) {
                        continue;
                    }

                    remaining.remove(&point_to_check);
                    points_to_check.push(point_to_check);
                }

                points.insert(point);
            }

            groups.push((*char, points));
        }

        groups
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, grid) = grid_parser(input)?;

        Ok((input, Input::new(grid)))
    }
}

fn group_area(points: &HashSet<Point>) -> usize {
    points.len()
}

fn group_perimeter(points: &HashSet<Point>) -> usize {
    let mut perimeter = 0;

    for point in points {
        for direction in DIRECTIONS {
            if !points.contains(&(point + direction * 1)) {
                perimeter += 1;
            }
        }
    }

    perimeter
}

fn group_sides(points: &HashSet<Point>) -> usize {
    let mut edges = HashSet::new();

    for point in points {
        for direction in DIRECTIONS {
            if !points.contains(&(point + direction * 1)) {
                edges.insert((point.clone(), direction));
            }
        }
    }

    let mut sides = 0;

    while let Some(edge) = edges.iter().next().cloned() {
        edges.remove(&edge);

        let (point, direction) = edge;

        match direction {
            Direction::Up | Direction::Down => {
                for col in (0..point.col).rev() {
                    let edge_to_test = (Point::new(point.row, col), direction);

                    if !edges.remove(&edge_to_test) {
                        break;
                    }
                }

                for col in point.col + 1.. {
                    let edge_to_test = (Point::new(point.row, col), direction);

                    if !edges.remove(&edge_to_test) {
                        break;
                    }
                }
            },
            Direction::Left | Direction::Right => {
                for row in (0..point.row).rev() {
                    let edge_to_test = (Point::new(row, point.col), direction);

                    if !edges.remove(&edge_to_test) {
                        break;
                    }
                }

                for row in point.row + 1.. {
                    let edge_to_test = (Point::new(row, point.col), direction);

                    if !edges.remove(&edge_to_test) {
                        break;
                    }
                }
            },
            _ => panic!("Here be dragons"),
        }

        sides += 1;
    }

    sides
}

fn solve_part1(input: &Input) -> usize {
    input.get_group().iter().map(|(_, points)| group_area(points) * group_perimeter(points)).sum()
}

fn solve_part2(input: &Input) -> usize {
    input.get_group().iter().map(|(_, points)| group_area(points) * group_sides(points)).sum()
}

fn main() {
    let input = parse(&read(12).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    fn parsed_input() -> Input {
        Input::new(vec![
            vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'F', 'F'],
            vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'C', 'F'],
            vec!['V', 'V', 'R', 'R', 'R', 'C', 'C', 'F', 'F', 'F'],
            vec!['V', 'V', 'R', 'C', 'C', 'C', 'J', 'F', 'F', 'F'],
            vec!['V', 'V', 'V', 'V', 'C', 'J', 'J', 'C', 'F', 'E'],
            vec!['V', 'V', 'I', 'V', 'C', 'C', 'J', 'J', 'E', 'E'],
            vec!['V', 'V', 'I', 'I', 'I', 'C', 'J', 'J', 'E', 'E'],
            vec!['M', 'I', 'I', 'I', 'I', 'I', 'J', 'J', 'E', 'E'],
            vec!['M', 'I', 'I', 'I', 'S', 'I', 'J', 'E', 'E', 'E'],
            vec!['M', 'M', 'M', 'I', 'S', 'S', 'J', 'E', 'E', 'E'],
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse::<Input>(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_get_groups() {
        assert_eq!(parsed_input().get_group().len(), 11);
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 1930);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 1206);
    }
}

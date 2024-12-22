use std::{collections::HashSet, convert::identity};
use advent_of_code_2024::{grid::{Direction, Grid, Point}, parser::*, read};
use nom::{branch::alt, bytes::complete::tag, combinator::value, multi::many1, sequence::terminated, IResult};

#[derive(Clone, PartialEq, Eq, Debug)]
enum Obstacle {
    Box,
    Wall,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    grid: Grid<Option<Obstacle>>,
    position: Point,
    directions: Vec<Direction>,
}

impl Input {
    fn new(grid: impl Into<Grid<Option<Obstacle>>>, position: Point, directions: impl Into<Vec<Direction>>) -> Self {
        Self { grid: grid.into(), position, directions: directions.into() }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, data) = many1(
            terminated(
                many1(
                    alt((
                        value(None, tag("@")),
                        value(Some(None), tag(".")),
                        value(Some(Some(Obstacle::Box)), tag("O")),
                        value(Some(Some(Obstacle::Wall)), tag("#")),
                    ),
                )),
                tag("\n"),
            )
        )(input)?;
        let (input, _) = tag("\n")(input)?;
        let (input, directions) = many1(
            alt((
                value(Some(Direction::Up), tag("^")),
                value(Some(Direction::Left), tag("<")),
                value(Some(Direction::Down), tag("v")),
                value(Some(Direction::Right), tag(">")),
                value(None, tag("\n")),
            ))
        )(input)?;

        let grid = Grid::from(data.clone());
        let position = grid.points().find(|point| grid.get(point) == &None).unwrap();

        let clean_data = data.into_iter().map(|row| row.into_iter().map(|value| match value {
            None => None,
            Some(v) => v,
        }).collect::<Vec<_>>()).collect::<Vec<_>>();
        let clean_grid = Grid::from(clean_data);

        Ok((input, Input::new(
            clean_grid,
            position,
            directions.into_iter().filter_map(identity).collect::<Vec<_>>(),
        )))
    }
}

fn solve_part1(input: &Input) -> i32 {
    let mut grid = input.grid.clone();
    let mut position = input.position;

    'outer: for direction in &input.directions {
        let next_position = position + direction * 1;

        match grid.get(&next_position) {
            None => {
                position = next_position;
            },
            Some(Obstacle::Wall) => {},
            Some(Obstacle::Box) => {
                'inner: for i in 2.. {
                    let next_box_position = position + direction * i;

                    match grid.get(&next_box_position) {
                        None => {
                            grid.set(&next_position, None);
                            grid.set(&next_box_position, Some(Obstacle::Box));

                            position = next_position;

                            continue 'outer;
                        },
                        Some(Obstacle::Wall) => {
                            continue 'outer;
                        },
                        Some(Obstacle::Box) => {
                            continue 'inner;
                        }
                    }
                }
            }
        }
    }

    grid.points().filter(|point| grid.get(point) == &Some(Obstacle::Box)).map(|point| point.row * 100 + point.col).sum()
}

fn find_box(grid: &Grid<Option<Obstacle>>, point: &Point) -> (Point, Point) {
    for i in 1.. {
        if grid.get(&(point + Direction::Right * i)) != &Some(Obstacle::Box) {
            match i % 2 {
                0 => {
                    return (point.clone(), Point::new(point.row, point.col + 1))
                },
                1 => {
                    return (Point::new(point.row, point.col - 1), point.clone())
                },
                _ => panic!("Here be dragons"),
            }
        }
    };

    panic!("Here be dragons");
}

fn solve_part2(input: &Input) -> i32 {
    let mut grid = Grid::new_fill(input.grid.rows(), input.grid.cols() * 2, None);

    for point in input.grid.points() {
        let value = input.grid.get(&point);

        grid.set(&Point::new(point.row, point.col * 2), value.clone());
        grid.set(&Point::new(point.row, point.col * 2 + 1), value.clone());
    }

    let mut position = Point::new(input.position.row, input.position.col * 2);

    'outer: for direction in &input.directions {
        let next_position = position + direction * 1;

        match direction {
            Direction::Left | Direction::Right => {
                // Same as previous problem in this direction
                match grid.get(&next_position) {
                    None => {
                        position = next_position;
                    },
                    Some(Obstacle::Wall) => {},
                    Some(Obstacle::Box) => {
                        for i in 2.. {
                            let next_box_position = position + direction * i;

                            match grid.get(&next_box_position) {
                                None => {
                                    grid.set(&next_position, None);
                                    grid.set(&next_box_position, Some(Obstacle::Box));

                                    position = next_position;

                                    continue 'outer;
                                },
                                Some(Obstacle::Wall) => {
                                    continue 'outer;
                                },
                                Some(Obstacle::Box) => {
                                    continue;
                                }
                            }
                        }
                    },
                }
            },
            Direction::Down | Direction::Up => {
                let mut boxes_to_push: HashSet<(Point, Point)> = HashSet::new();
                let mut points_to_check = vec![next_position];

                while let Some(point_to_check) = points_to_check.pop() {
                    match grid.get(&point_to_check) {
                        None => {
                            continue;
                        },
                        Some(Obstacle::Wall) => {
                            continue 'outer;
                        },
                        Some(Obstacle::Box) => {
                            let box_to_move = find_box(&grid, &point_to_check);

                            if boxes_to_push.insert(box_to_move) {
                                points_to_check.push(box_to_move.0 + direction * 1);
                                points_to_check.push(box_to_move.1 + direction * 1);
                            }
                        },
                    }
                }

                for box_to_push in &boxes_to_push {
                    grid.set(&box_to_push.0, None);
                    grid.set(&box_to_push.1, None);
                }

                for box_to_push in &boxes_to_push {
                    assert_eq!(grid.get(&(box_to_push.0 + direction * 1)), &None);
                    assert_eq!(grid.get(&(box_to_push.1 + direction * 1)), &None);

                    grid.set(&(box_to_push.0 + direction * 1), Some(Obstacle::Box));
                    grid.set(&(box_to_push.1 + direction * 1), Some(Obstacle::Box));
                }

                position = next_position;
            },
            _ => panic!("Here be dragons")
        }

        // Display grid for debug
        // for row in 0..grid.rows() {
        //     for col in 0..grid.cols() {
        //         let point = Point::new(row as i32, col as i32);

        //         if point == position {
        //             print!("@");
        //         } else {
        //             if let Some(obstacle) = grid.get(&point) {
        //                 match obstacle {
        //                     Obstacle::Box => {
        //                         print!("O");
        //                     },
        //                     Obstacle::Wall => {
        //                         print!("#");
        //                     },
        //                 };
        //             } else {
        //                 print!(".");
        //             }
        //         }
        //     }

        //     print!("\n")
        // }

        // print!("\n")
    }

    let mut boxes: HashSet<(Point, Point)> = HashSet::new();

    for point in grid.points() {
        if grid.get(&point) == &Some(Obstacle::Box) {
            boxes.insert(find_box(&grid, &point));
        }
    }

    boxes.into_iter().map(|(point, _)| point.row * 100 + point.col).sum()
}

fn main() {
    let input = parse(&read(15).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

    fn parsed_input() -> Input {
        Input::new(
            Grid::from(vec![
                vec![Some(Obstacle::Wall), Some(Obstacle::Wall), Some(Obstacle::Wall), Some(Obstacle::Wall), Some(Obstacle::Wall), Some(Obstacle::Wall), Some(Obstacle::Wall), Some(Obstacle::Wall)],
                vec![Some(Obstacle::Wall), None, None, Some(Obstacle::Box), None, Some(Obstacle::Box), None, Some(Obstacle::Wall)],
                vec![Some(Obstacle::Wall), Some(Obstacle::Wall), None, None, Some(Obstacle::Box), None, None, Some(Obstacle::Wall)],
                vec![Some(Obstacle::Wall), None, None, None, Some(Obstacle::Box), None, None, Some(Obstacle::Wall)],
                vec![Some(Obstacle::Wall), None, Some(Obstacle::Wall), None, Some(Obstacle::Box), None, None, Some(Obstacle::Wall)],
                vec![Some(Obstacle::Wall), None, None, None, Some(Obstacle::Box), None, None, Some(Obstacle::Wall)],
                vec![Some(Obstacle::Wall), None, None, None, None, None, None, Some(Obstacle::Wall)],
                vec![Some(Obstacle::Wall), Some(Obstacle::Wall), Some(Obstacle::Wall), Some(Obstacle::Wall), Some(Obstacle::Wall), Some(Obstacle::Wall), Some(Obstacle::Wall), Some(Obstacle::Wall)],
            ]),
            Point::new(2, 2),
            vec![Direction::Left, Direction::Up, Direction::Up, Direction::Right, Direction::Right, Direction::Right, Direction::Down, Direction::Down, Direction::Left, Direction::Down, Direction::Right, Direction::Right, Direction::Down, Direction::Left, Direction::Left]
        )
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse::<Input>(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 2028);
    }

    const LARGER_INPUT: &str  = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    fn larger_parsed_input() -> Input {
        parse::<Input>(LARGER_INPUT).unwrap()
    }

    #[test]
    fn test_solve_part1_larger() {
        assert_eq!(solve_part1(&larger_parsed_input()), 10092);
    }

    #[test]
    fn test_solve_part2_larger() {
        assert_eq!(solve_part2(&larger_parsed_input()), 9021);
    }
}

use advent_of_code_2021::{parse, read};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::character::complete::u32;
use nom::combinator::value;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::multi::many0;
use nom::IResult;
use std::collections::HashSet;
use std::iter::IntoIterator;

type Point = (u32, u32);

#[derive(Clone,Debug,Eq,PartialEq)]
enum Axis {
    X,
    Y,
}

#[derive(Clone,Debug,Eq,PartialEq)]
struct Fold {
    axis: Axis,
    position: u32,
}

// Fold x
// x1 -> x2
// x1 - x = x - x2
// x1 - 2x = - x2
// x2 = 2x - x1
fn fold(x: u32, position: u32) -> u32 {
    2 * position - x
}

impl Fold {
    fn apply(&self, points: impl IntoIterator<Item = Point>) -> HashSet<Point> {
        let mut folded = HashSet::new();

        for (x, y) in points {
            folded.insert(match self.axis {
                Axis::X => {
                    if x <= self.position {
                        (x, y)
                    } else {
                        (fold(x, self.position), y)
                    }
                },
                Axis::Y => {
                    if y <= self.position {
                        (x, y)
                    } else {
                        (x, fold(y, self.position))
                    }
                }
            });
        }

        folded
    }
}

fn point_parser(input: &str) -> IResult<&str, Point> {
    separated_pair(u32, tag(","), u32)(input)
}

fn points_parser(input: &str) -> IResult<&str, Vec<Point>> {
    many0(terminated(point_parser, newline))(input)
}

fn fold_parser(input: &str) -> IResult<&str, Fold> {
    let (rest, (_, axis, _, position)) = tuple((
        tag("fold along "),
        alt((
            value(Axis::X, tag("x")),
            value(Axis::Y, tag("y")),
        )),
        tag("="),
        u32,
    ))(input)?;

    Ok((rest, Fold {
        axis,
        position,
    }))
}

fn folds_parser(input: &str) -> IResult<&str, Vec<Fold>> {
    many0(terminated(fold_parser, newline))(input)
}

fn parser(input: &str) -> IResult<&str, (Vec<Point>, Vec<Fold>)> {
    separated_pair(points_parser, newline, folds_parser)(input)
}

fn solve_part1((points, folds): &(Vec<Point>, Vec<Fold>)) -> usize {
    folds[0].apply(points.to_vec()).len()
}

fn solve_part2((points, folds): &(Vec<Point>, Vec<Fold>)) -> String {
    let mut points: HashSet<Point> = HashSet::from_iter(points.iter().cloned());

    for fold in folds {
        points = fold.apply(points);
    }

    let mut chars: Vec<Vec<char>> = Vec::new();
    for point in points {
        let x = point.0 as usize;
        let y = point.1 as usize;

        if chars.len() < y + 1 {
            chars.resize(y + 1, Vec::new());
        }
        if chars[y].len() < x + 1 {
            chars[y].resize(x +1 , '.');
        }

        chars[y][x] = '#';
    }

    chars
        .into_iter()
        .map(|line| format!("{}\n", line.into_iter().collect::<String>()))
        .collect()
}

fn main() {
    let input = read(13);

    let parsed_input = parse(parser, &input);

    println!("{}", solve_part1(&parsed_input));
    println!("{}", solve_part2(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::point_parser;
    use super::parser;
    use super::fold;
    use super::Fold;
    use super::Axis;
    use super::solve_part1;
    use super::solve_part2;

    #[test]
    fn test_point_parser() {
        assert_eq!(point_parser("956,525"), Ok(("", (956, 525))));
    }

    #[test]
    fn test_parser() {
        assert_eq!(parser("956,525\n\nfold along x=655\n"), Ok(("", (vec![(956, 525)], vec![Fold { axis: Axis::X, position: 655 }]))));
    }

    #[test]
    fn test_fold() {
        assert_eq!(fold(14, 7), 0);
    }

    #[test]
    fn test_solve_part_1() {
        let input = (vec![
            (6, 10),
            (0, 14),
            (9, 10),
            (0, 3),
            (10, 4),
            (4, 11),
            (6, 0),
            (6, 12),
            (4, 1),
            (0, 13),
            (10, 12),
            (3, 4),
            (3, 0),
            (8, 4),
            (1, 10),
            (2, 14),
            (8, 10),
            (9, 0),
        ], vec![
            Fold { axis: Axis::Y, position: 7 },
            Fold { axis: Axis::X, position: 5 },
        ]);

        assert_eq!(solve_part1(&input), 17);
    }

    #[test]
    fn test_solve_part_2() {
        let input = (vec![
            (6, 10),
            (0, 14),
            (9, 10),
            (0, 3),
            (10, 4),
            (4, 11),
            (6, 0),
            (6, 12),
            (4, 1),
            (0, 13),
            (10, 12),
            (3, 4),
            (3, 0),
            (8, 4),
            (1, 10),
            (2, 14),
            (8, 10),
            (9, 0),
        ], vec![
            Fold { axis: Axis::Y, position: 7 },
            Fold { axis: Axis::X, position: 5 },
        ]);

        assert_eq!(solve_part2(&input), "#####\n#...#\n#...#\n#...#\n#####\n");
    }
}

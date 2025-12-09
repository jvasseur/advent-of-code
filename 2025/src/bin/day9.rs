use std::{collections::{HashMap, HashSet}, ops::RangeInclusive};

use advent_of_code_2025::{grid::Point, parser::*, read};
use derive_more::IntoIterator;
use itertools::Itertools;
use nom::{IResult, bytes::complete::tag, character::complete::i32, combinator::map, multi::many1, sequence::{separated_pair, terminated}};

#[derive(Clone, Debug, PartialEq, Eq, IntoIterator)]
#[into_iterator(owned, ref, ref_mut)]
struct Input(Vec<Point>);

impl Input {
    fn new(values: impl Into<Vec<Point>>) -> Self {
        Self(values.into())
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        map(
            many1(
                terminated(
                    map(
                        separated_pair(
                            i32,
                            tag(","),
                            i32,
                        ),
                        |(a, b)| Point::new(a, b),
                    ),
                    tag("\n"),
                ),
            ),
            Input::new,
        )(input)
    }
}

fn square_size((a, b): (&Point, &Point)) -> u64 {
    (a.col.abs_diff(b.col) + 1) as u64 * (a.row.abs_diff(b.row) + 1) as u64
}

fn solve_part1(input: &Input) -> u64 {
    input.into_iter()
        .tuple_combinations()
        .map(square_size)
        .max()
        .unwrap()
}

type Range = RangeInclusive<i32>;
type Region = (Range, Range);

struct Regions {
    row_ranges: Vec<Range>,
    col_ranges: Vec<Range>,
    inside: HashMap<Region, bool>,
}

impl Regions {
    fn new(row_ranges: Vec<Range>, col_ranges: Vec<Range>) -> Self {
        Self {
            row_ranges,
            col_ranges,
            inside: HashMap::new(),
        }
    }

    fn to_row_ranges(&self, (a, b): (i32, i32)) -> Vec<Range> {
        let mut ranges = Vec::new();
        let min_row = std::cmp::min(a, b);
        let max_row = std::cmp::max(a, b);

        for row_range in &self.row_ranges {
            if min_row <= *row_range.start() && *row_range.end() <= max_row {
                ranges.push(row_range.clone());
            }
        }

        ranges
    }

    fn to_col_ranges(&self, (a, b): (i32, i32)) -> Vec<Range> {
        let mut ranges = Vec::new();
        let min_col = std::cmp::min(a, b);
        let max_col = std::cmp::max(a, b);

        for col_range in &self.col_ranges {
            if min_col <= *col_range.start() && *col_range.end() <= max_col {
                ranges.push(col_range.clone());
            }
        }

        ranges
    }

    fn is_known(&self, region: &Region) -> bool {
        self.inside.get(region).is_some()
    }

    fn is_inside(&self, region: &Region) -> Option<bool> {
        self.inside.get(region).cloned()
    }

    fn set_inside(&mut self, region: &Region) {
        assert!(self.row_ranges.contains(&region.0), "Invalid region rows");
        assert!(self.col_ranges.contains(&region.1), "Invalid region cols");

        self.inside.insert(region.clone(), true);
    }

    fn set_outside(&mut self, region: &Region) {
        assert!(self.row_ranges.contains(&region.0), "Invalid region rows");
        assert!(self.col_ranges.contains(&region.1), "Invalid region cols");

        self.inside.insert(region.clone(), false);
    }
}

fn solve_part2(input: &Input) -> u64 {
    let rows: Vec<i32> = input.into_iter().map(|point| point.row).sorted().unique().collect();
    let cols: Vec<i32> = input.into_iter().map(|point| point.col).sorted().unique().collect();

    let row_ranges: Vec<Range> = rows.iter().map(|&row| row..=row).interleave(rows.iter().cloned().tuple_windows().map(|(a, b)| a + 1..=b - 1)).collect();
    let col_ranges: Vec<Range> = cols.iter().map(|&col| col..=col).interleave(cols.iter().cloned().tuple_windows().map(|(a, b)| a + 1..=b - 1)).collect();

    let mut regions = Regions::new(row_ranges.clone(), col_ranges.clone());

    println!("Building frontier");

    for (a, b) in input.into_iter().circular_tuple_windows() {
        if a.col == b.col {
            for row_range in regions.to_row_ranges((a.row, b.row)) {
                regions.set_inside(&(row_range.clone(), a.col..=a.col));
            }
        }

        if a.row == b.row {
            for col_range in regions.to_col_ranges((a.col, b.col)) {
                regions.set_inside(&(a.row..=a.row, col_range.clone()));
            }
        }
    }

    println!("Computing exterior");

    let mut regions_to_check = HashSet::new();

    let first_cols = col_ranges.first().unwrap().clone();
    let last_cols = col_ranges.last().unwrap().clone();

    for row_range in &row_ranges {
        regions_to_check.insert((row_range.clone(), first_cols.clone()));
        regions_to_check.insert((row_range.clone(), last_cols.clone()));
    }

    let first_rows = row_ranges.first().unwrap().clone();
    let last_rows = row_ranges.last().unwrap().clone();

    for col_range in &col_ranges {
        regions_to_check.insert((first_rows.clone(), col_range.clone()));
        regions_to_check.insert((last_rows.clone(), col_range.clone()));
    }

    while let Some(region_to_check) = regions_to_check.iter().next().cloned() {
        regions_to_check.remove(&region_to_check);

        if regions.is_known(&region_to_check) {
            continue;
        }

        regions.set_outside(&region_to_check);

        let row_range_index = row_ranges.iter().position(|row_range| row_range == &region_to_check.0).unwrap();
        let col_range_index = col_ranges.iter().position(|col_range| col_range == &region_to_check.1).unwrap();

        if 0 < row_range_index {
            regions_to_check.insert((row_ranges[row_range_index - 1].clone(), region_to_check.1.clone()));
        }

        if row_range_index < row_ranges.len() - 1 {
            regions_to_check.insert((row_ranges[row_range_index + 1].clone(), region_to_check.1.clone()));
        }

        if 0 < col_range_index {
            regions_to_check.insert((region_to_check.0.clone(), col_ranges[col_range_index - 1].clone()));
        }

        if col_range_index < col_ranges.len() - 1 {
            regions_to_check.insert((region_to_check.0.clone(), col_ranges[col_range_index + 1].clone()));
        }
    }

    println!("Solving");

    input.into_iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b, square_size((a, b))))
        .sorted_by(|(_, _, a), (_, _, b)| b.cmp(a))
        .filter(|(a, b, _)| {
            for row_range in regions.to_row_ranges((a.row, b.row)) {
                for col_range in regions.to_col_ranges((a.col, b.col)) {
                    if !regions.is_inside(&(row_range.clone(), col_range.clone())).unwrap_or(true) {
                        return false;
                    }
                }
            }

            return true;
        })
        .map(|(_, _, size)| size)
        .next()
        .unwrap()
}

fn main() {
    let input = from_str(&read(9).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    fn parsed_input() -> Input {
        Input::new([
            Point::new(7, 1),
            Point::new(11, 1),
            Point::new(11, 7),
            Point::new(9, 7),
            Point::new(9, 5),
            Point::new(2, 5),
            Point::new(2, 3),
            Point::new(7, 3),
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse(INPUT), Ok(("", parsed_input())));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 50);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 24);
    }
}

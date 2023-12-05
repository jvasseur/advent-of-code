use std::ops::RangeInclusive;
use advent_of_code_2023::{read, Parsable};
use nom::bytes::complete::tag;
use nom::character::complete::{u64, alpha1};
use nom::IResult;
use nom::multi::{many1, separated_list1};
use itertools::Itertools;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Map {
    from: String,
    to: String,
    ranges: Vec<Range>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Range {
    destination_start: u64,
    source_start: u64,
    range: u64,
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("seeds: ")(input)?;
        let (input, seeds) = separated_list1(tag(" "), u64)(input)?;
        let (input, _) = tag("\n\n")(input)?;
        let (input, maps) = separated_list1(tag("\n"), Map::parser)(input)?;

        Ok((input, Input { seeds, maps }))
    }
}

impl Parsable for Map {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, from) = alpha1(input)?;
        let (input, _) = tag("-to-")(input)?;
        let (input, to) = alpha1(input)?;
        let (input, _) = tag(" map:\n")(input)?;
        let (input, ranges) = many1(Range::parser)(input)?;

        Ok((input, Map::new(from, to, ranges)))
    }
}

impl Parsable for Range {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, destination_start) = u64(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, source_start) = u64(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, range) = u64(input)?;
        let (input, _) = tag("\n")(input)?;

        Ok((input, Range::new(destination_start, source_start, range)))
    }
}

impl Input {
    fn seed_ranges(&self) -> Vec<RangeInclusive<u64>> {
        self.seeds.iter().tuples().map(|(&start, &length)| start..=start + length - 1).collect()
    }
}

impl Map {
    fn new(from: impl Into<String>, to: impl Into<String>, ranges: impl Into<Vec<Range>>) -> Self {
        Self { from: from.into(), to: to.into(), ranges: ranges.into() }
    }

    fn map(&self, value: u64) -> u64 {
        for range in &self.ranges {
            match range.map(value) {
                Some(value) => {
                    return value;
                },
                None => {},
            }
        }

        return value;
    }

    fn map_ranges(&self, ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
        let mut ranges_mapped = Vec::new();
        let mut ranges_to_map = ranges;

        for range in &self.ranges {
            let ranges_to_map_this_round = ranges_to_map;

            ranges_to_map = Vec::new();

            for range_to_map in ranges_to_map_this_round {
                let (done, remaining) = range.map_range(range_to_map);

                ranges_mapped.extend(done);
                ranges_to_map.extend(remaining);
            }
        }

        ranges_mapped.extend(ranges_to_map);

        ranges_mapped
    }
}

impl Range {
    fn new(destination_start: u64, source_start: u64, range: u64) -> Self {
        Self { destination_start, source_start, range }
    }

    fn source(&self) -> RangeInclusive<u64> {
        self.source_start..=self.source_start + self.range - 1
    }

    fn map(&self, value: u64) -> Option<u64> {
        if self.source().contains(&value) {
            Some(value + self.destination_start - self.source_start)
        } else {
            None
        }
    }

    fn map_range(&self, value: RangeInclusive<u64>) -> (Vec<RangeInclusive<u64>>, Vec<RangeInclusive<u64>>) {
        let (inside, outside) = self.split_range(value);

        let inside = inside
            .into_iter()
            .map(|range| range.start() + self.destination_start - self.source_start..=range.end() + self.destination_start - self.source_start)
            .collect();

        (inside, outside)
    }

    fn split_range(&self, value: RangeInclusive<u64>) -> (Vec<RangeInclusive<u64>>, Vec<RangeInclusive<u64>>) {
        let source = self.source();

        // Value outside source
        if value.end() < source.start() || source.end() < value.start() {
            return (vec![], vec![value]);
        }

        // Value inside source
        if source.start() <= value.start() && value.end() <= source.end() {
            return (vec![value], vec![]);
        }

        // Source inside value
        if value.start() <= source.start() && source.end() <= value.end(){
            return (vec![source.clone()], vec![*value.start()..=*source.start() - 1, source.end() + 1..=*value.end()]);
        }

        // value around source start
        if value.start() <= source.start() && source.start() <= value.end() {
            return (vec![*source.start()..=*value.end()], vec![*value.start()..=*source.start()-1]);
        }

        // value around source end
        if value.start() <= source.end() && source.end() <= value.end() {
            return (vec![*value.start()..=*source.end()], vec![source.end()+1..=*value.end()]);
        }

        panic!();
    }
}

fn solve_part1(input: &Input) -> u64 {
    input.seeds.iter().map(|seed| {
        let mut value = seed.to_owned();

        for map in &input.maps {
            value = map.map(value);
        }

        value
    }).min().unwrap()
}

fn solve_part2(input: &Input) -> u64 {
    let mut ranges = input.seed_ranges();

    for map in &input.maps {
        ranges = map.map_ranges(ranges);
    }

    ranges.into_iter().map(|range| *range.start()).min().unwrap()
}

fn main() {
    let input = read(5);

    let parsed = Input::parse(&input).unwrap();

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    fn parsed_input() -> Input {
        Input {
            seeds: vec![79, 14, 55, 13],
            maps: vec![
                Map::new("seed", "soil", [
                    Range::new(50, 98, 2),
                    Range::new(52, 50, 48),
                ]),
                Map::new("soil", "fertilizer", [
                    Range::new(0, 15, 37),
                    Range::new(37, 52, 2),
                    Range::new(39, 0, 15),
                ]),
                Map::new("fertilizer", "water", [
                    Range::new(49, 53, 8),
                    Range::new(0, 11, 42),
                    Range::new(42, 0, 7),
                    Range::new(57, 7, 4),
                ]),
                Map::new("water", "light", [
                    Range::new(88, 18, 7),
                    Range::new(18, 25, 70),
                ]),
                Map::new("light", "temperature", [
                    Range::new(45, 77, 23),
                    Range::new(81, 45, 19),
                    Range::new(68, 64, 13),
                ]),
                Map::new("temperature", "humidity", [
                    Range::new(0, 69, 1),
                    Range::new(1, 0, 69),
                ]),
                Map::new("humidity", "location", [
                    Range::new(60, 56, 37),
                    Range::new(56, 93, 4),
                ]),
            ],
        }
    }

    #[test]
    fn test_parser() {
        assert_eq!(Input::parse(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_seed_ranges() {
        assert_eq!(parsed_input().seed_ranges(), vec![
            79..=92,
            55..=67,
        ]);
    }

    #[test]
    fn test_map_range() {
        // Seed to soil, range 1
        assert_eq!(Range::new(50, 98, 2).map_range(79..=92), (
            vec![],
            vec![79..=92],
        ));
        assert_eq!(Range::new(52, 50, 48).map_range(79..=92), (
            vec![81..=94],
            vec![],
        ));

        // Seed to soil, range 2
        assert_eq!(Range::new(50, 98, 2).map_range(55..=67), (
            vec![],
            vec![55..=67],
        ));
        assert_eq!(Range::new(52, 50, 48).map_range(55..=67), (
            vec![57..=69],
            vec![],
        ));

        assert_eq!(Range::new(55, 50, 50).map_range(55..=60), (
            vec![60..=65],
            vec![],
        ), "Completly inside");

        assert_eq!(Range::new(55, 50, 50).map_range(45..=105), (
            vec![55..=104],
            vec![45..=49, 100..=105],
        ), "Englobing");
    }

    #[test]
    fn test_map_ranges() {
        // Seed to soil
        assert_eq!(Map::new("seed", "soil", [
            Range::new(50, 98, 2),
            Range::new(52, 50, 48),
        ]).map_ranges(vec![79..=92, 55..=67]), vec![
            81..=94,
            57..=69,
        ]);
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 35);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 46);
    }
}

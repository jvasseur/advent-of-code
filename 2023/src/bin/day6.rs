use advent_of_code_2023::{read, Parsable};
use nom::bytes::complete::tag;
use nom::character::complete::{u64, space1};
use nom::IResult;
use nom::multi::separated_list1;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    races: Vec<Race>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Race {
    time: u64,
    record: u64,
}

impl Input {
    fn combined_race(&self) -> Race {
        let mut time = String::new();
        let mut record = String::new();

        for race in &self.races {
            time = format!("{}{}", time, race.time);
            record = format!("{}{}", record, race.record);
        }

        Race::new(time.parse().unwrap(), record.parse().unwrap())
    }
}

impl Race {
    fn new(time: u64, record: u64) -> Self {
        Race { time, record }
    }

    fn distance(&self, hold_time: u64) -> u64 {
        let speed = hold_time;
        let move_time = self.time - hold_time;

        speed * move_time
    }

    fn winnable_starts(&self) -> u64 {
        let first = (1..self.time)
            .find(|&hold_time| self.distance(hold_time) > self.record)
            .unwrap();

        let last = (1..self.time).rev()
            .find(|&hold_time| self.distance(hold_time) > self.record)
            .unwrap();

        last - first + 1
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("Time:")(input)?;
        let (input, _) = space1(input)?;
        let (input, times) = separated_list1(space1, u64)(input)?;
        let (input, _) = tag("\n")(input)?;

        let (input, _) = tag("Distance:")(input)?;
        let (input, _) = space1(input)?;
        let (input, distances) = separated_list1(space1, u64)(input)?;
        let (input, _) = tag("\n")(input)?;

        let races = times.into_iter().zip(distances.into_iter()).map(|(time, distance)| Race::new(time, distance)).collect();

        Ok((input, Input { races }))
    }
}

fn solve_part1(input: &Input) -> u64 {
    input.races.iter()
        .map(|race| race.winnable_starts())
        .product()
}

fn solve_part2(input: &Input) -> u64 {
    input.combined_race().winnable_starts()
}

fn main() {
    let input = read(6);

    let parsed = Input::parse(&input).unwrap();

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200
";

    fn parsed_input() -> Input {
        Input {
            races: vec![
                Race:: new(7, 9),
                Race:: new(15, 40),
                Race:: new(30, 200),
            ],
        }
    }

    #[test]
    fn test_parser() {
        assert_eq!(Input::parse(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 288);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 71503);
    }
}

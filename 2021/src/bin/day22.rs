use advent_of_code_2021::{parse_lines, read};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::combinator::value;
use nom::sequence::separated_pair;
use nom::IResult;
use std::ops::RangeInclusive;
use itertools::Itertools;

fn range_parser(input: &str) -> IResult<&str, Range> {
    let (input, (start, end)) = separated_pair(i32, tag(".."), i32)(input)?;

    Ok((input, Range { start, end }))
}

fn instruction_parser(input: &str) -> IResult<&str, Instruction> {
    let (input, switch) = alt((
        value(Switch::On, tag("on")),
        value(Switch::Off, tag("off")),
    ))(input)?;

    let (input, _) = tag(" x=")(input)?;

    let (input, x) = range_parser(input)?;

    let (input, _) = tag(",y=")(input)?;

    let (input, y) = range_parser(input)?;

    let (input, _) = tag(",z=")(input)?;

    let (input, z) = range_parser(input)?;

    Ok((input, Instruction {
        switch,
        region: Region {
            x,
            y,
            z,
        },
    }))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Switch {
    On,
    Off,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Range {
    start: i32,
    end: i32,
}

impl From<RangeInclusive<i32>> for Range {
    fn from(range: RangeInclusive<i32>) -> Self {
        Range::new(*range.start(), *range.end())
    }
}

impl Range {
    fn new(start: i32, end: i32) -> Self {
        if end < start {
            panic!("Invalid range: start={} end={}", start, end);
        }

        Range {
            start,
            end,
        }
    }

    fn len(&self) -> usize {
        (self.end - self.start + 1) as usize
    }

    fn include(&self, other: &Range) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    fn split(&self, other: &Range) -> Vec<Range> {
        // Included
        if other.include(self) {
            return vec![self.to_owned()];
        }

        // Disjoinct
        if other.end < self.start || self.end < other.start {
            return vec![self.to_owned()];
        }

        // Equal
        if self == other {
            return vec![self.to_owned()];
        }

        // Both inside
        if self.start <= other.start && other.end <= self.end {
            if self.start == other.start {
                return vec![
                    Range::new(other.start, other.end),
                    Range::new(other.end + 1, self.end)
                ]
            }

            if self.end == other.end {
                return vec![
                    Range::new(self.start, other.start - 1),
                    Range::new(other.start, other.end),
                ]
            }

            return vec![
                Range::new(self.start, other.start - 1),
                Range::new(other.start, other.end),
                Range::new(other.end + 1, self.end),
            ]
        }

        // End inside
        if self.start <= other.end && other.end <= self.end {
            return vec![
                Range::new(self.start, other.end),
                Range::new(other.end + 1, self.end),
            ]
        }

        // Start inside
        if self.start <= other.start && other.start <= self.end {
            return vec![
                Range::new(self.start, other.start - 1),
                Range::new(other.start, self.end),
            ]
        }

        panic!("Here be dragons");
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Region {
    x: Range,
    y: Range,
    z: Range,
}

impl Region {
    fn new(x: impl Into<Range>, y: impl Into<Range>, z: impl Into<Range>) -> Self {
        Region {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    fn size(&self) -> usize {
        self.x.len() * self.y.len() * self.z.len()
    }

    fn include(&self, other: &Region) -> bool {
        self.x.include(&other.x) && self.y.include(&other.y) && self.z.include(&other.z)
    }

    fn split_by(&self, other: &Region) -> Vec<Region> {
        let ranges_x = self.x.split(&other.x);
        let ranges_y = self.y.split(&other.y);
        let ranges_z = self.z.split(&other.z);

        [ranges_x, ranges_y, ranges_z]
            .into_iter()
            .multi_cartesian_product()
            .map(|ranges| Region::new(ranges[0], ranges[1], ranges[2]))
            .collect()

    }

    fn remove(&self, other: &Region) -> Vec<Region> {
        self.split_by(other)
            .into_iter()
            .filter(|region| !other.include(region))
            .collect()
    }
}

impl std::fmt::Debug for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "x={}..{},y={}..{},z={}..{}",
            self.x.start,
            self.x.end,
            self.y.start,
            self.y.end,
            self.z.start,
            self.z.end,
        )
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Instruction {
    switch: Switch,
    region: Region,
}

impl Instruction {
    fn new(switch: Switch, x: impl Into<Range>, y: impl Into<Range>, z: impl Into<Range>) -> Self {
        Instruction {
            switch,
            region: Region::new(x, y, z),
        }
    }
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {:?}",
            match self.switch {
                Switch::On => "on",
                Switch::Off => "off",
            },
            self.region,
        )
    }
}

fn apply(input: &[Instruction]) -> Vec<Region> {
    let mut regions: Vec<Region> = Vec::new();

    for (i, instruction) in input.iter().enumerate() {
        println!("{}", i);

        let mut new_regions: Vec<Region> = Vec::new();

        for region in regions {
            new_regions.extend(region.remove(&instruction.region).into_iter());
        }

        if instruction.switch == Switch::On {
            new_regions.push(instruction.region.clone());
        }

        regions = new_regions;

        println!("{}", regions.len());
    }

    regions
}

fn count(regions: &[Region]) -> usize {
    regions.into_iter().map(|region| region.size()).sum()
}

fn solve(input: &[Instruction]) -> (usize, usize) {
    let square = Region::new(-50..=50, -50..=50, -50..=50);

    let regions = apply(input);

    let filtered = regions
        .iter()
        .map(|region| {
            region
                .split_by(&square)
                .into_iter()
                .filter(|region| square.include(region))
                .collect::<Vec<Region>>()
        })
        .flatten()
        .collect::<Vec<Region>>();

    (count(&filtered), count(&regions))
}

fn main() {
    let input = read(22);

    let parsed_input = parse_lines(instruction_parser, &input);

    let (part1, part2) = solve(&parsed_input);

    println!("{}", part1);
    println!("{}", part2);
}

#[cfg(test)]
mod tests {
    use super::apply;
    use super::instruction_parser;
    use super::Switch;
    use super::Instruction;
    use super::Range;
    use super::Region;
    use super::solve;

    #[test]
    fn test_parser() {
        assert_eq!(
            instruction_parser("on x=-20..26,y=-36..17,z=-47..7"),
            Ok(("", Instruction::new(Switch::On, -20..=26, -36..=17, -47..=7))),
        );
    }

    #[test]
    fn test_len() {
        assert_eq!(Range::new(0, 12).len(), 13);
        assert_eq!(Range::new(-2, 2).len(), 5);
        assert_eq!(Range::new(-10, -1).len(), 10);
    }

    #[test]
    fn test_size() {
        assert_eq!(Region::new(0..=12, 0..=0, 0..=1).size(), 26);
        assert_eq!(Region::new(10..=12, 10..=12, 10..=12).size(), 27);
    }

    #[test]
    fn test_split() {
        assert_eq!(Range::new(0, 12).split(&Range::new(0, 12)), vec![
            Range::new(0, 12),
        ]);

        assert_eq!(Range::new(0, 12).split(&Range::new(-2, 14)), vec![
            Range::new(0, 12),
        ]);

        assert_eq!(Range::new(0, 12).split(&Range::new(4, 14)), vec![
            Range::new(0, 3),
            Range::new(4, 12),
        ]);

        assert_eq!(Range::new(0, 12).split(&Range::new(4, 12)), vec![
            Range::new(0, 3),
            Range::new(4, 12),
        ]);

        assert_eq!(Range::new(0, 12).split(&Range::new(-10, 4)), vec![
            Range::new(0, 4),
            Range::new(5, 12),
        ]);

        assert_eq!(Range::new(0, 12).split(&Range::new(-10, -4)), vec![
            Range::new(0, 12),
        ]);
    }

    #[test]
    fn test_split_2() {
        assert_eq!(Range::new(11, 12).split(&Range::new(9, 11)), vec![
            Range::new(11, 11),
            Range::new(12, 12),
        ]);
    }

    #[test]
    fn test_region_remove_0() {
        let this = Region::new(0..=10, 0..=10, 0..=10);
        let other = Region::new(15..=16, 15..=16, 16..=16);

        assert_eq!(this.remove(&other), vec![
            Region::new(0..=10, 0..=10, 0..=10),
        ]);
    }

    #[test]
    fn test_region_remove_1() {
        let this = Region::new(0..=10, 0..=10, 0..=10);
        let other = Region::new(0..=10, 0..=10, 5..=15);

        assert_eq!(this.remove(&other), vec![
            Region::new(0..=10, 0..=10, 0..=4),
        ]);
    }

    #[test]
    fn test_region_remove_2() {
        let this = Region::new(0..=10, 0..=10, 0..=10);
        let other = Region::new(0..=10, 5..=15, 5..=15);

        assert_eq!(this.remove(&other), vec![
            Region::new(0..=10, 0..=4, 0..=4),
            Region::new(0..=10, 0..=4, 5..=10),
            Region::new(0..=10, 5..=10, 0..=4),
        ]);
    }

    #[test]
    fn test_region_remove_3() {
        let this = Region::new(0..=10, 0..=10, 0..=10);
        let other = Region::new(0..=10, 0..=10, 5..=6);

        assert_eq!(this.remove(&other), vec![
            Region::new(0..=10, 0..=10, 0..=4),
            Region::new(0..=10, 0..=10, 7..=10),
        ]);
    }

    #[test]
    fn test_region_remove_4() {
        let this = Region::new(0..=10, 0..=10, 0..=10);
        let other = Region::new(0..=10, 5..=6, 5..=6);

        assert_eq!(this.remove(&other), vec![
            Region::new(0..=10, 0..=4, 0..=4),
            Region::new(0..=10, 0..=4, 5..=6),
            Region::new(0..=10, 0..=4, 7..=10),
            Region::new(0..=10, 5..=6, 0..=4),
            Region::new(0..=10, 5..=6, 7..=10),
            Region::new(0..=10, 7..=10, 0..=4),
            Region::new(0..=10, 7..=10, 5..=6),
            Region::new(0..=10, 7..=10, 7..=10),
        ]);
    }

    #[test]
    fn test_apply_1_1() {
        let instructions = vec![
            Instruction::new(Switch::On, -20..=26, -36..=17, -47..=7),
            Instruction::new(Switch::On, -20..=33, -21..=23, -26..=28),
        ];

        assert_eq!(apply(&instructions), vec![
            Region::new(-20..=26, -36..=-22, -47..=-27),
            Region::new(-20..=26, -36..=-22, -26..=7),
            Region::new(-20..=26, -21..=17, -47..=-27),
            Region::new(-20..=33, -21..=23, -26..=28),
        ]);
    }

    #[test]
    fn test_apply_1_2() {
        let instructions = vec![
            Instruction::new(Switch::On, -20..=26, -36..=17, -47..=7),
            Instruction::new(Switch::On, -20..=33, -21..=23, -26..=28),
            Instruction::new(Switch::On, -22..=28, -29..=23, -38..=16),
        ];

        assert_eq!(apply(&instructions), vec![
            // Region::new(-20..=26, -36..=-22, -47..=-27),
            // -20..26
            // -36..-30, -29..-22
            // -47..-39, -38..-27
            Region::new(-20..=26, -36..=-30, -47..=-39),
            Region::new(-20..=26, -36..=-30, -38..=-27),
            Region::new(-20..=26, -29..=-22, -47..=-39),

            // Region::new(-20..=26, -36..=-22, -26..=7),
            // -20..26
            // -36..-30, -29..-22
            // -26..7
            Region::new(-20..=26, -36..=-30, -26..=7),

            // Region::new(-20..=26, -21..=17, -47..=-27),
            // -20..26
            // -21..17
            // -47..-39, -38..-27
            Region::new(-20..=26, -21..=17, -47..=-39),

            // Region::new(-20..=33, -21..=23, -26..=28),
            // -20..28, 29..33
            // -21..23
            // -26..16, 17..28
            Region::new(-20..=28, -21..=23, 17..=28),
            Region::new(29..=33, -21..=23, -26..=16),
            Region::new(29..=33, -21..=23, 17..=28),

            Region::new(-22..=28, -29..=23, -38..=16),
        ]);
    }

    #[test]
    fn test_0_apply_1() {
        let instructions = vec![
            Instruction::new(Switch::On, 10..=12, 10..=12, 10..=12),
            Instruction::new(Switch::On, 11..=13, 11..=13, 11..=13),
        ];

        assert_eq!(apply(&instructions), vec![
            //Region::new(10..=12, 10..=12, 10..=12),
            // 10..10, 11..12
            // 10..10, 11..12
            // 10..10, 11..12
            Region::new(10..=10, 10..=10, 10..=10),
            Region::new(10..=10, 10..=10, 11..=12),
            Region::new(10..=10, 11..=12, 10..=10),
            Region::new(10..=10, 11..=12, 11..=12),
            Region::new(11..=12, 10..=10, 10..=10),
            Region::new(11..=12, 10..=10, 11..=12),
            Region::new(11..=12, 11..=12, 10..=10),

            Region::new(11..=13, 11..=13, 11..=13),
        ]);
    }

    #[test]
    fn test_0_apply_2() {
        let instructions = vec![
            Instruction::new(Switch::On, 10..=12, 10..=12, 10..=12),
            Instruction::new(Switch::On, 11..=13, 11..=13, 11..=13),
            Instruction::new(Switch::Off, 9..=11, 9..=11, 9..=11),
        ];

        assert_eq!(apply(&instructions), vec![
            //Region::new(10..=10, 10..=10, 10..=10),

            //Region::new(10..=10, 10..=10, 11..=12),
            // 10..10
            // 10..10
            // 11..11, 12..12
            Region::new(10..=10, 10..=10, 12..=12),

            //Region::new(10..=10, 11..=12, 10..=10),
            // 10..10
            // 11..11, 12..12
            // 10..10
            Region::new(10..=10, 12..=12, 10..=10),

            //Region::new(10..=10, 11..=12, 11..=12),
            // 10..10
            // 11..11, 12..12
            // 11..11, 12..12
            Region::new(10..=10, 11..=11, 12..=12),
            Region::new(10..=10, 12..=12, 11..=11),
            Region::new(10..=10, 12..=12, 12..=12),

            //Region::new(11..=12, 10..=10, 10..=10),
            Region::new(12..=12, 10..=10, 10..=10),

            //Region::new(11..=12, 10..=10, 11..=12),
            // 11..11, 12..12
            // 10..10
            // 11..11, 12..12
            Region::new(11..=11, 10..=10, 12..=12),
            Region::new(12..=12, 10..=10, 11..=11),
            Region::new(12..=12, 10..=10, 12..=12),

            //Region::new(11..=12, 11..=12, 10..=10),
            // 11..11, 12..12
            // 11..11, 12..12
            // 10..10
            Region::new(11..=11, 12..=12, 10..=10),
            Region::new(12..=12, 11..=11, 10..=10),
            Region::new(12..=12, 12..=12, 10..=10),

            //Region::new(11..=13, 11..=13, 11..=13),
            // 11..11, 12..13
            // 11..11, 12..13
            // 11..11, 12..13
            Region::new(11..=11, 11..=11, 12..=13),
            Region::new(11..=11, 12..=13, 11..=11),
            Region::new(11..=11, 12..=13, 12..=13),
            Region::new(12..=13, 11..=11, 11..=11),
            Region::new(12..=13, 11..=11, 12..=13),
            Region::new(12..=13, 12..=13, 11..=11),
            Region::new(12..=13, 12..=13, 12..=13),
        ]);
    }

    #[test]
    fn test_0_solve_1() {
        assert_eq!(solve(&vec![
            Instruction::new(Switch::On, 10..=12, 10..=12, 10..=12),
            Instruction::new(Switch::On, 11..=13, 11..=13, 11..=13),
        ]).0, 46);
    }

    #[test]
    fn test_0_solve_2() {
        assert_eq!(solve(&vec![
            Instruction::new(Switch::On, 10..=12, 10..=12, 10..=12),
            Instruction::new(Switch::On, 11..=13, 11..=13, 11..=13),
            Instruction::new(Switch::Off, 9..=11, 9..=11, 9..=11),
        ]).0, 38);
    }

    #[test]
    fn test_0_solve_3() {
        assert_eq!(solve(&vec![
            Instruction::new(Switch::On, 10..=12, 10..=12, 10..=12),
            Instruction::new(Switch::On, 11..=13, 11..=13, 11..=13),
            Instruction::new(Switch::Off, 9..=11, 9..=11, 9..=11),
            Instruction::new(Switch::On, 10..=10, 10..=10, 10..=10),
        ]).0, 39);
    }

    #[test]
    fn test_solve_part_1() {
        assert_eq!(solve(&vec![
            Instruction::new(Switch::On, -20..=26, -36..=17, -47..=7),
            Instruction::new(Switch::On, -20..=33, -21..=23, -26..=28),
            Instruction::new(Switch::On, -22..=28, -29..=23, -38..=16),
            Instruction::new(Switch::On, -46..=7, -6..=46, -50..=-1),
            Instruction::new(Switch::On, -49..=1, -3..=46, -24..=28),
            Instruction::new(Switch::On, 2..=47, -22..=22, -23..=27),
            Instruction::new(Switch::On, -27..=23, -28..=26, -21..=29),
            Instruction::new(Switch::On, -39..=5, -6..=47, -3..=44),
            Instruction::new(Switch::On, -30..=21, -8..=43, -13..=34),
            Instruction::new(Switch::On, -22..=26, -27..=20, -29..=19),
            Instruction::new(Switch::Off, -48..=-32, 26..=41, -47..=-37),
            Instruction::new(Switch::On, -12..=35, 6..=50, -50..=-2),
            Instruction::new(Switch::Off, -48..=-32, -32..=-16, -15..=-5),
            Instruction::new(Switch::On, -18..=26, -33..=15, -7..=46),
            Instruction::new(Switch::Off, -40..=-22, -38..=-28, 23..=41),
            Instruction::new(Switch::On, -16..=35, -41..=10, -47..=6),
            Instruction::new(Switch::Off, -32..=-23, 11..=30, -14..=3),
            Instruction::new(Switch::On, -49..=-5, -3..=45, -29..=18),
            Instruction::new(Switch::Off, 18..=30, -20..=-8, -3..=13),
            Instruction::new(Switch::On, -41..=9, -7..=43, -33..=15),
            Instruction::new(Switch::On, -54112..=-39298, -85059..=-49293, -27449..=7877),
            Instruction::new(Switch::On, 967..=23432, 45373..=81175, 27513..=53682),
        ]).0, 590784);
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(solve(&vec![
            Instruction::new(Switch::On, -5..=47, -31..=22, -19..=33),
            Instruction::new(Switch::On, -44..=5, -27..=21, -14..=35),
            Instruction::new(Switch::On, -49..=-1, -11..=42, -10..=38),
            Instruction::new(Switch::On, -20..=34, -40..=6, -44..=1),
            Instruction::new(Switch::Off, 26..=39, 40..=50, -2..=11),
            Instruction::new(Switch::On, -41..=5, -41..=6, -36..=8),
            Instruction::new(Switch::Off, -43..=-33, -45..=-28, 7..=25),
            Instruction::new(Switch::On, -33..=15, -32..=19, -34..=11),
            Instruction::new(Switch::Off, 35..=47, -46..=-34, -11..=5),
            Instruction::new(Switch::On, -14..=36, -6..=44, -16..=29),
            Instruction::new(Switch::On, -57795..=-6158, 29564..=72030, 20435..=90618),
            Instruction::new(Switch::On, 36731..=105352, -21140..=28532, 16094..=90401),
            Instruction::new(Switch::On, 30999..=107136, -53464..=15513, 8553..=71215),
            Instruction::new(Switch::On, 13528..=83982, -99403..=-27377, -24141..=23996),
            Instruction::new(Switch::On, -72682..=-12347, 18159..=111354, 7391..=80950),
            Instruction::new(Switch::On, -1060..=80757, -65301..=-20884, -103788..=-16709),
            Instruction::new(Switch::On, -83015..=-9461, -72160..=-8347, -81239..=-26856),
            Instruction::new(Switch::On, -52752..=22273, -49450..=9096, 54442..=119054),
            Instruction::new(Switch::On, -29982..=40483, -108474..=-28371, -24328..=38471),
            Instruction::new(Switch::On, -4958..=62750, 40422..=118853, -7672..=65583),
            Instruction::new(Switch::On, 55694..=108686, -43367..=46958, -26781..=48729),
            Instruction::new(Switch::On, -98497..=-18186, -63569..=3412, 1232..=88485),
            Instruction::new(Switch::On, -726..=56291, -62629..=13224, 18033..=85226),
            Instruction::new(Switch::On, -110886..=-34664, -81338..=-8658, 8914..=63723),
            Instruction::new(Switch::On, -55829..=24974, -16897..=54165, -121762..=-28058),
            Instruction::new(Switch::On, -65152..=-11147, 22489..=91432, -58782..=1780),
            Instruction::new(Switch::On, -120100..=-32970, -46592..=27473, -11695..=61039),
            Instruction::new(Switch::On, -18631..=37533, -124565..=-50804, -35667..=28308),
            Instruction::new(Switch::On, -57817..=18248, 49321..=117703, 5745..=55881),
            Instruction::new(Switch::On, 14781..=98692, -1341..=70827, 15753..=70151),
            Instruction::new(Switch::On, -34419..=55919, -19626..=40991, 39015..=114138),
            Instruction::new(Switch::On, -60785..=11593, -56135..=2999, -95368..=-26915),
            Instruction::new(Switch::On, -32178..=58085, 17647..=101866, -91405..=-8878),
            Instruction::new(Switch::On, -53655..=12091, 50097..=105568, -75335..=-4862),
            Instruction::new(Switch::On, -111166..=-40997, -71714..=2688, 5609..=50954),
            Instruction::new(Switch::On, -16602..=70118, -98693..=-44401, 5197..=76897),
            Instruction::new(Switch::On, 16383..=101554, 4615..=83635, -44907..=18747),
            Instruction::new(Switch::Off, -95822..=-15171, -19987..=48940, 10804..=104439),
            Instruction::new(Switch::On, -89813..=-14614, 16069..=88491, -3297..=45228),
            Instruction::new(Switch::On, 41075..=99376, -20427..=49978, -52012..=13762),
            Instruction::new(Switch::On, -21330..=50085, -17944..=62733, -112280..=-30197),
            Instruction::new(Switch::On, -16478..=35915, 36008..=118594, -7885..=47086),
            Instruction::new(Switch::Off, -98156..=-27851, -49952..=43171, -99005..=-8456),
            Instruction::new(Switch::Off, 2032..=69770, -71013..=4824, 7471..=94418),
            Instruction::new(Switch::On, 43670..=120875, -42068..=12382, -24787..=38892),
            Instruction::new(Switch::Off, 37514..=111226, -45862..=25743, -16714..=54663),
            Instruction::new(Switch::Off, 25699..=97951, -30668..=59918, -15349..=69697),
            Instruction::new(Switch::Off, -44271..=17935, -9516..=60759, 49131..=112598),
            Instruction::new(Switch::On, -61695..=-5813, 40978..=94975, 8655..=80240),
            Instruction::new(Switch::Off, -101086..=-9439, -7088..=67543, 33935..=83858),
            Instruction::new(Switch::Off, 18020..=114017, -48931..=32606, 21474..=89843),
            Instruction::new(Switch::Off, -77139..=10506, -89994..=-18797, -80..=59318),
            Instruction::new(Switch::Off, 8476..=79288, -75520..=11602, -96624..=-24783),
            Instruction::new(Switch::On, -47488..=-1262, 24338..=100707, 16292..=72967),
            Instruction::new(Switch::Off, -84341..=13987, 2429..=92914, -90671..=-1318),
            Instruction::new(Switch::Off, -37810..=49457, -71013..=-7894, -105357..=-13188),
            Instruction::new(Switch::Off, -27365..=46395, 31009..=98017, 15428..=76570),
            Instruction::new(Switch::Off, -70369..=-16548, 22648..=78696, -1892..=86821),
            Instruction::new(Switch::On, -53470..=21291, -120233..=-33476, -44150..=38147),
            Instruction::new(Switch::Off, -93533..=-4276, -16170..=68771, -104985..=-24507),
        ]).1, 2758514936282235);
    }
}

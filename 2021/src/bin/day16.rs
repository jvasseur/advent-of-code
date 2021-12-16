use advent_of_code_2021::{parse, read};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::character::complete::newline;
use nom::combinator::all_consuming;
use nom::combinator::value;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::multi::many0;
use nom::multi::many_m_n;
use nom::IResult;
use nom::Finish;

fn parser(input: &str) -> IResult<&str, Vec<u8>> {
    let (rest, groups) = terminated(many0(alt((
        value([0, 0, 0, 0], tag("0")),
        value([0, 0, 0, 1], tag("1")),
        value([0, 0, 1, 0], tag("2")),
        value([0, 0, 1, 1], tag("3")),
        value([0, 1, 0, 0], tag("4")),
        value([0, 1, 0, 1], tag("5")),
        value([0, 1, 1, 0], tag("6")),
        value([0, 1, 1, 1], tag("7")),
        value([1, 0, 0, 0], tag("8")),
        value([1, 0, 0, 1], tag("9")),
        value([1, 0, 1, 0], tag("A")),
        value([1, 0, 1, 1], tag("B")),
        value([1, 1, 0, 0], tag("C")),
        value([1, 1, 0, 1], tag("D")),
        value([1, 1, 1, 0], tag("E")),
        value([1, 1, 1, 1], tag("F")),
    ))), newline)(input)?;

    Ok((rest, groups.into_iter().flatten().collect()))
}

fn to_num(input: &[u8]) -> u32 {
    input.into_iter().rev().enumerate().filter(|(_, &val)| val == 1).map(|(i, _)| 2_u32.pow(i as u32)).sum()
}

#[derive(Clone,Debug,Eq,PartialEq)]
struct Packet {
    version: u8,
    data: PacketData,
}

#[derive(Clone,Debug,Eq,PartialEq)]
enum PacketData {
    Value(u64),
    Operator(Operator, Vec<Packet>),
}

#[derive(Clone,Debug,Eq,PartialEq)]
enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl Packet {
    fn sum_version(&self) -> u32 {
        self.version as u32 + match &self.data {
            PacketData::Value(_) => 0,
            PacketData::Operator(_, packets) => packets.iter().map(Packet::sum_version).sum(),
        }
    }

    fn compute(&self) -> u64 {
        match &self.data {
            PacketData::Value(value) => *value,
            PacketData::Operator(operator, packets) => match operator {
                Operator::Sum => packets.into_iter().map(|packet| packet.compute()).sum(),
                Operator::Product => packets.into_iter().map(|packet| packet.compute()).product(),
                Operator::Minimum => packets.into_iter().map(|packet| packet.compute()).min().unwrap(),
                Operator::Maximum => packets.into_iter().map(|packet| packet.compute()).max().unwrap(),
                Operator::GreaterThan => if packets[0].compute() > packets[1].compute() { 1 } else { 0 },
                Operator::LessThan => if packets[0].compute() < packets[1].compute() { 1 } else { 0 },
                Operator::EqualTo => if packets[0].compute() == packets[1].compute() { 1 } else { 0 },
            },
        }
    }
}

fn parse_packet(input: &[u8]) -> IResult<&[u8], Packet> {
    let (input, version) = take(3_u8)(input)?;

    let (input, data) = alt((
        parse_value_data,
        parse_operator_data,
    ))(input)?;

    Ok((input, Packet {
        version: version[0] * 4 + version[1] * 2 + version[2],
        data,
    }))
}

fn parse_value_data(input: &[u8]) -> IResult<&[u8], PacketData> {
    let (input, _) = tag([1, 0, 0])(input)?;

    let (input, mut data) = many0(preceded(tag([1]), take(4_u8)))(input)?;
    let (input, terminal) = preceded(tag([0]), take(4_u8))(input)?;

    data.push(terminal);

    let value = data.into_iter().flatten().rev().enumerate().filter(|(_, &val)| val == 1).map(|(i, _)| 2_u64.pow(i as u32)).sum();

    Ok((input, PacketData::Value(value)))
}

fn parse_operator_data(input: &[u8]) -> IResult<&[u8], PacketData> {
    let (input, operator) = alt((
        value(Operator::Sum, tag([0, 0, 0])),
        value(Operator::Product, tag([0, 0, 1])),
        value(Operator::Minimum, tag([0, 1, 0])),
        value(Operator::Maximum, tag([0, 1, 1])),
        value(Operator::GreaterThan, tag([1, 0, 1])),
        value(Operator::LessThan, tag([1, 1, 0])),
        value(Operator::EqualTo, tag([1, 1, 1])),
    ))(input)?;
    let (input, mode) = take(1_u8)(input)?;

    let (input, packets) = if mode == [0] {
        let (input, count) = take(15_u8)(input)?;

        let count = to_num(count);

        let (input, data) = take(count)(input)?;

        let (_, packets) = all_consuming(many0(parse_packet))(data)?;

        (input, packets)
    } else {
        let (input, count) = take(11_u8)(input)?;

        let count = to_num(count) as usize;

        many_m_n(count, count, parse_packet)(input)?
    };

    Ok((input, PacketData::Operator(operator, packets)))
}

fn solve_part1(input: &[u8]) -> u32 {
    let (_, packet) = parse_packet(input).finish().unwrap();

    packet.sum_version()
}

fn solve_part2(input: &[u8]) -> u64 {
    let (_, packet) = parse_packet(input).finish().unwrap();

    packet.compute()
}

fn main() {
    let input = read(16);

    let parsed_input = parse(parser, &input);

    println!("{}", solve_part1(&parsed_input));
    println!("{}", solve_part2(&parsed_input));
}

#[cfg(test)]
mod tests {
    use nom::Finish;
    use super::Packet;
    use super::PacketData;
    use super::Operator;
    use super::parser;
    use super::to_num;
    use super::parse_value_data;
    use super::parse_operator_data;
    use super::solve_part1;
    use super::solve_part2;

    #[test]
    fn test_parser() {
        assert_eq!(
            parser("D2FE28\n"),
            Ok(("", (vec![1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0])))
        );
    }

    #[test]
    fn test_to_num() {
        assert_eq!(to_num(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1]), 27);
    }

    #[test]
    fn test_parse_value_data() {
        assert_eq!(
            parse_value_data(&[1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0]),
            Ok((&[0, 0, 0][..], PacketData::Value(2021)))
        );
    }

    #[test]
    fn test_parse_operator_data() {
        assert_eq!(
            parse_operator_data(&[1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            Ok((&[0, 0, 0, 0, 0, 0, 0][..], PacketData::Operator(Operator::LessThan, vec![
                Packet { version: 6, data: PacketData::Value(10) },
                Packet { version: 2, data: PacketData::Value(20) },
            ])))
        );
    }

    #[test]
    fn test_solve_part_1() {
        let (_, input) = parser("A0016C880162017C3686B18A3D4780\n").finish().unwrap();

        assert_eq!(solve_part1(&input), 31);
    }

    #[test]
    fn test_solve_part_2() {
        let (_, input) = parser("9C0141080250320F1802104A08\n").finish().unwrap();

        assert_eq!(solve_part2(&input), 1);
    }
}

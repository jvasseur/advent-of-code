use nom::character::complete::newline;
use nom::combinator::all_consuming;
use nom::error::Error;
use nom::multi::many0;
use nom::sequence::terminated;
use nom::{Finish, Parser};
use std::fs::read_to_string;

pub mod util;

pub fn read(day: u8) -> String {
    read_to_string(format!("input/day{}.txt", day)).expect("Failed to read input file")
}

pub fn parse<'a, O>(parser: impl Parser<&'a str, O, Error<&'a str>>, input: &'a str) -> O {
    all_consuming(parser)(input).finish().expect("Failed to parse input").1
}

pub fn parse_lines<'a, O>(parser: impl Parser<&'a str, O, Error<&'a str>>, input: &'a str) -> Vec<O> {
    parse(many0(terminated(parser, newline)), input)
}

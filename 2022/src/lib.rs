use nom::combinator::all_consuming;
use nom::error::Error;
use nom::{Finish, Parser};
use std::fs::read_to_string;

pub fn read(day: u8) -> String {
    read_to_string(format!("input/day{}.txt", day)).expect("Failed to read input file")
}

pub fn parse<'a, O>(parser: impl Parser<&'a str, O, Error<&'a str>>, input: &'a str) -> O {
    all_consuming(parser)(input).finish().expect("Failed to parse input").1
}

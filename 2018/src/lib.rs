use nom::combinator::all_consuming;
use nom::error::Error;
use nom::{Finish, IResult, Parser};
use std::fs::read_to_string;

pub fn read(day: u8) -> String {
    read_to_string(format!("input/day{}.txt", day)).expect("Failed to read input file")
}

pub trait Parsable: Sized {
    fn parser(input: &str) -> IResult<&str, Self>;

    fn parse(input: &str) -> Result<Self, Error<&str>> {
        Ok(all_consuming(Self::parser)(input).finish()?.1)
    }
}

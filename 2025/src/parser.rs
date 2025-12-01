use nom::bytes::complete::tag;
use nom::combinator::all_consuming;
use nom::error::Error;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::{Finish, IResult};

pub fn parse<T: Parsable>(input: &str) -> Result<T, Error<&str>> {
    Ok(all_consuming(T::parser)(input).finish()?.1)
}

pub trait Parsable: Sized {
    fn parser(input: &str) -> IResult<&str, Self>;
}

impl Parsable for u8 {
    fn parser(input: &str) -> IResult<&str, Self> {
        nom::character::complete::u8(input)
    }
}

impl Parsable for u16 {
    fn parser(input: &str) -> IResult<&str, Self> {
        nom::character::complete::u16(input)
    }
}

impl Parsable for u32 {
    fn parser(input: &str) -> IResult<&str, Self> {
        nom::character::complete::u32(input)
    }
}

impl Parsable for u64 {
    fn parser(input: &str) -> IResult<&str, Self> {
        nom::character::complete::u64(input)
    }
}

impl Parsable for char {
    fn parser(input: &str) -> IResult<&str, Self> {
        nom::character::complete::anychar(input)
    }
}

pub fn lines_parser<T: Parsable>(input: &str) -> IResult<&str, Vec<T>>
{
    many1(terminated(T::parser, tag("\n")))(input)
}

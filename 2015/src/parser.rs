use nom::combinator::all_consuming;
use nom::error::Error;
use nom::{Finish, IResult};

pub type ParserResult<'a, T> = IResult<&'a str, T>;

pub fn parse<T: Parsable>(input: &str) -> Result<T, Error<&str>> {
    Ok(all_consuming(T::parser)(input).finish()?.1)
}

pub trait Parsable: Sized {
    fn parser<'a>(input: &'a str) -> ParserResult<'a, Self>;
}

impl Parsable for char {
    fn parser<'a>(input: &'a str) -> ParserResult<'a, Self> {
        nom::character::complete::anychar(input)
    }
}

impl Parsable for u8 {
    fn parser<'a>(input: &'a str) -> ParserResult<'a, Self> {
        nom::character::complete::u8(input)
    }
}

impl Parsable for u16 {
    fn parser<'a>(input: &'a str) -> ParserResult<'a, Self> {
        nom::character::complete::u16(input)
    }
}

impl Parsable for u32 {
    fn parser<'a>(input: &'a str) -> ParserResult<'a, Self> {
        nom::character::complete::u32(input)
    }
}

impl Parsable for u64 {
    fn parser<'a>(input: &'a str) -> ParserResult<'a, Self> {
        nom::character::complete::u64(input)
    }
}

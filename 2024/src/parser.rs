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

pub fn lines_parser<T: Parsable>(input: &str) -> IResult<&str, Vec<T>>
{
    many1(terminated(T::parser, tag("\n")))(input)
}

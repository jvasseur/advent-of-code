use nom::bytes::complete::tag;
use nom::character::complete::none_of;
use nom::combinator::{all_consuming, map_parser, recognize};
use nom::error::Error;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::{Finish, IResult};

use crate::grid::Grid;

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

pub fn lines_parser<T: Parsable>(input: &str) -> IResult<&str, Vec<T>>
{
    many1(terminated(T::parser, tag("\n")))(input)
}

pub fn grid_parser<T: Parsable + Clone + Default>(input: &str) -> IResult<&str, Grid<T>>
{
    let (input, grid) = many1(
        terminated(
            many1(
                map_parser(
                    recognize(none_of("\n")),
                    T::parser,
                ),
            ),
            tag("\n"),
        )
    )(input)?;

    Ok((input, grid.into()))
}

use nom::combinator::all_consuming;
use nom::error::Error;
use nom::Finish;
use nom::InputLength;
use nom::Parser;

pub fn apply<I, O>(parser: impl Parser<I, O, Error<I>>, input: I) -> Result<O, Error<I>>
where
    I: InputLength,
{
    let (_, result) = all_consuming(parser)(input).finish()?;

    Ok(result)
}

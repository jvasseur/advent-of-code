use advent_of_code_2021::{parse, read};
use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::character::complete::newline;
use nom::sequence::tuple;
use nom::IResult;
use nom::sequence::separated_pair;

fn parse_pair(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(i32, tag(".."), i32)(input)
}

fn parser(input: &str) -> IResult<&str, ((i32, i32), (i32, i32))> {
    let (input, (_, x, _, y, _)) = tuple((
        tag("target area: x="),
        parse_pair,
        tag(", y="),
        parse_pair,
        newline,
    ))(input)?;

    Ok((input, (x, y)))
}

fn solve_part1(input: &((i32, i32), (i32, i32))) -> i32 {
    let min_y = input.1.0;
    let max_y = input.1.1;

    let mut global_max = 0;

    for vy0 in 1..100 {
        let mut vy = vy0;
        let mut y = 0;
        let mut local_max = 0;

        loop {
            y += vy;
            vy -= 1;

            if y > local_max {
                local_max = y
            }

            if y > max_y {
                continue;
            }

            if y >= min_y {
                global_max = local_max;
            }

            break;
        }
    }

    global_max
}

fn solve_part2(input: &((i32, i32), (i32, i32))) -> i32 {
    let min_x = input.0.0;
    let max_x = input.0.1;
    let min_y = input.1.0;
    let max_y = input.1.1;

    let mut count = 0;

    for vx0 in 1..=1000 {
        for vy0 in -1000..=1000 {
            let mut vx = vx0;
            let mut vy = vy0;
            let mut x = 0;
            let mut y = 0;

            loop {
                x += vx;
                y += vy;
                if vx > 0 {
                    vx -= 1;
                }
                vy -= 1;

                if min_x <= x && x <= max_x && min_y <= y && y <= max_y {
                    count += 1;

                    break;
                }

                if x < min_x && vx == 0 {
                    // Probe lost all velocity, no hope to reach trench :(
                    break;
                }

                if x > max_x || y < min_y  {
                    // We overshot!
                    break;
                }
            }
        }
    }

    count
}

fn main() {
    let input = read(17);

    let parsed_input = parse(parser, &input);

    println!("{}", solve_part1(&parsed_input));
    println!("{}", solve_part2(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::parser;
    use super::solve_part1;
    use super::solve_part2;

    #[test]
    fn test_parser() {
        assert_eq!(
            parser("target area: x=20..30, y=-10..-5\n"),
            Ok(("", ((20, 30), (-10, -5))))
        );
    }

    #[test]
    fn test_solve_part_1() {
        assert_eq!(solve_part1(&((20, 30), (-10, -5))), 45);
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(solve_part2(&((20, 30), (-10, -5))), 112);
    }
}

use advent_of_code_2021::{parse, read};
use nom::bytes::complete::tag;
use nom::character::complete::i64;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;
use std::cmp::max;
use std::cmp::min;

fn bloc_parser(input: &str) -> IResult<&str, Bloc> {
    let (input, _) = tag("inp w\n")(input)?;
    let (input, _) = tag("mul x 0\n")(input)?;
    let (input, _) = tag("add x z\n")(input)?;
    let (input, _) = tag("mod x 26\n")(input)?;
    let (input, (_, a, _)) = tuple((tag("div z "), i64, tag("\n")))(input)?;
    let (input, (_, b, _)) = tuple((tag("add x "), i64, tag("\n")))(input)?;
    let (input, _) = tag("eql x w\n")(input)?;
    let (input, _) = tag("eql x 0\n")(input)?;
    let (input, _) = tag("mul y 0\n")(input)?;
    let (input, _) = tag("add y 25\n")(input)?;
    let (input, _) = tag("mul y x\n")(input)?;
    let (input, _) = tag("add y 1\n")(input)?;
    let (input, _) = tag("mul z y\n")(input)?;
    let (input, _) = tag("mul y 0\n")(input)?;
    let (input, _) = tag("add y w\n")(input)?;
    let (input, (_, c, _)) = tuple((tag("add y "), i64, tag("\n")))(input)?;
    let (input, _) = tag("mul y x\n")(input)?;
    let (input, _) = tag("add z y\n")(input)?;

    Ok((input, Bloc { a, b, c }))
}

#[derive(Clone)]
struct Bloc {
    a: i64, // a == 1 || a == 16
    b: i64, // if a == 1 b >= 10 else b < 0
    c: i64, // 1 <= c <= 16
}

/*
mul x 0     x = x * 0       x = 0
add x z     x = x + z       x = z
mod x 26    x = x mod 26    x = z mod 26
div z a     z = z / a       z = z / a
add x b     x = x + b       x = x + b
eql x w     x = x == w      x = (x + b) == w
eql x 0     x = x == 0      x = (x + b) != w
mul y 0     y = y * 0       y1 = 0
add y 25    y = y + 25      y1 = 25
mul y x     y = y * x       y1 = 25 * x
add y 1     y = y + 1       y1 = 25 * x + 1
mul z y     z = z * y       z = z * y1
mul y 0     y = y * 0       y2 = 0
add y w     y = y + w       y2 = w
add y c     y = y + c       y2 = w + c
mul y x     y = y * x       y2 = (w + c) * x
add z y     z = z + y       z = z + y2
*/
/*
impl Bloc {
    fn run(&self, z: i64, w: i64) -> i64 {
        let x = z.rem_euclid(26);
        let z = z / self.a;

        if x + self.b != w {
            return z * 26 + w + self.c;
        } else {
            return z;
        }
    }

    fn run2(&self, z: i64, w: i64) -> i64 {
        if self.a == 1 {
            return z * 26 + w + self.c;
        } else {
            let x = z.rem_euclid(26);

            if x + self.b != w {
                return z + w + self.c;
            } else {
                return z / 26;
            }
        }
    }
}*/

fn solve_part_1(instructions: &Vec<Bloc>) -> u64 {
    let mut num = [-1; 14];
    let mut stack: Vec<(usize, Bloc)> = Vec::new();

    for i in 0..14 {
        let this = instructions[i].clone();
        if instructions[i].a == 1 {
            stack.push((i, this));
        } else {
            let (j, poped) = stack.pop().unwrap();

            // this.w - this.b == poped.w + poped.c
            // this.w = poped.w + this.b + poped.c
            // poped.w = this.w - this.b - poped.c
            num[i] = min(9, 9 + this.b + poped.c);
            num[j] = min(9, 9 - this.b - poped.c);
        }
    }

    let mut result = 0;

    for i in 0..14 {
        result += (num[13 - i] as u64) * 10_u64.pow(i as u32);
    }

    result
}

fn solve_part_2(instructions: &Vec<Bloc>) -> u64 {
    let mut num = [-1; 14];
    let mut stack: Vec<(usize, Bloc)> = Vec::new();

    for i in 0..14 {
        let this = instructions[i].clone();
        if instructions[i].a == 1 {
            stack.push((i, this));
        } else {
            let (j, poped) = stack.pop().unwrap();

            // this.w - this.b == poped.w + poped.c
            // this.w = poped.w + this.b + poped.c
            // poped.w = this.w - this.b - poped.c
            num[i] = max(1, 1 + this.b + poped.c);
            num[j] = max(1, 1 - this.b - poped.c);
        }
    }

    let mut result = 0;

    for i in 0..14 {
        result += (num[13 - i] as u64) * 10_u64.pow(i as u32);
    }

    result
}

fn main() {
    let input = read(24);

    let parsed_input = parse(many0(bloc_parser), &input);

    println!("Part 1: {}", solve_part_1(&parsed_input));
    println!("Part 2: {}", solve_part_2(&parsed_input));
}

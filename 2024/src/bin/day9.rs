use advent_of_code_2024::{parser::*, read};
use nom::{bytes::complete::tag, character::complete::{anychar, u8}, combinator::{map_parser, recognize}, multi::many1, sequence::terminated, IResult};

#[derive(Clone, PartialEq, Eq, Debug)]
struct Space {
    size: u8,
    file: Option<usize>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    spaces: Vec<Space>,
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, data) = terminated(
            many1(
                map_parser(
                    recognize(anychar),
                     u8,
                ),
            ),
            tag("\n"),
        )(input)?;

        let mut spaces = Vec::new();

        let mut id = 0;
        let mut file = true;
        for size in data {
            if file {
                spaces.push(Space { size, file: Some(id) });

                id += 1;
                file = false;
            } else {
                if size > 0 {
                    spaces.push(Space { size, file: None });
                }

                file = true;
            }
        }

        Ok((input, Input { spaces }))
    }
}

fn solve_part1(input: &Input) -> usize {
    let mut blocks = Vec::new();

    for space in &input.spaces {
        for _ in 0..space.size {
            blocks.push(space.file);
        }
    }

    loop {
        let last_filled = blocks.iter().enumerate().rev().find(|(_, value)| value.is_some()).unwrap().0;
        let first_empty = blocks.iter().enumerate().find(|(_, value)| value.is_none()).unwrap().0;

        if last_filled < first_empty {
            break;
        }

        blocks.swap(last_filled, first_empty);
    }

    blocks.iter().enumerate().map(|(index, value)| value.unwrap_or(0) * index).sum()
}

fn solve_part2(input: &Input) -> usize {
    let last_file = input.spaces.last().unwrap().file.unwrap();

    let mut spaces = input.spaces.to_owned();

    for file in (0..=last_file).rev() {
        let (file_index, file_info) = spaces.iter().enumerate().find(|(_, space)| space.file == Some(file)).unwrap();
        let file_size = file_info.size;

        let mut dest = None;
        for (index, space) in spaces.iter().enumerate() {
            if index > file_index {
                break;
            }

            if space.file == None && space.size >= file_size {
                dest = Some((index, space.size));

                break;
            }
        }

        if let Some((dest_index, dest_size)) = dest {
            let file_info = std::mem::replace(&mut spaces[file_index], Space { size: file_size, file: None });

            spaces[dest_index] = file_info;

            if file_size < dest_size {
                spaces.insert(dest_index + 1, Space { size: dest_size - file_size, file: None });
            }
        }
    }

    let mut checksum = 0;
    let mut index = 0;

    for space in spaces {
        if let Some(file) = space.file {
            let index_sum: usize = (index..index + (space.size as usize)).sum();

            checksum += file * index_sum;
        }

        index += space.size as usize;
    }

    checksum
}

fn main() {
    let input = parse(&read(9).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402\n";

    fn parsed_input() -> Input {
        Input {
            spaces: vec![
                Space { size: 2, file: Some(0) },
                Space { size: 3, file: None },
                Space { size: 3, file: Some(1) },
                Space { size: 3, file: None },
                Space { size: 1, file: Some(2) },
                Space { size: 3, file: None },
                Space { size: 3, file: Some(3) },
                Space { size: 1, file: None },
                Space { size: 2, file: Some(4) },
                Space { size: 1, file: None },
                Space { size: 4, file: Some(5) },
                Space { size: 1, file: None },
                Space { size: 4, file: Some(6) },
                Space { size: 1, file: None },
                Space { size: 3, file: Some(7) },
                Space { size: 1, file: None },
                Space { size: 4, file: Some(8) },
                Space { size: 2, file: Some(9) },
            ],
        }
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse::<Input>(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 1928);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 2858);
    }
}

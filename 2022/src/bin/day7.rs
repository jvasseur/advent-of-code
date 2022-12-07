use advent_of_code_2022::{read, parse};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{alpha1, u32};
use nom::combinator::map;
use nom::IResult;
use nom::multi::many1;
use nom::sequence::{preceded, separated_pair, terminated};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Instruction {
    Cd(Move),
    Ls(Vec<Entry>),
}

#[derive(Debug, PartialEq)]
enum Move {
    Root,
    Up,
    Dir(String),
}

#[derive(Debug, PartialEq)]
struct Entry {
    name: String,
    kind: EntryKind,
}

impl Entry {
    pub fn dir(name: String) -> Self {
        Entry {
            name,
            kind: EntryKind::Dir,
        }
    }

    pub fn file(name: String, size: u32) -> Self {
        Entry {
            name,
            kind: EntryKind::File(size),
        }
    }
}

#[derive(Debug, PartialEq)]
enum EntryKind {
    Dir,
    File(u32),
}

fn cd_parser(input: &str) -> IResult<&str, Move> {
    terminated(
        alt((
            map(tag("$ cd /"), |_| Move::Root),
            map(tag("$ cd .."), |_| Move::Up),
            map(preceded(tag("$ cd "), alpha1), |dir: &str| Move::Dir(dir.to_owned())),
        )),
        tag("\n"),
    )(input)
}

fn ls_parser(input: &str) -> IResult<&str, Vec<Entry>> {
    preceded(
        tag("$ ls\n"),
        many1(
            terminated(
                alt((
                    map(preceded(
                        tag("dir "),
                        alpha1,
                    ), |name: &str| Entry::dir(name.to_owned())),
                    map(separated_pair(
                        u32,
                        tag(" "),
                        is_not(" \n"),
                    ), |(size, name): (u32, &str)| Entry::file(name.to_owned(), size)),
                )),
                tag("\n"),
            ),
        )
    )(input)
}

fn parser(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(
        alt((
            map(cd_parser, |cd| Instruction::Cd(cd)),
            map(ls_parser, |ls| Instruction::Ls(ls)),
        )),
    )(input)
}

#[derive(Debug, PartialEq)]
struct Dir {
    files: HashMap<String, DirEntry>,
}

impl Dir {
    pub fn new() -> Self {
        Dir {
            files: HashMap::new(),
        }
    }

    pub fn size(&self) -> u32 {
        self.files.iter().map(|(_, entry)| entry.size()).sum()
    }
}

#[derive(Debug, PartialEq)]
enum DirEntry {
    Dir(Dir),
    File(u32),
}

impl DirEntry {
    pub fn size(&self) -> u32 {
        match self {
            DirEntry::Dir(dir) => dir.size(),
            DirEntry::File(size) => *size,
        }
    }
}

fn build_tree(instructions: &[Instruction]) -> Dir {
    let mut dir = Dir::new();
    let mut current: Vec<String> = Vec::new();

    for instruction in instructions {
        match instruction {
            Instruction::Cd(cd) => match cd {
                Move::Root => {
                    current = Vec::new();
                },
                Move::Up => {
                    current.pop();
                },
                Move::Dir(name) => {
                    current.push(name.to_owned());
                },
            },
            Instruction::Ls(entries) => {
                let mut current_dir = Box::new(&mut dir);

                for path in &current {
                    *current_dir = match current_dir.files.get_mut(path).unwrap() {
                        DirEntry::Dir(d) => d,
                        DirEntry::File(_) => panic!("Here be dragons"),
                    }
                }

                for entry in entries {
                    current_dir.files.insert(entry.name.clone(), match entry.kind {
                        EntryKind::Dir => DirEntry::Dir(Dir::new()),
                        EntryKind::File(size) => DirEntry::File(size),
                    });
                }
            }
        }
    }

    dir
}

fn solve_part1(input: &Dir) -> u32 {
    let mut sum = 0;

    if input.size() <= 100000 {
        sum += input.size();
    }

    for (_, entry) in &input.files {
        if let DirEntry::Dir(dir) = entry {
            sum += solve_part1(dir);
        }
    }

    sum
}

fn find_candidate(input: &Dir, needed: u32, candidate: u32) -> u32 {
    let mut candidate = candidate;

    if input.size() > needed && input.size() < candidate {
        candidate = input.size();
    }

    for (_, entry) in &input.files {
        if let DirEntry::Dir(dir) = entry {
            candidate = find_candidate(dir, needed, candidate);
        }
    }

    candidate
}

fn solve_part2(input: &Dir) -> u32 {
    let remaining = 70000000 - input.size();
    let needed = 30000000 - remaining;

    find_candidate(input, needed, 70000000)
}

fn main() {
    let input = read(7);

    let parsed = parse(parser, &input);

    let tree = build_tree(&parsed);

    println!("{}", solve_part1(&tree));
    println!("{}", solve_part2(&tree));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(parser("$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k\n"), Ok(("", vec![
            Instruction::Cd(Move::Root),
            Instruction::Ls(vec![
                Entry::dir("a".to_owned()),
                Entry::file("b.txt".to_owned(), 14848514),
                Entry::file("c.dat".to_owned(), 8504156),
                Entry::dir("d".to_owned()),
            ]),
            Instruction::Cd(Move::Dir("a".to_owned())),
            Instruction::Ls(vec![
                Entry::dir("e".to_owned()),
                Entry::file("f".to_owned(), 29116),
                Entry::file("g".to_owned(), 2557),
                Entry::file("h.lst".to_owned(), 62596),
            ]),
            Instruction::Cd(Move::Dir("e".to_owned())),
            Instruction::Ls(vec![
                Entry::file("i".to_owned(), 584),
            ]),
            Instruction::Cd(Move::Up),
            Instruction::Cd(Move::Up),
            Instruction::Cd(Move::Dir("d".to_owned())),
            Instruction::Ls(vec![
                Entry::file("j".to_owned(), 4060174),
                Entry::file("d.log".to_owned(), 8033020),
                Entry::file("d.ext".to_owned(), 5626152),
                Entry::file("k".to_owned(), 7214296),
            ]),
        ])));
    }

    #[test]
    fn test_build_tree() {
        assert_eq!(build_tree(&vec![
            Instruction::Cd(Move::Root),
            Instruction::Ls(vec![
                Entry::dir("a".to_owned()),
                Entry::file("b.txt".to_owned(), 14848514),
                Entry::file("c.dat".to_owned(), 8504156),
                Entry::dir("d".to_owned()),
            ]),
            Instruction::Cd(Move::Dir("a".to_owned())),
            Instruction::Ls(vec![
                Entry::dir("e".to_owned()),
                Entry::file("f".to_owned(), 29116),
                Entry::file("g".to_owned(), 2557),
                Entry::file("h.lst".to_owned(), 62596),
            ]),
            Instruction::Cd(Move::Dir("e".to_owned())),
            Instruction::Ls(vec![
                Entry::file("i".to_owned(), 584),
            ]),
            Instruction::Cd(Move::Up),
            Instruction::Cd(Move::Up),
            Instruction::Cd(Move::Dir("d".to_owned())),
            Instruction::Ls(vec![
                Entry::file("j".to_owned(), 4060174),
                Entry::file("d.log".to_owned(), 8033020),
                Entry::file("d.ext".to_owned(), 5626152),
                Entry::file("k".to_owned(), 7214296),
            ]),
        ]), Dir {
            files: HashMap::from([
                ("a".to_owned(), DirEntry::Dir(Dir {
                    files: HashMap::from([
                        ("e".to_owned(), DirEntry::Dir(Dir {
                            files: HashMap::from([
                                ("i".to_owned(), DirEntry::File(584)),
                            ]),
                        })),
                        ("f".to_owned(), DirEntry::File(29116)),
                        ("g".to_owned(), DirEntry::File(2557)),
                        ("h.lst".to_owned(), DirEntry::File(62596)),
                    ]),
                })),
                ("b.txt".to_owned(), DirEntry::File(14848514)),
                ("c.dat".to_owned(), DirEntry::File(8504156)),
                ("d".to_owned(), DirEntry::Dir(Dir {
                    files: HashMap::from([
                        ("j".to_owned(), DirEntry::File(4060174)),
                        ("d.log".to_owned(), DirEntry::File(8033020)),
                        ("d.ext".to_owned(), DirEntry::File(5626152)),
                        ("k".to_owned(), DirEntry::File(7214296)),
                    ]),
                })),
            ]),
        })
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&Dir {
            files: HashMap::from([
                ("a".to_owned(), DirEntry::Dir(Dir {
                    files: HashMap::from([
                        ("e".to_owned(), DirEntry::Dir(Dir {
                            files: HashMap::from([
                                ("i".to_owned(), DirEntry::File(584)),
                            ]),
                        })),
                        ("f".to_owned(), DirEntry::File(29116)),
                        ("g".to_owned(), DirEntry::File(2557)),
                        ("h.lst".to_owned(), DirEntry::File(62596)),
                    ]),
                })),
                ("b.txt".to_owned(), DirEntry::File(14848514)),
                ("c.dat".to_owned(), DirEntry::File(8504156)),
                ("d".to_owned(), DirEntry::Dir(Dir {
                    files: HashMap::from([
                        ("j".to_owned(), DirEntry::File(4060174)),
                        ("d.log".to_owned(), DirEntry::File(8033020)),
                        ("d.ext".to_owned(), DirEntry::File(5626152)),
                        ("k".to_owned(), DirEntry::File(7214296)),
                    ]),
                })),
            ]),
        }), 95437);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&Dir {
            files: HashMap::from([
                ("a".to_owned(), DirEntry::Dir(Dir {
                    files: HashMap::from([
                        ("e".to_owned(), DirEntry::Dir(Dir {
                            files: HashMap::from([
                                ("i".to_owned(), DirEntry::File(584)),
                            ]),
                        })),
                        ("f".to_owned(), DirEntry::File(29116)),
                        ("g".to_owned(), DirEntry::File(2557)),
                        ("h.lst".to_owned(), DirEntry::File(62596)),
                    ]),
                })),
                ("b.txt".to_owned(), DirEntry::File(14848514)),
                ("c.dat".to_owned(), DirEntry::File(8504156)),
                ("d".to_owned(), DirEntry::Dir(Dir {
                    files: HashMap::from([
                        ("j".to_owned(), DirEntry::File(4060174)),
                        ("d.log".to_owned(), DirEntry::File(8033020)),
                        ("d.ext".to_owned(), DirEntry::File(5626152)),
                        ("k".to_owned(), DirEntry::File(7214296)),
                    ]),
                })),
            ]),
        }), 24933642);
    }

}

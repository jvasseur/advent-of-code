use advent_of_code_2024::{parser::*, read};
use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete::u32, multi::many1, sequence::terminated, IResult};

#[derive(Clone, PartialEq, Eq, Debug)]
struct Code {
    numeric_part: u32,
}

impl Code {
    fn new(numeric_part: u32) -> Self {
        Self { numeric_part }
    }
}

impl Parsable for Code {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, numeric_part) = terminated(u32, tag("A"))(input)?;

        Ok((input, Self::new(numeric_part)))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    codes: Vec<Code>,
}

impl Input {
    fn new(codes: Vec<Code>) -> Self {
        Self { codes }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, codes) = many1(terminated(Code::parser, tag("\n")))(input)?;

        Ok((input, Input::new(codes)))
    }
}

fn range(a: u32, b: u32) -> Vec<u32> {
    if a < b {
        return (a..=b).collect()
    }

    if a > b {
        return (b..=a).rev().collect()
    }

    Vec::new()
}

type KeyPadButton = (u32, u32);

trait KeyPad {
    fn get_start(&self) -> KeyPadButton;

    fn is_valid(&self, position: &KeyPadButton) -> bool;

    fn get_paths(&self, from: KeyPadButton, to: KeyPadButton) -> Vec<Vec<KeyPadButton>> {
        let mut first = vec![from];

        {
            for i in range(from.0, to.0).into_iter().skip(1) {
                first.push((i, from.1));
            }

            for j in range(from.1, to.1).into_iter().skip(1) {
                first.push((to.0, j));
            }
        }

        let mut second = vec![from];

        {
            for j in range(from.1, to.1).into_iter().skip(1) {
                second.push((from.0, j));
            }

            for i in range(from.0, to.0).into_iter().skip(1) {
                second.push((i, to.1));
            }
        }

        vec![first, second].into_iter().filter(|path| path.iter().all(|position| self.is_valid(position))).unique().collect()
    }

    fn get_possible_buttons_for_code(&self, code: &[u32]) -> Vec<Vec<KeyPadButton>>;
}

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
struct NumericKeyPad {
}

impl NumericKeyPad {
    pub fn new() -> Self {
        Self {}
    }
}

impl KeyPad for NumericKeyPad {
    fn get_start(&self) -> KeyPadButton {
        (2, 0)
    }

    fn is_valid(&self, position: &KeyPadButton) -> bool {
        position != &(0, 0)
    }

    fn get_possible_buttons_for_code(&self, code: &[u32]) -> Vec<Vec<KeyPadButton>> {
        let mut buttons = Vec::new();

        for number in code {
            buttons.push(match number {
                0 => (1, 0),
                1 => (0, 1),
                2 => (1, 1),
                3 => (2, 1),
                4 => (0, 2),
                5 => (1, 2),
                6 => (2, 2),
                7 => (0, 3),
                8 => (1, 3),
                9 => (2, 3),
                _ => panic!("Invalid button"),
            })
        }

        buttons.push((2, 0));

        vec![buttons]
    }
}

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
struct RobotKeyPad {
    target: Box<dyn KeyPad>,
}

impl RobotKeyPad {
    pub fn new(target: Box<dyn KeyPad>) -> Self {
        Self { target }
    }
}

impl KeyPad for RobotKeyPad {
    fn get_start(&self) -> KeyPadButton {
        (2, 1)
    }

    fn is_valid(&self, position: &KeyPadButton) -> bool {
        position != &(0, 1)
    }

    fn get_possible_buttons_for_code(&self, code: &[u32]) -> Vec<Vec<KeyPadButton>> {
        let mut possible_buttons = Vec::new();

        for buttons in self.target.get_possible_buttons_for_code(code) {
            let mut current = vec![vec![]];

            for (start, end) in [self.target.get_start()].into_iter().chain(buttons.into_iter()).tuple_windows() {
                let mut paths_buttons = Vec::new();

                for path in self.target.get_paths(start, end) {
                    let mut path_buttons = Vec::new();

                    for (a, b) in path.into_iter().tuple_windows() {
                        path_buttons.push(match ((b.0 as i32 - a.0 as i32), (b.1 as i32 - a.1 as i32)) {
                            (-1, 0) => (0, 0),
                            (1, 0) => (2, 0),
                            (0, -1) => (1, 0),
                            (0, 1) => (1, 1),
                            _ => panic!("Invalid movment"),
                        });
                    }

                    path_buttons.push((2, 1));

                    paths_buttons.push(path_buttons);
                }

                current = current.into_iter().cartesian_product(paths_buttons.into_iter()).map(|(a, b)| [a, b].concat()).collect();
            }

            possible_buttons = [possible_buttons, current].concat();
        }

        let best = possible_buttons.iter().map(|puttons| puttons.len()).min().unwrap();

        possible_buttons.into_iter().filter(|buttons| buttons.len() == best).collect()
    }
}

fn solve_part1(input: &Input) -> usize {
    let robot = RobotKeyPad::new(
        Box::new(RobotKeyPad::new(
            Box::new(RobotKeyPad::new(
                Box::new(NumericKeyPad::new()),
            )),
        )),
    );

    let mut result = 0;

    for code in &input.codes {
        let code_as_array = [code.numeric_part / 100, (code.numeric_part / 10) % 10, code.numeric_part % 10];

        let best_path_len = robot.get_possible_buttons_for_code(&code_as_array).into_iter().map(|codes| codes.len()).min().unwrap();

        result += best_path_len * (code.numeric_part as usize);
    }

    result
}

fn solve_part2(input: &Input) -> usize {
    0
}

fn main() {
    let input = parse(&read(21).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "029A
980A
179A
456A
379A
";

    fn parsed_input() -> Input {
        Input::new(vec![
            Code::new(029),
            Code::new(980),
            Code::new(179),
            Code::new(456),
            Code::new(379),
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse::<Input>(INPUT), Ok(parsed_input()));
    }

    #[test]
    fn test_get_paths() {
        let numeric = NumericKeyPad::new();
        let robot = RobotKeyPad::new(Box::new(NumericKeyPad::new()));

        assert_eq!(numeric.get_paths((2, 0), (1, 1)), vec![
            vec![(2, 0), (1, 0), (1, 1)],
            vec![(2, 0), (2, 1), (1, 1)],
        ]);

        assert_eq!(robot.get_paths((2, 0), (1, 1)), vec![
            vec![(2, 0), (1, 0), (1, 1)],
            vec![(2, 0), (2, 1), (1, 1)],
        ]);
    }

    #[test]
    fn test_get_paths_same() {
        let numeric = NumericKeyPad::new();
        let robot = RobotKeyPad::new(Box::new(NumericKeyPad::new()));

        assert_eq!(numeric.get_paths((2, 0), (2, 0)), vec![
            vec![(2, 0)],
        ]);

        assert_eq!(robot.get_paths((2, 0), (2, 0)), vec![
            vec![(2, 0)],
        ]);
    }

    #[test]
    fn test_get_paths_line() {
        let numeric = NumericKeyPad::new();
        let robot = RobotKeyPad::new(Box::new(NumericKeyPad::new()));

        assert_eq!(numeric.get_paths((2, 0), (2, 2)), vec![
            vec![(2, 0), (2, 1), (2, 2)],
        ]);

        assert_eq!(robot.get_paths((2, 0), (2, 2)), vec![
            vec![(2, 0), (2, 1), (2, 2)],
        ]);
    }

    #[test]
    fn get_possible_buttons_for_code() {
        let robot = RobotKeyPad::new(Box::new(NumericKeyPad::new()));

        let mut left = robot.get_possible_buttons_for_code(&[0, 2, 9]);
        let mut right = vec![
            vec![(0, 0), (2, 1), (1, 1), (2, 1), (2, 0), (1, 1), (1, 1), (2, 1), (1, 0), (1, 0), (1, 0), (2, 1)],
            // This one is given in the example but since it's not an optimal one we don't return it
            // vec![(0, 0), (2, 1), (1, 1), (2, 1), (1, 1), (2, 0), (1, 1), (2, 1), (1, 0), (1, 0), (1, 0), (2, 1)],
            vec![(0, 0), (2, 1), (1, 1), (2, 1), (1, 1), (1, 1), (2, 0), (2, 1), (1, 0), (1, 0), (1, 0), (2, 1)],
        ];

        for l in &left {
            println!("{:?}", l);
        }

        left.sort();
        right.sort();

        assert_eq!(left, right);
    }


    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 126384);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 0);
    }
}

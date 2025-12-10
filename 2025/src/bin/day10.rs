use advent_of_code_2025::{dijkstra::{Edge, shortest_path}, gcd, parser::*, read};
use derive_more::IntoIterator;
use nom::{IResult, branch::alt, bytes::complete::tag, combinator::{map, value}, multi::{many1, separated_list1}, sequence::{delimited, tuple}};

type Light = u8;
type Joltage = u16;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Machine {
    light_diagram: Vec<bool>,
    wiring_schematics: Vec<Vec<Light>>,
    joltage_requirements: Vec<Joltage>,
}

impl Machine {
    fn new(light_diagram: impl Into<Vec<bool>>, wiring_schematics: impl Into<Vec<Vec<Light>>>, joltage_requirements: impl Into<Vec<Joltage>>) -> Self {
        Self { light_diagram: light_diagram.into(), wiring_schematics: wiring_schematics.into(), joltage_requirements: joltage_requirements.into() }
    }
}

impl Parsable for Machine {
    fn parser(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                delimited(
                    tag("["),
                    many1(
                        alt((
                            value(true, tag("#")),
                            value(false, tag(".")),
                        )),
                    ),
                    tag("]"),
                ),
                tag(" "),
                separated_list1(
                    tag(" "),
                    delimited(
                        tag("("),
                        separated_list1(
                            tag(","),
                            parse,
                        ),
                        tag(")"),
                    ),
                ),
                tag(" "),
                delimited(
                    tag("{"),
                    separated_list1(
                        tag(","),
                        parse,
                    ),
                    tag("}"),
                ),
            )),
            |(light_diagram, _, wiring_schematics, _, joltage_requirements)| Machine::new(light_diagram, wiring_schematics, joltage_requirements),
        )(input)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, IntoIterator)]
#[into_iterator(owned, ref, ref_mut)]
struct Input {
    machines: Vec<Machine>
}

impl Input {
    fn new(machines: impl Into<Vec<Machine>>) -> Self {
        Self { machines: machines.into() }
    }
}

impl Parsable for Input {
    fn parser(input: &str) -> IResult<&str, Self> {
        map(
            parse_lines,
            Input::new,
        )(input)
    }
}

fn solve_part1(input: &Input) -> u32 {
    let mut total = 0;

    for machine in input {
        total += shortest_path(
            [vec![false; machine.light_diagram.len()]],
            |state| machine.wiring_schematics.iter()
                .map(|indexes| {
                    let mut new_state = state.clone();

                    for &index in indexes {
                        new_state[index as usize] = !new_state[index as usize]
                    }

                    Edge::new(new_state, 1)
                })
                .collect(),
            |state| state == &machine.light_diagram,
        ).unwrap();
    }

    total
}

fn solve_part2(input: &Input) -> u16 {
    let mut total = 0;

    // fn print_matrice(matrice: &Vec<Vec<i32>>) {
    //     for row in matrice {
    //         for col in row {
    //             print!("{} ", col);
    //         }

    //         print!("\n");
    //     }

    //     print!("\n");
    // }

    fn first_non_zero(vec: &Vec<i32>) -> usize {
        vec.iter().position(|&i| i != 0).map(|position| position).unwrap_or(vec.len())
    }

    fn exact_div(a: i32, b: i32) -> i32 {
        let div = a.div_euclid(b);
        let rem = a.rem_euclid(b);

        assert_eq!(rem, 0);

        div
    }

    for (machine_index, machine) in input.into_iter().enumerate() {
        // Building matrix
        let mut matrix = vec![vec![0; machine.wiring_schematics.len() + 1]; machine.joltage_requirements.len()];

        for (wiring_index, wiring_schematics) in machine.wiring_schematics.iter().enumerate() {
            for &jotlage_index in wiring_schematics {
                matrix[jotlage_index as usize][wiring_index] = 1;
            }
        }

        for (joltage_index, &joltage_value) in machine.joltage_requirements.iter().enumerate() {
            matrix[joltage_index][machine.wiring_schematics.len()] = joltage_value as i32;
        }

        // Diagonalization
        for i in 0..std::cmp::min(machine.wiring_schematics.len(), machine.joltage_requirements.len()) {
            // Sort matrix
            matrix.sort_by_key(first_non_zero);

            // Normalise matrix a bit: this is not needed but helps debuging by having cleaner matrices
            if matrix[i][i] < 0 {
                matrix[i] = matrix[i].iter().map(|v| -v).collect();
            }

            if matrix[i][i] > 1 {
                let gcd = matrix[i].iter().cloned().filter(|&v| v != 0).reduce(|a, b| gcd(a.abs(), b.abs())).unwrap();
                matrix[i] = matrix[i].iter().map(|&v| exact_div(v, gcd)).collect();
            }

            let factor_a = matrix[i][i];

            if factor_a == 0 {
                // This is maybe fine
                continue;
            }

            for j in i+1..matrix.len() {
                let factor_b = matrix[j][i];

                if factor_b == 0 {
                    continue;
                }

                matrix[j] = (0..machine.wiring_schematics.len() + 1).map(|col| matrix[j][col] * factor_a - matrix[i][col] * factor_b).collect();
            }
        }

        // Remove nul rows
        while let Some(_) = matrix.pop_if(|row| row.iter().all(|&v| v == 0)) {}

        // Solving
        let maxes: Vec<_> = machine.wiring_schematics.iter()
            .map(|wiring_schemantic| wiring_schemantic.iter()
                .map(|&index| machine.joltage_requirements[index as usize])
                .min()
                .unwrap()
            )
            .collect();

        let mut solutions: Vec<Vec<Option<u16>>> = vec![vec![None; machine.wiring_schematics.len()]];

        for row in matrix.iter().rev() {
            let mut row = row.clone();
            let value = row.pop().unwrap();
            let coefs: Vec<_> = row.into_iter().enumerate().filter(|(_, v)| v != &0).collect();

            let mut new_solutions = Vec::new();

            for solution in solutions {
                let mut solutions_to_test = vec![solution.clone()];

                for (index, _) in &coefs {
                    if solution[*index].is_some() {
                        continue;
                    }

                    solutions_to_test = solutions_to_test.iter()
                        .flat_map(|solution_to_test| (0..=maxes[*index]).map(|value| {
                            let mut clone = solution_to_test.clone();

                            clone[*index] = Some(value);

                            clone
                        }))
                        .collect();
                }

                for solution_to_test in solutions_to_test {
                    let mut sum = 0;

                    for (index, coef) in &coefs {
                        sum += (solution_to_test[*index].unwrap() as i32) * coef;
                    }

                    if sum == value {
                        new_solutions.push(solution_to_test);
                    }
                }
            }

            solutions = new_solutions;
        }

        let min: u16 = solutions.iter()
            .map(|solution| solution.iter()
                .map(|count| count.unwrap())
                .sum()
            )
            .min()
            .unwrap();

        println!("Machine {}: {} solutions, min={}", machine_index + 1, solutions.len(), min);

        total += min;
    }

    total
}

fn main() {
    let input = from_str(&read(10).unwrap()).unwrap();

    println!("{}", solve_part1(&input));
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

    fn parsed_input() -> Input {
        Input::new([
            Machine::new([false, true, true, false], [vec![3], vec![1,3], vec![2], vec![2,3], vec![0,2], vec![0,1]], [3, 5, 4, 7]),
            Machine::new([false, false, false, true, false], [vec![0,2,3,4], vec![2,3], vec![0,4], vec![0,1,2], vec![1,2,3,4]], [7, 5, 12, 7, 2]),
            Machine::new([false, true, true, true, false, true], [vec![0,1,2,3,4], vec![0,3,4], vec![0,1,2,4,5], vec![1,2]], [10, 11, 11, 5, 10, 5]),
        ])
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse(INPUT), Ok(("", parsed_input())));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 7);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 33);
    }
}

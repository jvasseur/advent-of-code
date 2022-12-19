use advent_of_code_2022::{read, parse};
use nom::bytes::complete::tag;
use nom::character::complete::u8;
use nom::IResult;
use nom::multi:: many0;
use nom::sequence::terminated;

#[derive(Debug, PartialEq, Eq)]
struct Blueprint {
    id: u8,
    ore_robot_ore_cost: u8,
    clay_robot_ore_cost: u8,
    obsidian_robot_ore_cost: u8,
    obsidian_robot_clay_cost: u8,
    geode_robot_ore_cost: u8,
    geode_robot_obsidian_cost: u8,

    max_ore_cost: u8,
}

impl Blueprint {
    pub fn new(
        id: u8,
        ore_robot_ore_cost: u8,
        clay_robot_ore_cost: u8,
        obsidian_robot_ore_cost: u8,
        obsidian_robot_clay_cost: u8,
        geode_robot_ore_cost: u8,
        geode_robot_obsidian_cost: u8,
    ) -> Self {
        Self {
            id,
            ore_robot_ore_cost,
            clay_robot_ore_cost,
            obsidian_robot_ore_cost,
            obsidian_robot_clay_cost,
            geode_robot_ore_cost,
            geode_robot_obsidian_cost,

            max_ore_cost: [ore_robot_ore_cost, clay_robot_ore_cost, obsidian_robot_ore_cost, geode_robot_ore_cost].into_iter().max().unwrap(),
        }
    }

    pub fn max_ore_cost(&self) -> u8 {
        self.max_ore_cost
    }

    pub fn max_clay_cost(&self) -> u8 {
        self.obsidian_robot_clay_cost
    }

    pub fn max_obsidian_cost(&self) -> u8 {
        self.geode_robot_obsidian_cost
    }
}

fn blueprint_parser(input: &str) -> IResult<&str, Blueprint> {
    let (input, _) = tag("Blueprint ")(input)?;
    let (input, id) = u8(input)?;
    let (input, _) = tag(": Each ore robot costs ")(input)?;
    let (input, ore_robot_ore_cost) = u8(input)?;
    let (input, _) = tag(" ore. Each clay robot costs ")(input)?;
    let (input, clay_robot_ore_cost) = u8(input)?;
    let (input, _) = tag(" ore. Each obsidian robot costs ")(input)?;
    let (input, obsidian_robot_ore_cost) = u8(input)?;
    let (input, _) = tag(" ore and ")(input)?;
    let (input, obsidian_robot_clay_cost) = u8(input)?;
    let (input, _) = tag(" clay. Each geode robot costs ")(input)?;
    let (input, geode_robot_ore_cost) = u8(input)?;
    let (input, _) = tag(" ore and ")(input)?;
    let (input, geode_robot_obsidian_cost) = u8(input)?;
    let (input, _) = tag(" obsidian.")(input)?;

    Ok((input, Blueprint::new(
        id,
        ore_robot_ore_cost,
        clay_robot_ore_cost,
        obsidian_robot_ore_cost,
        obsidian_robot_clay_cost,
        geode_robot_ore_cost,
        geode_robot_obsidian_cost,
    )))
}

type Input = Vec<Blueprint>;

fn parser(input: &str) -> IResult<&str, Input> {
    many0(terminated(blueprint_parser, tag("\n")))(input)
}

#[derive(PartialEq, Eq)]
struct Factory<'a> {
    blueprint: &'a Blueprint,

    ore_robots: u8,
    clay_robots: u8,
    obsidian_robots: u8,
    geode_robots: u16,

    ore: u8,
    clay: u8,
    obsidian: u8,
    geodes: u16,
}

impl<'a> Factory<'a> {
    pub fn new(blueprint: &'a Blueprint) -> Self {
        Self {
            blueprint,

            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,

            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
        }
    }

    pub fn next_states(&self) -> Vec<Factory<'a>> {
        let mut states = Vec::new();
        let mut can_wait_ressources = false;

        if self.ore >= self.blueprint.geode_robot_ore_cost && self.obsidian >= self.blueprint.geode_robot_obsidian_cost {
            states.push(Factory {
                blueprint: self.blueprint,

                ore_robots: self.ore_robots,
                clay_robots: self.clay_robots,
                obsidian_robots: self.obsidian_robots,
                geode_robots: self.geode_robots + 1,

                ore: self.ore - self.blueprint.geode_robot_ore_cost + self.ore_robots,
                clay: self.clay + self.clay_robots,
                obsidian: self.obsidian - self.blueprint.geode_robot_obsidian_cost + self.obsidian_robots,
                geodes: self.geodes + self.geode_robots,
            });

            return states;
        } else {
            if self.obsidian_robots > 0 {
                can_wait_ressources = true;
            }
        }

        if self.ore_robots < self.blueprint.max_ore_cost() {
            if self.ore >= self.blueprint.ore_robot_ore_cost {
                states.push(Factory {
                    blueprint: self.blueprint,

                    ore_robots: self.ore_robots + 1,
                    clay_robots: self.clay_robots,
                    obsidian_robots: self.obsidian_robots,
                    geode_robots: self.geode_robots,

                    ore: self.ore - self.blueprint.ore_robot_ore_cost + self.ore_robots,
                    clay: self.clay + self.clay_robots,
                    obsidian: self.obsidian + self.obsidian_robots,
                    geodes: self.geodes + self.geode_robots,
                });
            } else {
                can_wait_ressources = true;
            }
        }

        if self.clay_robots < self.blueprint.max_clay_cost() {
            if self.ore >= self.blueprint.clay_robot_ore_cost {
                states.push(Factory {
                    blueprint: self.blueprint,

                    ore_robots: self.ore_robots,
                    clay_robots: self.clay_robots + 1,
                    obsidian_robots: self.obsidian_robots,
                    geode_robots: self.geode_robots,

                    ore: self.ore - self.blueprint.clay_robot_ore_cost  + self.ore_robots,
                    clay: self.clay + self.clay_robots,
                    obsidian: self.obsidian + self.obsidian_robots,
                    geodes: self.geodes + self.geode_robots,
                });
            } else {
                can_wait_ressources = true;
            }
        }

        if self.obsidian_robots < self.blueprint.max_obsidian_cost() {
            if self.ore >= self.blueprint.obsidian_robot_ore_cost && self.clay >= self.blueprint.obsidian_robot_clay_cost {
                states.push(Factory {
                    blueprint: self.blueprint,

                    ore_robots: self.ore_robots,
                    clay_robots: self.clay_robots,
                    obsidian_robots: self.obsidian_robots + 1,
                    geode_robots: self.geode_robots,

                    ore: self.ore - self.blueprint.obsidian_robot_ore_cost + self.ore_robots,
                    clay: self.clay - self.blueprint.obsidian_robot_clay_cost + self.clay_robots,
                    obsidian: self.obsidian + self.obsidian_robots,
                    geodes: self.geodes + self.geode_robots,
                });
            } else {
                if self.clay_robots > 0 {
                    can_wait_ressources = true;
                }
            }
        }

        if can_wait_ressources {
            states.push(Factory {
                blueprint: self.blueprint,

                ore_robots: self.ore_robots,
                clay_robots: self.clay_robots,
                obsidian_robots: self.obsidian_robots,
                geode_robots: self.geode_robots,

                ore: self.ore + self.ore_robots,
                clay: self.clay + self.clay_robots,
                obsidian: self.obsidian + self.obsidian_robots,
                geodes: self.geodes + self.geode_robots,
            });
        }

        states
    }
}

fn get_best_geodes_count(factory: Factory, remaining_time: u8) -> u16 {
    if remaining_time == 0 {
        return factory.geodes;
    }

    factory
        .next_states()
        .into_iter()
        .map(|next_state| get_best_geodes_count(next_state, remaining_time - 1))
        .max()
        .unwrap()
}

fn solve_part1(input: &Input) -> u16 {
    input
        .into_iter()
        .map(|blueprint| get_best_geodes_count(Factory::new(blueprint), 24) * blueprint.id as u16)
        .sum()
}

fn solve_part2(input: &Input) -> u16 {
    input
        .into_iter()
        .take(3)
        .map(|blueprint| get_best_geodes_count(Factory::new(blueprint), 32))
        .product()
}

fn main() {
    let input = read(19);

    let parsed = parse(parser, &input);

    println!("{}", solve_part1(&parsed));
    println!("{}", solve_part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

    fn parsed_input() -> Input {
        vec![
            Blueprint::new(1, 4, 2, 3, 14, 2, 7),
            Blueprint::new(2, 2, 3, 3, 8, 3, 12),
        ]
    }

    #[test]
    fn test_parser() {
        assert_eq!(parser(INPUT), Ok(("", parsed_input())));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parsed_input()), 33);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&parsed_input()), 56 * 62);
    }
}

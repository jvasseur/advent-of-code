use advent_of_code_2021::{parse, read};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::IResult;
use std::collections::HashMap;
use std::collections::HashSet;

fn parse_type(input: &str) -> IResult<&str, Type> {
    alt((
        value(Type::Amber, tag("A")),
        value(Type::Bronze, tag("B")),
        value(Type::Copper, tag("C")),
        value(Type::Desert, tag("D")),
    ))(input)
}

fn parser(input: &str) -> IResult<&str, Map<2>> {
    // Line 1
    let (input, _) = tag("#############\n")(input)?;
    // Line 2
    let (input, _) = tag("#...........#\n")(input)?;
    // Line 3
    let (input, _) = tag("###")(input)?;
    let (input, t11) = parse_type(input)?;
    let (input, _) = tag("#")(input)?;
    let (input, t12) = parse_type(input)?;
    let (input, _) = tag("#")(input)?;
    let (input, t13) = parse_type(input)?;
    let (input, _) = tag("#")(input)?;
    let (input, t14) = parse_type(input)?;
    let (input, _) = tag("###\n")(input)?;
    // Line 4
    let (input, _) = tag("  #")(input)?;
    let (input, t21) = parse_type(input)?;
    let (input, _) = tag("#")(input)?;
    let (input, t22) = parse_type(input)?;
    let (input, _) = tag("#")(input)?;
    let (input, t23) = parse_type(input)?;
    let (input, _) = tag("#")(input)?;
    let (input, t24) = parse_type(input)?;
    let (input, _) = tag("#\n")(input)?;
    // Line 5
    let (input, _) = tag("  #########\n")(input)?;

    Ok((input, Map {
        corridor: [None, None, None, None, None, None, None],
        rooms: [
            [Some(t11), Some(t21)],
            [Some(t12), Some(t22)],
            [Some(t13), Some(t23)],
            [Some(t14), Some(t24)],
        ]
    }))
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Map<const SIZE: usize> {
    corridor: [Option<Type>; 7],
    rooms: [[Option<Type>; SIZE]; 4],
}

fn d(t: &Option<Type>) -> char {
    match t {
        None => '.',
        Some(u) => match u {
            Type::Amber => 'A',
            Type::Bronze => 'B',
            Type::Copper => 'C',
            Type::Desert => 'D',
        }
    }
}

impl<const SIZE: usize> std::fmt::Display for Map<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#############")?;
        writeln!(f, "#{}{}.{}.{}.{}.{}{}#", d(&self.corridor[0]), d(&self.corridor[1]), d(&self.corridor[2]), d(&self.corridor[3]), d(&self.corridor[4]), d(&self.corridor[5]), d(&self.corridor[6]))?;

        for i in 0..SIZE {
            if i == 0 {
                write!(f, "##")?;
            } else {
                write!(f, "  ")?;
            }

            write!(f, "#{}#{}#{}#{}#", d(&self.rooms[0][i]), d(&self.rooms[1][i]), d(&self.rooms[2][i]), d(&self.rooms[3][i]))?;

            if i == 0 {
                writeln!(f, "##")?;
            } else {
                writeln!(f, "")?;
            }
        }

        write!(f, "  #########")
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum Type {
    Amber,
    Bronze,
    Copper,
    Desert,
}

fn move_energy(amphipod: &Type) -> u32 {
    match amphipod {
        Type::Amber => 1,
        Type::Bronze => 10,
        Type::Copper => 100,
        Type::Desert => 1000,
    }
}

static DESTINATIONS: [Type; 4] = [Type::Amber, Type::Bronze, Type::Copper, Type::Desert];

/*
#############
#01.2.3.4.56#
###0#.#.#.###
  #1#.#.#.#
  #########
*/
fn outsides(room: usize) -> [usize; 2] {
    [room + 1, room + 2]
}

fn move_cost(room: usize, depth: usize, corridor: usize, cost: u32) -> u32 {
    let outsides = outsides(room);

    let mut move_cost = 0;
    let mut position = corridor;

    if position == 0 {
        position += 1;
        move_cost += cost;
    }

    while position < outsides[0] {
        position += 1;
        move_cost += 2 * cost;
    }

    if position == 6 {
        position -= 1;
        move_cost += cost;
    }

    while position > outsides[1] {
        position -= 1;
        move_cost += 2 * cost;
    }

    move_cost += (depth as u32 + 2) * cost;

    move_cost
}

fn move_out<const SIZE: usize>(map: &Map<SIZE>, room: usize, depth: usize, corridor: usize) -> (Map<SIZE>, u32) {
    let mut new = map.clone();

    let am = map.rooms[room][depth].to_owned().unwrap();

    debug_assert!(map.corridor[corridor] == None, "Trying to move to an non-empty space");

    let cost = move_energy(&am);

    new.corridor[corridor] = Some(am);
    new.rooms[room][depth] = None;

    (new, move_cost(room, depth, corridor, cost))
}

fn move_in<const SIZE: usize>(map: &Map<SIZE>, corridor: usize, room: usize, depth: usize) -> (Map<SIZE>, u32) {
    let mut new = map.clone();

    let am = map.corridor[corridor].to_owned().unwrap();

    debug_assert!(map.rooms[room][depth] == None, "Trying to move to an non-empty space");

    let cost = move_energy(&am);

    new.corridor[corridor] = None;
    new.rooms[room][depth] = Some(am);

    (new, move_cost(room, depth, corridor, cost))
}

fn move_outside<const SIZE: usize>(moves: &mut Vec<(Map<SIZE>, u32)>, map: &Map<SIZE>, room: usize, start: usize) {
    let [mut left, mut right] = outsides(room);

    while None == map.corridor[left] {
        moves.push(move_out(map, room, start, left));

        if left == 0 {
            break;
        }

        left -= 1;
    }

    while None == map.corridor[right] {
        moves.push(move_out(map, room, start, right));

        if right == 6 {
            break;
        }

        right += 1;
    }
}

fn move_inside<const SIZE: usize>(moves: &mut Vec<(Map<SIZE>, u32)>, map: &Map<SIZE>, room: usize, depth: usize) {
    let [mut left, mut right] = outsides(room);

    loop {
        if None == map.corridor[left] {
            if left == 0 {
                break;
            } else {
                left -= 1;

                continue;
            }
        }

        if map.corridor[left] != Some(DESTINATIONS[room]) {
            break;
        }

        moves.push(move_in(map, left, room, depth));

        break;
    }

    loop {
        if None == map.corridor[right] {
            if right == 6 {
                break;
            } else {
                right += 1;

                continue;
            }
        }

        if map.corridor[right] != Some(DESTINATIONS[room]) {
            break;
        }

        moves.push(move_in(map, right, room, depth));

        break;
    }
}

/*
#############
#01.2.3.4.56#
###0#.#.#.###
  #1#.#.#.#
  #########
*/
fn moves<const SIZE: usize>(map: &Map<SIZE>) -> Vec<(Map<SIZE>, u32)> {
    let mut moves = Vec::new();

    for room in 0..4 {
        for i in 0..SIZE {
            if &None == &map.rooms[room][i] {
                // Nothing to move
                continue;
            }

            if (i..SIZE).all(|j| &map.rooms[room][j] == &Some(DESTINATIONS[room])) {
                // Everything is good, we shouldn't move anything
                break;
            }

            move_outside(&mut moves, map, room, i);

            break;
        }
    }

    for room in 0..4 {
        for i in 0..SIZE {
            if &None != &map.rooms[room][i] {
                // It's full !
                break;
            }

            // Everything under us is good
            if (i+1..SIZE).all(|j| &map.rooms[room][j] == &Some(DESTINATIONS[room])) {
                move_inside(&mut moves, map, room, i);

                break;
            }
        }
    }

    moves
}

fn is_win<const SIZE: usize>(map: &Map<SIZE>) -> bool {
    if map.corridor != [None, None, None, None, None, None, None] {
        return false;
    }

    for room in 0..4 {
        for i in 0..SIZE {
            if map.rooms[room][i] != Some(DESTINATIONS[room]) {
                return false;
            }
        }
    }

    return true;
}

fn all_min<'a, const SIZE: usize>(states: &HashMap<Map<SIZE>, u32>, unvisited: &HashSet<Map<SIZE>>) -> (Vec<Map<SIZE>>, u32) {
    let mut cost = &u32::MAX;
    let mut maps = Vec::new();

    for map in unvisited {
        let map_cost = states.get(map).unwrap();

        if map_cost < cost {
            maps = Vec::new();
            cost = map_cost;
        }

        if map_cost == cost {
            maps.push(map);
        }
    }

    (maps.into_iter().map(|map| map.to_owned()).collect(), *cost)
}

fn solve<const SIZE: usize>(input: &Map<SIZE>) -> u32 {
    let mut states = HashMap::new();
    let mut unvisited = HashSet::new();

    states.insert(input.to_owned(), 0_u32);
    unvisited.insert(input.to_owned());

    loop {
        //println!("{} {}", states.len(), unvisited.len());

        let (maps, cost) = all_min(&states, &unvisited);

        //println!("{}", cost);

        for map in maps {
            if is_win(&map) {
                return cost;
            }

            for (new_state, move_cost) in moves(&map) {
                let total_cost = cost + move_cost;

                if let Some(current_cost) = states.get(&new_state) {
                    if total_cost < *current_cost {
                        states.insert(new_state, total_cost);
                    }
                } else {
                    states.insert(new_state.clone(), total_cost);
                    unvisited.insert(new_state);
                }
            }

            unvisited.remove(&map);
        }
    }
}

fn main() {
    let input = read(23);

    let part_1_map = parse(parser, &input);
    let part_2_map = Map {
        corridor: [None, None, None, None, None, None, None],
        rooms: [
            [part_1_map.rooms[0][0], Some(Type::Desert), Some(Type::Desert), part_1_map.rooms[0][1]],
            [part_1_map.rooms[1][0], Some(Type::Copper), Some(Type::Bronze), part_1_map.rooms[1][1]],
            [part_1_map.rooms[2][0], Some(Type::Bronze), Some(Type::Amber), part_1_map.rooms[2][1]],
            [part_1_map.rooms[3][0], Some(Type::Amber), Some(Type::Copper), part_1_map.rooms[3][1]],
        ]
    };

    println!("Part 1: {}", solve(&part_1_map));
    println!("Part 2: {}", solve(&part_2_map));
}

#[cfg(test)]
mod tests {
    use super::outsides;
    use super::moves;
    use super::move_cost;
    use super::move_inside;
    use super::move_outside;
    use super::solve;
    use super::is_win;
    use super::parser;
    use super::Type;
    use super::Map;

    #[test]
    fn test_outsides() {
        // #############
        // #01.2.3.4.56#
        // ###0#1#2#3###
        //   #.#.#.#.#
        //   #########
        assert_eq!(outsides(0), [1, 2]);
        assert_eq!(outsides(1), [2, 3]);
        assert_eq!(outsides(2), [3, 4]);
        assert_eq!(outsides(3), [4, 5]);
    }

    #[test]
    fn test_move_cost() {
        // #############
        // #01.2.3.4.56#
        // ###0#1#2#3###
        //   #.#.#.#.#
        //   #########
        assert_eq!(move_cost(0, 0, 1, 1), 2);
        assert_eq!(move_cost(0, 1, 1, 1), 3);
        assert_eq!(move_cost(0, 0, 0, 1), 3);
        assert_eq!(move_cost(0, 1, 0, 1), 4);
        assert_eq!(move_cost(1, 0, 1, 1), 4);
    }

    #[test]
    fn test_parser() {
        let text = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
";

        assert_eq!(parser(text), Ok(("", Map {
            corridor: [None, None, None, None, None, None, None],
            rooms: [
                [Some(Type::Bronze), Some(Type::Amber)],
                [Some(Type::Copper), Some(Type::Desert)],
                [Some(Type::Bronze), Some(Type::Copper)],
                [Some(Type::Desert), Some(Type::Amber)],
            ],
        })));
    }

    #[test]
    fn test_move_inside_1() {
        let map = Map {
            corridor: [Some(Type::Bronze), None, None, None, None, None, None],
            rooms: [
                [None, Some(Type::Amber)],
                [Some(Type::Copper), Some(Type::Desert)],
                [Some(Type::Bronze), Some(Type::Copper)],
                [Some(Type::Desert), Some(Type::Amber)],
            ],
        };

        let mut moves = Vec::new();

        move_inside(&mut moves, &map, 0, 0);

        assert_eq!(moves, Vec::new());
    }

    #[test]
    fn test_move_inside_2() {
        let map = Map {
            corridor: [Some(Type::Amber), None, None, None, None, None, None],
            rooms: [
                [None, Some(Type::Amber)],
                [Some(Type::Copper), Some(Type::Desert)],
                [Some(Type::Bronze), Some(Type::Copper)],
                [Some(Type::Desert), Some(Type::Bronze)],
            ],
        };

        let mut moves = Vec::new();

        move_inside(&mut moves, &map, 0, 0);

        assert_eq!(moves, vec![
            (Map {
                corridor: [None, None, None, None, None, None, None],
                rooms: [
                    [Some(Type::Amber), Some(Type::Amber)],
                    [Some(Type::Copper), Some(Type::Desert)],
                    [Some(Type::Bronze), Some(Type::Copper)],
                    [Some(Type::Desert), Some(Type::Bronze)],
                ],
            }, 3),
        ]);
    }

    #[test]
    fn test_move_inside_3() {
        let map = Map {
            corridor: [None, None, None, None, None, None, Some(Type::Amber)],
            rooms: [
                [None, Some(Type::Amber)],
                [Some(Type::Copper), Some(Type::Desert)],
                [Some(Type::Bronze), Some(Type::Copper)],
                [Some(Type::Desert), Some(Type::Bronze)],
            ],
        };

        let mut moves = Vec::new();

        move_inside(&mut moves, &map, 0, 0);

        assert_eq!(moves, vec![
            (Map {
                corridor: [None, None, None, None, None, None, None],
                rooms: [
                    [Some(Type::Amber), Some(Type::Amber)],
                    [Some(Type::Copper), Some(Type::Desert)],
                    [Some(Type::Bronze), Some(Type::Copper)],
                    [Some(Type::Desert), Some(Type::Bronze)],
                ],
            }, 9),
        ]);
    }

    #[test]
    fn test_move_outside_1() {
        let map = Map {
            corridor: [None, None, Some(Type::Copper), None, None, None, None],
            rooms: [
                [Some(Type::Bronze), Some(Type::Amber)],
                [None, Some(Type::Desert)],
                [Some(Type::Bronze), Some(Type::Copper)],
                [Some(Type::Desert), Some(Type::Amber)],
            ],
        };

        let mut moves = Vec::new();

        move_outside(&mut moves, &map, 0, 0);

        assert_eq!(moves, vec![
            (Map {
                corridor: [None, Some(Type::Bronze), Some(Type::Copper), None, None, None, None],
                rooms: [
                    [None, Some(Type::Amber)],
                    [None, Some(Type::Desert)],
                    [Some(Type::Bronze), Some(Type::Copper)],
                    [Some(Type::Desert), Some(Type::Amber)],
                ],
            }, 20),
            (Map {
                corridor: [Some(Type::Bronze), None, Some(Type::Copper), None, None, None, None],
                rooms: [
                    [None, Some(Type::Amber)],
                    [None, Some(Type::Desert)],
                    [Some(Type::Bronze), Some(Type::Copper)],
                    [Some(Type::Desert), Some(Type::Amber)],
                ],
            }, 30),
        ]);
    }

    #[test]
    fn test_moves_1() {
        let map = Map {
            corridor: [None, None, None, None, None, None, None],
            rooms: [
                [Some(Type::Bronze), Some(Type::Amber)],
                [Some(Type::Copper), Some(Type::Desert)],
                [Some(Type::Bronze), Some(Type::Copper)],
                [Some(Type::Desert), Some(Type::Amber)],
            ],
        };

        let moves = moves(&map);

        assert_eq!(moves.len(), 4 * 7);
    }

    #[test]
    #[ignore]
    fn test_solve_part_1() {
        let map = Map {
            corridor: [None, None, None, None, None, None, None],
            rooms: [
                [Some(Type::Bronze), Some(Type::Amber)],
                [Some(Type::Copper), Some(Type::Desert)],
                [Some(Type::Bronze), Some(Type::Copper)],
                [Some(Type::Desert), Some(Type::Amber)],
            ],
        };

        assert_eq!(solve(&map), 12521);
    }

    #[test]
    #[ignore]
    fn test_solve_part_2() {
        let map = Map {
            corridor: [None, None, None, None, None, None, None],
            rooms: [
                [Some(Type::Bronze), Some(Type::Desert), Some(Type::Desert), Some(Type::Amber)],
                [Some(Type::Copper), Some(Type::Copper), Some(Type::Bronze), Some(Type::Desert)],
                [Some(Type::Bronze), Some(Type::Bronze), Some(Type::Amber), Some(Type::Copper)],
                [Some(Type::Desert), Some(Type::Amber), Some(Type::Copper), Some(Type::Amber)],
            ],
        };

        assert_eq!(solve(&map), 44169);
    }

    #[test]
    fn test_is_win_1() {
        let map = Map {
            corridor: [None, None, None, None, None, None, None],
            rooms: [
                [Some(Type::Bronze), Some(Type::Amber)],
                [Some(Type::Copper), Some(Type::Desert)],
                [Some(Type::Bronze), Some(Type::Copper)],
                [Some(Type::Desert), Some(Type::Amber)],
            ],
        };

        assert_eq!(is_win(&map), false);
    }

    #[test]
    fn test_is_win_2() {
        let map = Map {
            corridor: [None, None, None, None, None, None, None],
            rooms: [
                [Some(Type::Amber), Some(Type::Amber)],
                [Some(Type::Bronze), Some(Type::Bronze)],
                [Some(Type::Copper), Some(Type::Copper)],
                [Some(Type::Desert), Some(Type::Desert)],
            ],
        };

        assert_eq!(is_win(&map), true);
    }
}

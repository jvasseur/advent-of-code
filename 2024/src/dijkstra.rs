use core::hash::Hash;
use std::{cmp::Ordering, collections::{HashMap, BinaryHeap}};

pub struct Edge<T> {
    pub node: T,
    pub cost: u32,
}

#[derive(Eq, PartialEq)]
struct State<T> {
    cost: u32,
    position: T,
}

impl<T: Eq + PartialEq> Ord for State<T>  {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<T: Eq + PartialEq> PartialOrd for State<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn shortest_path<T: Eq + Clone + Hash>(
    starts: impl IntoIterator<Item = T>,
    get_edges: impl Fn(&T) -> Vec<Edge<T>>,
    is_goal: impl Fn(&T) -> bool,
) -> Option<u32> {
    let mut distances = HashMap::new();
    let mut heap = BinaryHeap::new();

    for start in starts {
        distances.insert(start.clone(), 0);
        heap.push(State { cost: 0, position: start });
    }

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if is_goal(&position) {
            return Some(cost);
        }

        if let Some(&position_cost) = distances.get(&position) {
            if cost > position_cost {
                continue;
            }
        }

        for edge in get_edges(&position) {
            let next_cost = cost + edge.cost;
            let next_position = edge.node;

            if let Some(&position_cost) = distances.get(&next_position) {
                if next_cost >= position_cost {
                    continue;
                }
            }

            distances.insert(next_position.clone(), next_cost);
            heap.push(State { cost: next_cost, position: next_position });
        }
    }

    None
}

pub fn get_paths<T: Eq + Clone + Hash>(
    starts: impl IntoIterator<Item = T>,
    get_edges: impl Fn(&T) -> Vec<Edge<T>>,
    is_goal: impl Fn(&T) -> bool
) -> Option<Vec<Vec<T>>> {
    let mut distances = HashMap::new();
    let mut paths = HashMap::new();
    let mut heap = BinaryHeap::new();

    for start in starts {
        distances.insert(start.clone(), 0);
        paths.insert(start.clone(), vec![vec![start.clone()]]);
        heap.push(State { cost: 0, position: start });
    }

    let mut goal_cost = None;
    let mut goal_paths = vec![];

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        if let Some(max) = goal_cost {
            if cost > max {
                return Some(goal_paths);
            }
        }

        if is_goal(&position) {
            goal_cost = Some(cost);
            goal_paths = [goal_paths, paths.get(&position).unwrap().to_owned()].concat();
        }

        if let Some(&position_cost) = distances.get(&position) {
            if cost > position_cost {
                continue;
            }
        }

        for edge in get_edges(&position) {
            let next_cost = cost + edge.cost;
            let next_position = edge.node.clone();
            let mut next_paths: Vec<Vec<T>> = paths.get(&position).unwrap().iter().map(|path| [path.to_owned(), vec![edge.node.clone()]].concat()).collect();

            if let Some(&position_cost) = distances.get(&next_position) {
                if next_cost >= position_cost {
                    if next_cost == position_cost {
                        paths.get_mut(&next_position).unwrap().append(&mut next_paths);
                    }

                    continue;
                }
            }

            distances.insert(next_position.clone(), next_cost);
            paths.insert(next_position.clone(), next_paths);
            heap.push(State { cost: next_cost, position: next_position });
        }
    }

    None
}

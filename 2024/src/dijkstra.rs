use core::hash::Hash;
use std::{cmp::Ordering, collections::{HashMap, BinaryHeap}};

pub trait Node: Sized + Clone + Eq + PartialEq + Hash {
    fn edges(&self) -> Vec<Edge<Self>>;
}

pub struct Edge<T: Node> {
    pub node: T,
    pub cost: u32,
}

#[derive(Eq, PartialEq)]
struct State<T: Node> {
    cost: u32,
    position: T,
}

impl<T: Node> Ord for State<T>  {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<T: Node> PartialOrd for State<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn shortest_path<T: Node>(starts: impl IntoIterator<Item = T>, is_goal: impl Fn(&T) -> bool) -> Option<u32> {
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

        for edge in position.edges() {
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

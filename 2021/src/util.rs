use std::collections::HashMap;
use std::collections::hash_map::Values;
use std::hash::Hash;
use std::ops::Add;

/// Recreation of the abs_diff function to allow using it while staying on stable
pub fn abs_diff<T, U>(a: T, b: T) -> U
where
    T: std::cmp::Ord + std::ops::Sub<Output = U>,
{
    if a < b {
        b - a
    } else {
        a - b
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Counter<T>
where
    T: Hash + Eq + Copy,
{
    storage: HashMap<T, usize>,
}

impl<T> Counter<T>
where
    T: Hash + Eq + Copy,
{
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub fn increment(&mut self, value: &T) {
        self.add_count(value, 1);
    }

    pub fn decrement(&mut self, value: &T) {
        self.remove_count(value, 1);
    }

    pub fn get(&self, value: &T) -> usize {
        match self.storage.get(value) {
            None => 0,
            Some(count) => *count,
        }
    }

    pub fn values(&self) -> Values<T, usize> {
        self.storage.values()
    }

    fn add_count(&mut self, value: &T, count: usize) {
        match self.storage.get_mut(value) {
            None => {
                self.storage.insert(*value, count);
            },
            Some(current) => {
                *current += count;
            },
        };
    }

    fn remove_count(&mut self, value: &T, count: usize) {
        match self.storage.get_mut(value) {
            None => panic!("Reaching negative count"),
            Some(current) => {
                *current -= count;
            },
        };
    }
}

impl<T> Add for Counter<T>
where
    T: Hash + Eq + Copy,
{
    type Output = Counter<T>;

    fn add(self, rhs: Counter<T>) -> Counter<T> {
        let mut sum = Counter::new();

        for (key, value) in self.storage {
            sum.add_count(&key, value);
        }

        for (key, value) in rhs.storage {
            sum.add_count(&key, value);
        }

        sum
    }
}

impl<T, const N: usize> From<[(T, usize); N]> for Counter<T>
where
    T: Hash + Eq + Copy,
{
    fn from(arr: [(T, usize); N]) -> Self {
        Self {
            storage: HashMap::from(arr),
        }
    }
}

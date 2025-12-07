use std::{collections::HashMap, hash::Hash};

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

    pub fn get(&self, value: &T) -> usize {
        match self.storage.get(value) {
            None => 0,
            Some(count) => *count,
        }
    }

    pub fn counts<'a>(&'a self) -> impl Iterator<Item=(&'a T, &'a usize)> {
        self.storage.iter()
    }

    pub fn increment(&mut self, value: &T) {
        self.add_count(value, 1);
    }

    pub fn decrement(&mut self, value: &T) {
        self.remove_count(value, 1);
    }

    pub fn add_count(&mut self, value: &T, count: usize) {
        match self.storage.get_mut(value) {
            None => {
                self.storage.insert(*value, count);
            },
            Some(current) => {
                *current += count;
            },
        };
    }

    pub fn remove_count(&mut self, value: &T, count: usize) {
        match self.storage.get_mut(value) {
            None => panic!("Reaching negative count"),
            Some(current) => {
                *current -= count;
            },
        };
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

use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Deref, DerefMut},
};

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Counter<T>(HashMap<T, u64>);

impl<T: Hash + Eq> Counter<T> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add(&mut self, key: T, count: u64) {
        *self.0.entry(key).or_default() += count;
    }

    pub fn inc(&mut self, key: T) {
        self.add(key, 1);
    }

    pub fn min(&self) -> (&T, &u64) {
        self.iter()
            .reduce(|(s, c), (t, d)| if c < d { (s, c) } else { (t, d) })
            .unwrap()
    }

    pub fn max(&self) -> (&T, &u64) {
        self.iter()
            .reduce(|(s, c), (t, d)| if c > d { (s, c) } else { (t, d) })
            .unwrap()
    }

    pub fn count(&mut self, iter: impl Iterator<Item = T>) {
        iter.for_each(|t| self.inc(t));
    }

    pub fn add_from(&mut self, other: Self) {
        for (k, v) in other.0 {
            self.add(k, v);
        }
    }
}

impl<T> Hash for Counter<T>
where
    T: Hash + Ord,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for (k, v) in self.0.iter().sorted_by_key(|(k, _)| *k) {
            k.hash(state);
            v.hash(state);
        }
    }
}

impl<T> Deref for Counter<T> {
    type Target = HashMap<T, u64>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Counter<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

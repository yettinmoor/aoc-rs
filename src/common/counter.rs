use std::{collections::HashMap, hash::Hash};

pub trait Countable {
    type Item;
    fn counter(&mut self) -> HashMap<Self::Item, usize>;
}

impl<T: Eq + Hash, I: Iterator<Item = T>> Countable for I {
    type Item = T;
    fn counter(&mut self) -> HashMap<T, usize> {
        let mut count = HashMap::new();
        for k in self {
            count.entry(k).and_modify(|v| *v += 1).or_insert(1);
        }
        count
    }
}

pub trait CounterSum {
    type Item;
    fn counter_sum(&mut self) -> HashMap<Self::Item, usize>;
}

impl<T: Eq + Hash, I: Iterator<Item = (T, usize)>> CounterSum for I {
    type Item = T;
    fn counter_sum(&mut self) -> HashMap<T, usize> {
        let mut count = HashMap::new();
        for (k, v) in self {
            count.entry(k).and_modify(|ov| *ov += v).or_insert(v);
        }
        count
    }
}

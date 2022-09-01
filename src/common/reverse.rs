use std::collections::HashMap;

pub trait Reverse {
    fn reverse(&self) -> Self;
}

impl<K, V> Reverse for HashMap<K, V>
where
    K: Copy,
    V: Copy,
    HashMap<K, V>: FromIterator<(V, K)>,
{
    fn reverse(&self) -> Self {
        self.iter().map(|(&k, &v)| (v, k)).collect()
    }
}

use std::hash::Hash;
use indexmap::IndexMap;

pub struct FixedMap<K, V> {
    pub map: IndexMap<K, V>,
    capacity: usize,
}

impl<K, V> FixedMap<K, V>
    where K: Eq + Hash,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: IndexMap::with_capacity(capacity),
        }
    }

    /// Inserts an item, if the map is not at capacity. Otherwise do nothing.
    /// TODO remove oldest values? (IndexMap is used instead of HashMap so this is possible)
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.map.len() >= self.capacity {
            None
        } else {
            self.map.insert(key, value)
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}
use std::vec::IntoIter;

use super::{Key, Value};

#[derive(Debug)]
pub enum OpType<K: Key, V: Value> {
    Insert((K, V)),
}

#[derive(Debug, Default)]
pub struct Batch<K: Key, V: Value>(Vec<OpType<K, V>>);

impl<K: Key, V: Value> Batch<K, V> {
    pub fn add_insert(&mut self, k: K, v: V) {
        self.0.push(OpType::Insert((k, v)));
    }
    pub fn into_iter(self) -> IntoIter<OpType<K, V>> {
        self.0.into_iter()
    }
}

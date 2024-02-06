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
}
impl<K: Key, V: Value> Iterator for Batch<K, V> {
    type Item = OpType<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.remove(0))
        }
    }
}

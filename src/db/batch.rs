use std::{ops::DerefMut, vec::IntoIter};

use super::{Key, Value};
use crate::prelude::*;

#[derive(Debug)]
pub enum OpType<K: Key, V: Value> {
    Insert((K, V)),
    Delete(K),
}

#[derive(Debug, Default)]
pub struct Batch<K: Key, V: Value>(Vec<OpType<K, V>>);

impl<K: Key, V: Value> IntoIterator for Batch<K, V> {
    type Item = OpType<K, V>;
    type IntoIter = IntoIter<OpType<K, V>>;

    fn into_iter(self) -> IntoIter<OpType<K, V>> {
        self.0.into_iter()
    }
}

impl<K: Key, V: Value> Deref for Batch<K, V> {
    type Target = Vec<OpType<K, V>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<K: Key, V: Value> DerefMut for Batch<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K: Key, V: Value> Batch<K, V> {
    fn add_insert(mut self, k: K, v: V) -> Self {
        self.push(OpType::Insert((k, v)));
        self
    }

    fn add_delete<K2: Into<K>>(mut self, k: K2) -> Self {
        self.push(OpType::Delete(k.into()));
        self
    }
}

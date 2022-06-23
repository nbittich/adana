use crate::prelude::*;

use super::{Key, Op, Value};

type InnerMap<K, V> = HashMap<K, V>;
#[derive(Debug, Deserialize, Serialize)]
pub struct Tree<K: Key, V: Value>(InnerMap<K, V>);

impl<K: Key, V: Value> Default for Tree<K, V> {
    fn default() -> Tree<K, V> {
        Tree(HashMap::new())
    }
}

impl<K: Key, V: Value> Deref for Tree<K, V> {
    type Target = InnerMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K: Key + Clone, V: Value> Op<K, V> for Tree<K, V> {
    fn read(
        &self,
        k: impl Into<K>,
        mapper: impl Fn(&V) -> Option<V>,
    ) -> Option<V> {
        let v = self.get(&k.into())?;
        mapper(v)
    }

    fn insert(&mut self, k: impl Into<K>, v: impl Into<V>) -> Option<V> {
        self.0.insert(k.into(), v.into())
    }

    fn remove(&mut self, k: impl Into<K>) -> Option<V> {
        self.0.remove(&k.into())
    }

    fn clear(&mut self) {
        self.0.clear();
    }

    fn contains(&self, k: &K) -> Option<bool> {
        Some(self.contains_key(k))
    }

    fn len(&self) -> Option<usize> {
        Some(self.0.len())
    }

    fn keys(&self) -> Vec<K> {
        self.0.keys().into_iter().cloned().collect()
    }

    fn list_all(&self) -> HashMap<K, V> {
        self.0.clone()
    }
}

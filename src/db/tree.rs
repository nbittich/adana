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

impl<K: Key + Clone, V: Value + Clone> Op<K, V> for Tree<K, V> {
    fn read<E, K2: Into<K>>(
        &self,
        k: K2,
        mapper: impl Fn(&V) -> Option<E>,
    ) -> Option<E> {
        let v = self.get(&k.into())?;
        mapper(v)
    }

    fn insert<K2: Into<K>, V2: Into<V>>(&mut self, k: K2, v: V2) -> Option<V> {
        self.0.insert(k.into(), v.into())
    }

    fn remove<K2: Into<K>>(&mut self, k: K2) -> Option<V> {
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

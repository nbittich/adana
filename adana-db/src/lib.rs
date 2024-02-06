mod batch;
mod database;
mod file_db;
mod file_lock;
mod in_memory;
mod tree;

pub use batch::*;
pub use database::*;
pub use file_db::*;
pub use file_lock::*;
pub use in_memory::*;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::hash::Hash;
pub use tree::Tree;

pub trait Key:
    Hash + Eq + Send + Clone + Serialize + Debug + Sync + Ord
{
}
pub trait Value: Serialize + Send + Clone + Debug + Sync {}

impl<T: Hash + Eq + Send + Clone + Serialize + Debug + Sync + Ord> Key for T {}
impl<T: Serialize + Send + Clone + Debug + Sync> Value for T {}
pub trait Op<K: Key, V: Value> {
    fn read(&self, k: impl Into<K>, r: impl Fn(&V) -> Option<V>) -> Option<V>;

    fn get_value(&self, k: impl Into<K>) -> Option<V> {
        self.read(k, |v| Some(v.clone()))
    }
    fn list_all(&self) -> BTreeMap<K, V>;

    fn read_no_op(
        &self,
        k: impl Into<K>,
        r: impl Fn(&V) -> Option<V>,
    ) -> Option<V> {
        self.read(k.into(), |v| {
            r(v);
            None
        })
    }
    fn keys(&self) -> Vec<K>;
    fn insert(&mut self, k: impl Into<K>, v: impl Into<V>) -> Option<V>;
    fn remove(&mut self, k: impl Into<K>) -> Option<V>;
    fn clear(&mut self);
    fn contains(&self, k: &K) -> Option<bool>;
    fn len(&self) -> Option<usize>;
    fn is_empty(&self) -> bool {
        self.len().filter(|&s| s > 0).is_none()
    }
}
pub trait DbOp<K: Key, V: Value>: Op<K, V> {
    fn get_current_tree(&self) -> Option<String>;

    fn flush(&self) -> anyhow::Result<&'static str> {
        Ok("sync not implemented")
    }

    fn open_tree(&mut self, tree_name: &str) -> Option<bool>;

    fn tree_names(&self) -> Vec<String>;

    fn drop_tree(&mut self, tree_name: &str) -> bool;

    fn clear_tree(&mut self, tree_name: &str) -> bool;

    fn merge_trees(
        &mut self,
        tree_name_source: &str,
        tree_name_dest: &str,
    ) -> Option<()>;

    fn merge_current_tree_with(&mut self, tree_name_source: &str)
        -> Option<()>;

    fn apply_batch(&mut self, batch: Batch<K, V>) -> Option<()>;

    fn apply_tree(
        &mut self,
        tree_name: &str,
        consumer: &mut impl FnMut(&mut Tree<K, V>) -> Option<V>,
    ) -> Option<V>;

    fn open_tree_and_apply(
        &mut self,
        tree_name: &str,
        consumer: &mut impl FnMut(&mut Tree<K, V>) -> Option<V>,
    ) -> Option<V> {
        self.open_tree(tree_name);
        self.apply_tree(tree_name, consumer)
    }
}

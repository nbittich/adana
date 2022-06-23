mod batch;
mod file_db;
mod file_lock;
mod in_memory;
mod tree;

pub use batch::*;
pub use file_db::*;
pub use file_lock::*;
pub use in_memory::*;
pub use tree::Tree;

use crate::prelude::*;
use std::fmt::Debug;

pub trait Key: Hash + Eq + Send + Clone + Serialize + Debug {}
pub trait Value: Serialize + Send + Clone + Debug {}

impl<T: Hash + Eq + Send + Clone + Serialize + Debug> Key for T {}
impl<T: Serialize + Send + Clone + Debug> Value for T {}
pub trait Op<K: Key, V: Value> {
    fn read(&self, k: impl Into<K>, r: impl Fn(&V) -> Option<V>) -> Option<V>;

    fn get_value(&self, k: impl Into<K>) -> Option<V> {
        self.read(k, |v| Some(v.clone()))
    }
    fn list_all(&self) -> HashMap<K, V>;

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
}
pub trait DbOp<K: Key, V: Value>: Op<K, V> {
    fn get_current_tree(&self) -> Option<String>;

    fn open_tree(&mut self, tree_name: &str);

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

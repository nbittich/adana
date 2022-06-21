mod batch;
mod file_db;
mod file_lock;
mod in_memory;
mod tree;

pub use batch::*;
pub use file_db::*;
pub use file_lock::*;
pub use in_memory::*;

use crate::prelude::*;

pub trait Key: Hash + Eq + Send + Clone + Serialize {}
pub trait Value: Serialize + Send + Clone {}

impl<T: Hash + Eq + Send + Clone + Serialize> Key for T {}
impl<T: Serialize + Send + Clone> Value for T {}

pub trait Op<K: Key, V: Value> {
    fn read<E, K2: Into<K>>(
        &self,
        k: K2,
        r: impl Fn(&V) -> Option<E>,
    ) -> Option<E>;

    fn get_value<E, K2: Into<K>>(&self, k: K2) -> Option<V> {
        self.read(k, |v| Some(v.clone()))
    }
    fn list_all(&self) -> HashMap<K, V>;

    fn read_no_op<K2: Into<K>>(&self, k: K2, r: impl Fn(&V)) -> Option<()> {
        self.read(k.into(), |v| {
            r(v);
            Some(())
        })
    }
    fn keys(&self) -> Vec<K>;
    fn insert<K2: Into<K>, V2: Into<V>>(&mut self, k: K2, v: V2) -> Option<V>;
    fn remove<K2: Into<K>>(&mut self, k: K2) -> Option<V>;
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
}

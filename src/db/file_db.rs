use std::{
    path::Path,
    sync::mpsc::{Receiver, Sender},
    thread::JoinHandle,
    vec,
};

use serde::de::DeserializeOwned;

use crate::prelude::*;

use super::{DbOp, FileLock, InMemoryDb, Key, Op, Value};

enum Notify {
    Update,
    Stop,
}

#[derive(Debug)]
pub struct FileDb<K: Key, V: Value> {
    __inner: Arc<Mutex<InMemoryDb<K, V>>>,
    __event_sender: Sender<Notify>,
    __thread_handle: Option<JoinHandle<()>>,
    config: Arc<Config>,
}
#[derive(Debug)]
pub struct Config {
    pub file_lock: FileLock,
}

trait GuardedDb<K: Key, V: Value> {
    fn get_guard(&self) -> Option<MutexGuard<InMemoryDb<K, V>>>;
    fn get_sender(&self) -> &Sender<Notify>;
    fn update<E, F: FnOnce(MutexGuard<InMemoryDb<K, V>>) -> Option<E>>(
        &self,
        f: F,
    ) -> Option<E> {
        let guard = self.get_guard()?;
        let sender = self.get_sender();
        sender.send(Notify::Update).ok()?;
        f(guard)
    }
}

impl<K: Key, V: Value> GuardedDb<K, V> for FileDb<K, V> {
    fn get_guard(&self) -> Option<MutexGuard<InMemoryDb<K, V>>> {
        match self.__inner.lock() {
            Ok(lock) => Some(lock),
            Err(e) => {
                error!("Lock could not be acquired! {e}");
                None
            }
        }
    }

    fn get_sender(&self) -> &Sender<Notify> {
        &self.__event_sender
    }
}

impl<K: Key, V: Value> DbOp<K, V> for FileDb<K, V> {
    fn get_current_tree(&self) -> Option<String> {
        let guard = self.get_guard()?;
        guard.get_current_tree()
    }

    fn open_tree(&mut self, tree_name: &str) -> Option<bool> {
        let mut guard = self.get_guard()?;
        let res = guard.open_tree(tree_name)?;
        if res {
            self.__event_sender.send(Notify::Update).ok()?;
        }
        Some(res)
    }

    fn tree_names(&self) -> Vec<String> {
        if let Some(guard) = self.get_guard() {
            guard.tree_names()
        } else {
            vec![]
        }
    }

    fn drop_tree(&mut self, tree_name: &str) -> bool {
        if let Some(res) =
            self.update(|mut guard| Some(guard.drop_tree(tree_name)))
        {
            res
        } else {
            false
        }
    }

    fn clear_tree(&mut self, tree_name: &str) -> bool {
        if let Some(res) =
            self.update(|mut guard| Some(guard.clear_tree(tree_name)))
        {
            res
        } else {
            false
        }
    }

    fn merge_trees(
        &mut self,
        tree_name_source: &str,
        tree_name_dest: &str,
    ) -> Option<()> {
        self.update(|mut guard| {
            guard.merge_trees(tree_name_source, tree_name_dest)
        })
    }

    fn merge_current_tree_with(
        &mut self,
        tree_name_source: &str,
    ) -> Option<()> {
        self.update(|mut guard| guard.merge_current_tree_with(tree_name_source))
    }

    fn apply_batch(&mut self, batch: super::Batch<K, V>) -> Option<()> {
        self.update(|mut guard| guard.apply_batch(batch))
    }

    fn apply_tree(
        &mut self,
        tree_name: &str,
        consumer: &mut impl FnMut(&mut super::tree::Tree<K, V>) -> Option<V>,
    ) -> Option<V> {
        self.update(|mut guard| guard.apply_tree(tree_name, consumer))
    }
}

impl<K: Key, V: Value> Op<K, V> for FileDb<K, V> {
    fn read(&self, k: impl Into<K>, r: impl Fn(&V) -> Option<V>) -> Option<V> {
        let guard = self.get_guard()?;
        guard.read(k, r)
    }

    fn insert(&mut self, k: impl Into<K>, v: impl Into<V>) -> Option<V> {
        self.update(move |mut guard| guard.insert(k, v))
    }

    fn remove(&mut self, k: impl Into<K>) -> Option<V> {
        self.update(move |mut guard| guard.remove(k))
    }

    fn clear(&mut self) {
        self.update(|mut guard| {
            guard.clear();
            Some(())
        });
    }

    fn contains(&self, k: &K) -> Option<bool> {
        let guard = self.get_guard()?;
        guard.contains(k)
    }

    fn len(&self) -> Option<usize> {
        let guard = self.get_guard()?;
        guard.len()
    }

    fn keys(&self) -> Vec<K> {
        if let Some(guard) = self.get_guard() {
            guard.keys()
        } else {
            vec![]
        }
    }

    fn list_all(&self) -> BTreeMap<K, V> {
        if let Some(guard) = self.get_guard() {
            guard.list_all()
        } else {
            BTreeMap::default()
        }
    }
}

impl<K, V> FileDb<K, V>
where
    K: 'static + Key + DeserializeOwned + std::fmt::Debug,
    V: 'static + Value + DeserializeOwned + std::fmt::Debug,
{
    pub fn open<P: AsRef<Path>>(path: P) -> anyhow::Result<FileDb<K, V>> {
        let file_lock = FileLock::open(path)?;

        Self::open_with_config(Config { file_lock })
    }

    pub fn open_temporary() -> anyhow::Result<InMemoryDb<K, V>> {
        Ok(Default::default())
    }

    pub fn open_with_config(config: Config) -> anyhow::Result<FileDb<K, V>> {
        let __inner = {
            let reader = config.file_lock.read()?;
            if let Ok(inner_db) =
                bincode::deserialize_from::<_, InMemoryDb<K, V>>(reader)
            {
                Arc::new(Mutex::new(inner_db))
            } else {
                Arc::new(Mutex::new(Default::default()))
            }
        };
        let (__event_sender, receiver) = std::sync::mpsc::channel();
        let mut db = FileDb {
            config: Arc::new(config),
            __inner,
            __event_sender,
            __thread_handle: None,
        };
        db.start_file_db(receiver)?;
        Ok(db)
    }

    fn __flush(
        inner_db: Arc<Mutex<InMemoryDb<K, V>>>,
        file_lock: &FileLock,
    ) -> anyhow::Result<()> {
        trace!("syncing");
        let db =
            inner_db.lock().map_err(|e| anyhow::Error::msg(e.to_string()))?;
        let bytes = bincode::serialize(&*db)?;
        drop(db); // try to release the lock before writing to the file
        let _ = file_lock.write(&bytes)?;
        trace!("syncing done");
        Ok(())
    }
    fn start_file_db(
        &mut self,
        receiver: Receiver<Notify>,
    ) -> anyhow::Result<()> {
        let clone = Arc::clone(&self.__inner);
        let config = self.config.clone();

        let handle = std::thread::spawn(move || {
            debug!("start syncing");

            for event in receiver.iter() {
                match event {
                    Notify::Update => {
                        info!("receive update!");
                        if let Err(e) =
                            Self::__flush(Arc::clone(&clone), &config.file_lock)
                        {
                            error!("could not flush db. Err: '{e}'.");
                        } else {
                            trace!("sync done");
                        }
                    }
                    Notify::Stop => {
                        info!("receive stop!");
                        break;
                    }
                }
            }

            debug!("DROPPED");

            if let Err(e) = Self::__flush(clone, &config.file_lock) {
                error!("could not flush db. Err: '{e}'.");
            }
        });

        self.__thread_handle = Some(handle);
        Ok(())
    }
}
impl<K: Key, V: Value> Drop for FileDb<K, V> {
    fn drop(&mut self) {
        debug!("done");
        self.__event_sender
            .send(Notify::Stop)
            .expect("could not send stop event!!!");
        if let Some(handle) = self.__thread_handle.take() {
            handle.join().expect("Could not cleanup thread handle!!!");
        }
        debug!("cleanup file db success!")
    }
}

#[cfg(test)]
mod test {

    use crate::db::file_db::{Config, FileDb};
    use crate::db::file_lock::FileLock;
    use crate::db::DbOp;
    use crate::prelude::*;

    use super::Op;

    #[derive(Serialize, Debug, PartialEq)]
    struct MyString(String);

    #[test]
    fn test_file_db_lock() {
        let _ = File::create("/tmp/karsher.db").unwrap();

        let mut file_db: FileDb<u64, String> =
            FileDb::open_with_config(Config {
                file_lock: FileLock::open("/tmp/karsher.db").unwrap(),
            })
            .unwrap();

        file_db.open_tree("rust");

        for i in 1..100u64 {
            file_db.insert(i, format!("ok mani{i}"));
            file_db.insert(i * 100, format!("ok rebenga{i}"));
            std::thread::sleep(Duration::from_millis(100));
        }
        assert_eq!(Some(198), file_db.len());

        drop(file_db); // force destroying the object to flush db

        let mut file_db: FileDb<u64, String> =
            FileDb::open_with_config(Config {
                file_lock: FileLock::open("/tmp/karsher.db").unwrap(),
            })
            .unwrap();

        file_db.open_tree("rust");

        file_db.insert(3991u64, format!("new!"));

        assert_eq!(Some(199), file_db.len());
    }
}

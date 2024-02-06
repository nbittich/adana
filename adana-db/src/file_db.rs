use std::{
    collections::BTreeMap,
    path::PathBuf,
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Mutex, MutexGuard,
    },
    thread::JoinHandle,
    vec,
};

use log::{debug, error, trace};
use serde::de::DeserializeOwned;

use super::{DbOp, FileLock, InMemoryDb, Key, Op, Value};

pub(super) enum Notify {
    Update,
    FullFlush,
    Stop,
}

#[derive(Debug)]
pub struct FileDb<K: Key, V: Value> {
    __inner: Arc<Mutex<InMemoryDb<K, V>>>,
    __event_sender: Sender<Notify>,
    __thread_handle: Option<JoinHandle<()>>,
    __file_lock: Arc<FileLock>,
}

#[derive(Debug)]
pub(super) struct FileDbConfig<K: Key, V: Value> {
    pub(super) inner: Arc<Mutex<InMemoryDb<K, V>>>,
    pub(super) file_lock: Arc<FileLock>,
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

impl<K, V> DbOp<K, V> for FileDb<K, V>
where
    K: 'static + Key + DeserializeOwned + std::fmt::Debug,
    V: 'static + Value + DeserializeOwned + std::fmt::Debug,
{
    fn get_current_tree(&self) -> Option<String> {
        let guard = self.get_guard()?;
        guard.get_current_tree()
    }

    fn flush(&self) -> anyhow::Result<&'static str> {
        match self.get_sender().send(Notify::FullFlush) {
            Ok(_) => Ok("notify db to update itself"),
            Err(e) => Err(anyhow::Error::from(e)),
        }
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
    pub fn get_path(&self) -> &PathBuf {
        self.__file_lock.get_path()
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
        file_lock.write(&bytes)?;
        trace!("syncing done");
        Ok(())
    }
    fn start_file_db(
        &mut self,
        receiver: Receiver<Notify>,
    ) -> anyhow::Result<()> {
        let clone = Arc::clone(&self.__inner);
        let file_lock = self.__file_lock.clone();

        let handle = std::thread::spawn(move || {
            debug!("start syncing");

            for event in receiver.iter() {
                match event {
                    Notify::Update => {
                        debug!("receive update!");
                        if let Err(e) =
                            Self::__flush(Arc::clone(&clone), &file_lock)
                        {
                            error!("could not flush db. Err: '{e}'.");
                        } else {
                            trace!("sync done");
                        }
                    }
                    Notify::FullFlush => {
                        debug!("receive full flush!");
                        if let Err(e) =
                            Self::__flush(Arc::clone(&clone), &file_lock)
                        {
                            error!("could not flush db. Err: '{e}'.");
                        } else if let Err(e) = file_lock.flush() {
                            error!("could not write on file lock {e}");
                        } else {
                            trace!("full flush done");
                        }
                    }
                    Notify::Stop => {
                        debug!("receive stop!");
                        break;
                    }
                }
            }

            debug!("DROPPED");

            if let Err(e) = Self::__flush(clone, &file_lock) {
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

impl<K, V> TryFrom<FileDbConfig<K, V>> for FileDb<K, V>
where
    K: 'static + Key + DeserializeOwned + std::fmt::Debug,
    V: 'static + Value + DeserializeOwned + std::fmt::Debug,
{
    type Error = anyhow::Error;

    fn try_from(config: FileDbConfig<K, V>) -> Result<Self, Self::Error> {
        let (__event_sender, receiver) = std::sync::mpsc::channel();
        let mut db = FileDb {
            __file_lock: config.file_lock,
            __inner: config.inner,
            __event_sender,
            __thread_handle: None,
        };
        db.start_file_db(receiver)?;
        Ok(db)
    }
}

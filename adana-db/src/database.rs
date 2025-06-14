use std::{
    fmt::Display,
    path::Path,
    sync::{Arc, Mutex},
};

use crate::FileDbConfig;
use anyhow::Context;
use log::debug;
use serde::de::DeserializeOwned;

const ADANA_DB_DIR: &str = "adana/db";

use super::{
    FileDb, FileLock, FileLockError, InMemoryDb, Key, Value, file_lock,
};

fn get_default_db_path() -> Option<Box<Path>> {
    let mut db_dir = dirs::data_dir().or_else(dirs::home_dir)?;
    db_dir.push(ADANA_DB_DIR);
    debug!("db dir: {}", db_dir.as_path().to_string_lossy());
    if !db_dir.exists() {
        std::fs::create_dir_all(&db_dir).ok()?;
    }
    db_dir.push("adana.db");

    Some(db_dir.into_boxed_path())
}

#[derive(Debug)]
pub struct Config {
    path: Option<Box<Path>>,
    in_memory: bool,
    fall_back_in_memory: bool,
}
impl Config {
    pub fn new<P: AsRef<Path>>(
        path: Option<P>,
        in_memory: bool,
        fall_back_in_memory: bool,
    ) -> Config {
        if in_memory {
            Config { in_memory, path: None, fall_back_in_memory: false }
        } else {
            Config {
                in_memory,
                path: path
                    .map(|p| {
                        let path: Box<Path> = p.as_ref().into();
                        path
                    })
                    .or_else(get_default_db_path),
                fall_back_in_memory,
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path: get_default_db_path(),
            in_memory: false,
            fall_back_in_memory: true,
        }
    }
}

pub enum Db<K: Key, V: Value> {
    FileBased(FileDb<K, V>),
    InMemory(InMemoryDb<K, V>),
}

impl<K, V> Db<K, V>
where
    K: 'static + Key + DeserializeOwned + std::fmt::Debug,
    V: 'static + Value + DeserializeOwned + std::fmt::Debug,
{
    fn in_memory_fallback(e: impl Display) -> anyhow::Result<Db<K, V>> {
        eprintln!("Warning! {e} \nAttempt to open a temporary db...\n",);
        Ok(Db::InMemory(Default::default()))
    }
    pub fn open(config: Config) -> anyhow::Result<Db<K, V>> {
        if config.in_memory {
            return Ok(Db::InMemory(Default::default()));
        }
        let path = config.path.context("not in memory but path empty")?;

        let file_lock = FileLock::open(path.as_ref());
        match file_lock {
            Err(e) if !config.fall_back_in_memory => {
                Err(anyhow::Error::msg(e.to_string()))
            }
            Err(pid_exist @ FileLockError::PidExist(_)) => {
                eprintln!(
                    "Warning! {pid_exist} \nAttempt to open a temporary db...\n",
                );
                let p = path.as_ref();
                let pb = p.to_path_buf();
                match file_lock::read_file(&pb) {
                    Ok(reader) => {
                        match bincode::deserialize_from::<_, InMemoryDb<K, V>>(
                            reader,
                        ) {
                            Ok(inner_db) => Ok(Db::InMemory(inner_db)),
                            Err(e) => {
                                eprintln!(
                                    "Warning! {e:?} \nAttempt to deserialize db, could be because it is the first time you use it\n",
                                );
                                Self::in_memory_fallback(e)
                            }
                        }
                    }
                    Err(e) if config.fall_back_in_memory => {
                        Self::in_memory_fallback(e)
                    }
                    Err(e) => Err(e),
                }
            }
            Err(e) => Self::in_memory_fallback(e),
            Ok(file_lock) => {
                let inner = match file_lock.read() {
                    Ok(reader) => {
                        match bincode::deserialize_from::<_, InMemoryDb<K, V>>(
                            reader,
                        ) {
                            Ok(inner_db) => Arc::new(Mutex::new(inner_db)),
                            Err(e) => {
                                eprintln!(
                                    "Warning! {e:?} \nAttempt to deserialize db, could be because it is the first time you use it\n",
                                );
                                Arc::new(Mutex::new(Default::default()))
                            }
                        }
                    }
                    Err(e) if config.fall_back_in_memory => {
                        return Self::in_memory_fallback(e);
                    }
                    Err(e) => return Err(e),
                };

                let db_config =
                    FileDbConfig { file_lock: Arc::new(file_lock), inner };
                match FileDb::try_from(db_config) {
                    Ok(file_db) => Ok(Db::FileBased(file_db)),
                    Err(e) if config.fall_back_in_memory => {
                        Self::in_memory_fallback(e)
                    }
                    Err(e) => Err(e),
                }
            }
        }
    }
}

#[cfg(test)]
mod test {

    use std::fs::File;

    use crate::{Config, Db, DbOp, Op};

    #[test]
    fn test_file_db_lock() {
        let _ = File::create("/tmp/adana.db"); // reset the file

        let file_db: Db<u64, String> =
            Db::open(Config::new(Some("/tmp/adana.db"), false, false)).unwrap();

        let mut file_db = match file_db {
            Db::FileBased(file_db) => file_db,
            _ => {
                panic!("error, should be file db")
            }
        };
        file_db.open_tree("rust");

        for i in 1..100u64 {
            file_db.insert(i, format!("ok mani{i}"));
            file_db.insert(i * 100, format!("ok rebenga{i}"));
        }
        assert_eq!(Some(198), file_db.len());

        drop(file_db); // force destroying the object to flush db

        let file_db: Db<u64, String> =
            Db::open(Config::new(Some("/tmp/adana.db"), false, false)).unwrap();

        let mut file_db = match file_db {
            Db::FileBased(file_db) => file_db,
            _ => {
                panic!("error, should be file db")
            }
        };

        file_db.open_tree("rust");

        file_db.insert(39912u64, "new!".to_string());

        assert_eq!(Some(199), file_db.len());
    }
}

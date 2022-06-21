use std::path::Path;

use serde::{Deserialize, Serialize};
use sled::{Batch, Config, Db, IVec, Tree};

pub use crate::prelude::*;
use crate::utils::calculate_hash;

// todo this is not exposed by sled
const SLED_DEFAULT_TREE_ID: &[u8] = b"__sled__default";

const DEFAULT_CACHE_KEY: &[u8] = b"$___DEF_CACHE_KEY_LOC___$";

lazy_static::lazy_static! {
    static ref DB_FILE_PATH: PathBuf = {
        let mut db_dir = dirs::data_dir().expect("db not found");
        db_dir.push(".karsher");
        if !db_dir.exists() {
            std::fs::create_dir(&db_dir).expect("could not create db directory");
        }
        db_dir
    };

}
#[derive(Serialize, Deserialize)]
struct CacheJson {
    name: String,
    values: HashMap<String, String>,
}

#[derive(Debug)]
pub struct CacheManager {
    _db_inner: Db,
}

impl Default for CacheManager {
    fn default() -> Self {
        let _db_inner = {
            match sled::open(DB_FILE_PATH.as_path()) {
                Ok(db) => db,
                Err(e) => {
                    eprintln!(
                        "{} {e} \nAttempt to open a temporary db..\n.",
                        colors::Red.paint("Warning!")
                    );
                    Config::new().temporary(true).open().unwrap()
                }
            }
        };

        Self { _db_inner }
    }
}

fn i_vec_to_string(v: &IVec) -> String {
    String::from_utf8_lossy(v).to_string()
}

impl CacheManager {
    fn check_cache_name(&self, cache_name: &str) -> Option<()> {
        if cache_name.as_bytes() != SLED_DEFAULT_TREE_ID {
            Some(())
        } else {
            println!("{} you cannot do this.", colors::Red.paint("Warning!"));
            None
        }
    }

    pub fn restore(&self, path: &Path) -> Option<()> {
        let file = File::open(path).ok()?;
        let buf_reader = BufReader::new(file);
        let caches: Vec<CacheJson> =
            serde_json::from_reader(buf_reader).ok()?;
        for cache in caches {
            let mut batch = Batch::default();
            let tree = self
                .get_cache(&cache.name)
                .ok_or("cache could not be inserted")
                .ok()?;
            for (key, value) in cache.values {
                batch.insert(key.as_bytes(), IVec::from(value.as_bytes()));
            }
            tree.apply_batch(batch).ok()?;
            tree.flush().ok()?;
        }
        Some(())
    }

    pub fn backup(&self, path: &Path) -> Option<()> {
        let json = self.dump(None)?;
        std::fs::write(path, json).ok()
    }

    fn is_cache_exist(&self, ns: &str) -> bool {
        self.get_cache_names().iter().any(|c| c == ns)
    }

    pub fn dump(&self, namespace: Option<&str>) -> Option<String> {
        let tree_to_json = |(ns, tree)| {
            let values: HashMap<String, String> =
                self.list_values_from_tree(&tree)?.into_iter().collect();
            serde_json::to_string_pretty(&CacheJson { name: ns, values }).ok()
        };
        if let Some(ns) = namespace {
            if !self.is_cache_exist(ns) {
                None
            } else if let Some((ns, tree)) = self.get_cache(ns).map(|t| (ns, t))
            {
                tree_to_json((ns.to_string(), tree))
            } else {
                None
            }
        } else {
            let caches: Vec<String> = self
                .get_cache_names()
                .iter()
                .filter_map(|n| self.get_cache(n).map(|t| (n, t)))
                .filter_map(|(n, t)| tree_to_json((n.clone(), t)))
                .collect();

            Some(format!("[{}]", caches.join(",\n")))
        }
    }

    fn get_cache(&self, key: &str) -> Option<Tree> {
        self.check_cache_name(key)?;
        self._db_inner.open_tree(key).ok()
    }

    pub fn get_default_cache(&self) -> Option<String> {
        self._db_inner.get(DEFAULT_CACHE_KEY).ok()?.map(|v| i_vec_to_string(&v))
    }

    pub fn set_default_cache(&self, default_cache: &str) -> Option<()> {
        self.check_cache_name(default_cache)?;
        let _ = self._db_inner.insert(DEFAULT_CACHE_KEY, default_cache).ok()?;

        Some(())
    }

    pub fn merge(&self, key_1: &str, key_2: &str) -> Option<()> {
        if !self.is_cache_exist(key_1) || !self.is_cache_exist(key_2) {
            return None;
        }

        let tree_1 = self.get_cache(key_1)?;
        let tree_2 = self.get_cache(key_2)?;

        for r in tree_1.iter() {
            let (k, v) = r.ok()?;
            tree_2.insert(k, v).ok()?;
        }
        Some(())
    }

    pub fn get_cache_names(&self) -> Vec<String> {
        self._db_inner
            .tree_names()
            .iter()
            .filter(|v| v != &SLED_DEFAULT_TREE_ID)
            .map(i_vec_to_string)
            .collect()
    }

    pub fn remove_cache(&self, cache_name: &str) -> Option<bool> {
        self.check_cache_name(cache_name);
        self._db_inner.drop_tree(cache_name).ok()
    }

    pub fn clear_values(&self, cache_name: &str) -> Option<()> {
        let tree = self.get_cache(cache_name)?;
        tree.clear().ok()
    }

    pub fn insert_value(
        &self,
        namespace: &str,
        aliases: Vec<&str>,
        value: &str,
    ) -> Option<u64> {
        let tree = self.get_cache(namespace)?;
        let mut batch = Batch::default();

        let i_vec = IVec::from(value);

        let uniq_id = calculate_hash(&value);

        batch.insert(uniq_id.to_string().as_bytes(), &i_vec);

        let aliases: Vec<&str> = aliases
            .iter()
            .filter_map(|alias| {
                if !tree.contains_key(*alias).ok()? {
                    Some(*alias)
                } else {
                    None
                }
            })
            .collect();

        for hash_alias in &aliases {
            batch.insert(hash_alias.as_bytes(), &i_vec);
        }
        tree.apply_batch(batch).ok()?;
        tree.flush().ok()?;
        Some(uniq_id)
    }

    pub fn get_value(&self, namespace: &str, key: &str) -> Option<String> {
        let tree = self.get_cache(namespace)?;

        let value = &tree.get(key).ok()??;
        Some(i_vec_to_string(value))
    }

    pub fn list_values(
        &self,
        namespace: &str,
    ) -> Option<Vec<(String, String)>> {
        let tree = self.get_cache(namespace)?;
        self.list_values_from_tree(&tree)
    }

    fn list_values_from_tree(
        &self,
        tree: &Tree,
    ) -> Option<Vec<(String, String)>> {
        let len = &tree.len();

        let mut values = Vec::with_capacity(*len);

        for r in tree.iter() {
            let (k, v) = r.ok()?;
            values.push((i_vec_to_string(&k), i_vec_to_string(&v)));
        }

        Some(values)
    }

    pub fn remove_value(&self, namespace: &str, key: &str) -> Option<String> {
        let tree = self.get_cache(namespace)?;
        let value = tree.get(key).ok()??;
        let keys: Vec<IVec> = tree
            .iter()
            .filter_map(|r| r.ok())
            .filter_map(|(k, v)| if v == value { Some(k) } else { None })
            .collect();

        for key in keys {
            tree.remove(key).ok()?;
        }

        Some(i_vec_to_string(&value))
    }

    pub fn flush_sync(&self) -> anyhow::Result<()> {
        self._db_inner.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use sled::IVec;

    use crate::cache::i_vec_to_string;

    #[test]
    fn sled_test() {
        let db = sled::open("/tmp/sled_test").unwrap();

        let tree = db.open_tree("x").unwrap();

        let v = IVec::from("wesh");
        tree.insert("general", &v).unwrap();
        tree.insert("aliased", &v).unwrap();
        tree.insert("aliased2", &v).unwrap();
        tree.insert("aliased3", &v).unwrap();

        println!(
            "{:?}",
            i_vec_to_string(&tree.get("general").unwrap().unwrap())
        );
        tree.insert("general", "woops").unwrap();
        println!(
            "{:?}",
            i_vec_to_string(&tree.get("general").unwrap().unwrap())
        );
    }
}

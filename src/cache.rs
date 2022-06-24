use std::path::Path;

pub use crate::prelude::*;
use crate::{
    db::{Batch, DbOp, Op, Tree, DEFAULT_TREE},
    utils::calculate_hash,
};

const DEFAULT_CACHE_KEY: &str = "$___DEF_CACHE_KEY_LOC___$";

pub fn get_value(
    db: &mut impl DbOp<String, String>,
    namespace: &str,
    key: &str,
) -> Option<String> {
    db.open_tree(namespace)?;
    db.get_value(key)
}

pub fn list_values(
    db: &mut impl DbOp<String, String>,
    namespace: &str,
) -> Option<Vec<(String, String)>> {
    db.open_tree(namespace)?;
    Some(db.list_all().into_iter().collect())
}

pub fn remove_value(
    db: &mut impl DbOp<String, String>,
    namespace: &str,
    key: &str,
) -> Option<String> {
    let mut consumer = |tree: &mut Tree<String, String>| {
        let value = tree.get_value(key)?;
        let to_delete: Vec<String> = tree
            .iter()
            .filter_map(|(k, v)| if v == &value { Some(k) } else { None })
            .cloned()
            .collect();
        for k in to_delete {
            tree.remove(&*k)?;
        }
        Some(value)
    };

    db.apply_tree(namespace, &mut consumer)
}

pub fn insert_value(
    db: &mut impl DbOp<String, String>,
    namespace: &str,
    aliases: Vec<&str>,
    value: &str,
) -> Option<String> {
    db.open_tree(namespace)?;
    let mut batch = crate::db::Batch::default();
    let keys = db.keys();

    let aliases: Vec<&str> = aliases
        .iter()
        .filter_map(|alias| {
            if keys.contains(&alias.to_string()) {
                None
            } else {
                Some(*alias)
            }
        })
        .collect();

    for hash_alias in &aliases {
        batch.add_insert(hash_alias.to_string(), value.to_string());
    }

    if aliases.is_empty() {
        let uniq_id = calculate_hash(&value);

        batch.add_insert(uniq_id.to_string(), value.to_string());
    }

    db.apply_batch(batch)?;

    Some(aliases.join(", "))
}

pub fn clear_values(db: &mut impl DbOp<String, String>, cache_name: &str) {
    if db.open_tree(cache_name).is_some() {
        db.clear();
    }
}

pub fn remove_cache(
    db: &mut impl DbOp<String, String>,
    cache_name: &str,
) -> bool {
    db.drop_tree(cache_name)
}

pub fn get_cache_names(db: &mut impl DbOp<String, String>) -> Vec<String> {
    db.tree_names().into_iter().filter(|v| v != DEFAULT_TREE).collect()
}

pub fn merge(
    db: &mut impl DbOp<String, String>,
    key_1: &str,
    key_2: &str,
) -> Option<()> {
    check_cache_name(key_1)?;
    check_cache_name(key_2)?;
    db.merge_trees(key_1, key_2)
}

pub fn set_default_cache(
    db: &mut impl DbOp<String, String>,
    default_cache: &str,
) -> Option<()> {
    check_cache_name(default_cache)?;
    db.open_tree(DEFAULT_TREE)?;
    let _ = db.insert(DEFAULT_CACHE_KEY, default_cache);
    db.open_tree(default_cache)?;
    Some(())
}

pub fn get_default_cache(db: &mut impl DbOp<String, String>) -> Option<String> {
    db.open_tree(DEFAULT_TREE)?;
    db.get_value(DEFAULT_CACHE_KEY)
}

pub fn dump(
    db: &mut impl DbOp<String, String>,
    namespace: Option<&str>,
) -> Option<String> {
    if let Some(ns) = namespace {
        db.apply_tree(ns, &mut move |t| {
            serde_json::to_string_pretty(&CacheJson {
                name: ns.to_string(),
                values: t.list_all(),
            })
            .ok()
        })
    } else {
        let caches: Vec<String> = get_cache_names(db)
            .iter()
            .filter_map(|ns| {
                db.apply_tree(ns, &mut move |t| {
                    serde_json::to_string_pretty(&CacheJson {
                        name: ns.clone(),
                        values: t.list_all(),
                    })
                    .ok()
                })
            })
            .collect();

        Some(format!("[{}]", caches.join(",\n")))
    }
}

pub fn backup(db: &mut impl DbOp<String, String>, path: &Path) -> Option<()> {
    let json = dump(db, None)?;
    std::fs::write(path, json).ok()
}

pub fn restore(db: &mut impl DbOp<String, String>, path: &Path) -> Option<()> {
    let file = File::open(path).ok()?;
    let buf_reader = BufReader::new(file);
    let caches: Vec<CacheJson> = serde_json::from_reader(buf_reader).ok()?;
    for cache in caches {
        let mut batch = Batch::default();
        db.open_tree(&cache.name)?;

        for (key, value) in cache.values {
            batch.add_insert(key, value);
        }
        db.apply_batch(batch)?;
    }
    Some(())
}

fn check_cache_name(cache_name: &str) -> Option<()> {
    if cache_name != DEFAULT_TREE {
        Some(())
    } else {
        println!("{} you cannot do this.", colors::Red.paint("Warning!"));
        None
    }
}

#[derive(Serialize, Deserialize)]
struct CacheJson {
    name: String,
    values: BTreeMap<String, String>,
}

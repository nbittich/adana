use serde::{Deserialize, Serialize};

pub use crate::prelude::*;
use crate::utils::calculate_hash;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Cache {
    cache: BTreeMap<u64, String>,
    cache_aliases: BTreeMap<String, u64>,
}
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CacheManager {
    default_cache: Option<String>,
    caches: HashMap<String, Cache>,
}

impl CacheManager {
    pub fn get_mut_or_insert(&mut self, key: &str) -> Option<&mut Cache> {
        if !self.caches.contains_key(key) {
            let _ = self.caches.insert(key.into(), Default::default());
        }
        self.caches.get_mut(key)
    }

    pub fn get_default_cache(&self) -> &Option<String> {
        &self.default_cache
    }

    pub fn set_default_cache(&mut self, default_cache: &str) {
        self.default_cache = Some(String::from(default_cache));
    }

    pub fn get(&self, key: &str) -> Option<&Cache> {
        self.caches.get(key)
    }

    pub fn get_cache_names(&self) -> Vec<&String> {
        self.caches.keys().into_iter().collect()
    }

    pub fn remove_cache(&mut self, cache_name: &str) -> Option<Cache> {
        self.caches.remove(cache_name)
    }
}

impl Cache {
    pub fn insert(&mut self, aliases: Vec<&str>, value: &str) -> u64 {
        let key = calculate_hash(&value);
        self.cache.insert(key, value.to_owned());

        let aliases: Vec<&str> = aliases
            .iter()
            .filter_map(|alias| {
                if !self.cache_aliases.contains_key(*alias) {
                    Some(*alias)
                } else {
                    None
                }
            })
            .collect();

        for hash_alias in &aliases {
            self.cache_aliases.insert(hash_alias.to_string(), key);
        }
        key
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        let parsed_key = {
            if let Some(actual_key) = self.cache_aliases.get(key) {
                Some(*actual_key)
            } else {
                key.parse::<u64>().ok()
            }
        };

        if let Some(key) = parsed_key && let Some(value) = self.cache.get(&key) {
            Some(value)
        } else {
            None
        }
    }

    pub fn list(&self) -> Vec<(&u64,&String, Vec<&String>)> {
        self.cache
            .iter()
            .map(|(key, value)| {
                (
                    key,
                    value,
                    self.cache_aliases
                        .iter()
                        .filter_map(move |(alias, k)| if key == k { Some(alias) } else { None })
                        .collect(),
                )
            })
            .collect()
    }
    pub fn remove(&mut self, key: &str) -> Option<String> {
        let key = {
            if let Some(actual_key) = self.cache_aliases.remove(key) {
                Some(actual_key)
            } else {
                key.parse::<u64>().ok()
            }
        };

        if let Some(key) = key && let Some(v) = self.cache.remove(&key) {
            self.cache_aliases = self.cache_aliases.drain_filter(|_, v| v != &key)
            .collect();
            Some(v)
        }else {
            None
        }
    }
}

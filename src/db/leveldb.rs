use crate::db::Database;
use ethereum_types::U256;
use leveldb::database;
use leveldb::kv::KV;
use leveldb::options::{Options, ReadOptions, WriteOptions};
use std::path;

extern crate db_key as key;
use key::Key;

struct MyKey(U256);

impl Key for MyKey {
    fn from_u8(key: &[u8]) -> Self {
        Self(U256::from_big_endian(key))
    }

    fn as_slice<T, F: Fn(&[u8]) -> T>(&self, f: F) -> T {
        let mut slice = [0u8; 32];
        self.0.to_big_endian(&mut slice);
        f(&slice)
    }
}

pub struct LevelDBDatabase {
    db: database::Database<MyKey>,
}

impl LevelDBDatabase {
    /// Creates a new LevelDB file backed database instance.
    pub fn new(path: &path::Path) -> Self {
        let mut options = Options::new();
        options.create_if_missing = true;
        Self {
            db: database::Database::open(path, options).unwrap(),
        }
    }
}

impl Database for LevelDBDatabase {
    fn get(&self, key: U256) -> U256 {
        match self.db.get(ReadOptions::new(), &MyKey(key)).unwrap() {
            None => 0.into(),
            Some(v) => MyKey::from_u8(&v).0,
        }
    }

    fn set(&mut self, key: U256, value: U256) {
        let wo = WriteOptions::new();
        if value == U256::default() {
            self.db.delete(wo, &MyKey(key)).unwrap()
        } else {
            MyKey(value).as_slice(|v| self.db.put(wo, MyKey(key), v).unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn test_memory_database_empty() {
        let dir = TempDir::new("leveldbtest").unwrap();
        let db = LevelDBDatabase::new(&dir.path());
        assert_eq!(db.get(999.into()), 0.into());
    }

    #[test]
    fn test_memory_database_get_set() {
        let dir = TempDir::new("leveldbtest").unwrap();
        let mut db = LevelDBDatabase::new(&dir.path());
        db.set(123.into(), 456.into());
        assert_eq!(db.get(123.into()), 456.into());
        assert_eq!(db.get(124.into()), 0.into());
        db.set(123.into(), 789.into());
        assert_eq!(db.get(123.into()), 789.into());
        assert_eq!(db.get(124.into()), 0.into());
        db.set(123.into(), 0.into());
        assert_eq!(db.get(123.into()), 0.into());
    }
}

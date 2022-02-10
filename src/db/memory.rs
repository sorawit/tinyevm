use crate::db::Database;
use ethereum_types::U256;
use std::collections::HashMap;

pub struct MemoryDatabase {
    db: HashMap<U256, U256>,
}

impl MemoryDatabase {
    /// Creates a new in-memory database.
    pub fn new() -> Self {
        Self { db: HashMap::new() }
    }
}

impl Database for MemoryDatabase {
    fn get(&self, key: U256) -> U256 {
        self.db.get(&key).cloned().unwrap_or_default()
    }

    fn set(&mut self, key: U256, value: U256) {
        if value == U256::default() {
            self.db.remove(&key);
        } else {
            self.db.insert(key, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_database_empty() {
        let db = MemoryDatabase::new();
        assert_eq!(db.get(999.into()), 0.into());
    }

    #[test]
    fn test_memory_database_get_set() {
        let mut db = MemoryDatabase::new();
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

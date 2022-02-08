use crate::database::Database;
use ethereum_types::U256;
use std::collections::HashMap;

pub struct State<DB> {
    db: DB,
    cache: HashMap<U256, U256>, // TODO: More optimization?
}

impl<DB: Database> State<DB> {
    /// Creates a new state based on the given database.
    pub fn new(db: DB) -> Self {
        Self {
            db,
            cache: HashMap::new(),
        }
    }

    /// Returns the value at the specified key from this state.
    pub fn load(&self, key: U256) -> U256 {
        match self.cache.get(&key) {
            Some(value) => value.into(),
            None => self.db.get(key),
        }
    }

    /// Stores the given key-value to the pending change set.
    pub fn store(&mut self, key: U256, value: U256) {
        self.cache.insert(key, value);
    }

    /// Reverts all the pending changes and goes back to database state.
    pub fn rollback(&mut self) {
        self.cache.clear()
    }

    /// Commits all the pending changes to the database.
    pub fn commit(&mut self) {
        self.cache.drain().for_each(|(k, v)| self.db.set(k, v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::MemoryDatabase;

    #[test]
    fn test_state_load_store() {
        let mut db = MemoryDatabase::new();
        db.set(123.into(), 456.into());
        let mut st = State::new(db);
        assert_eq!(st.load(123.into()), 456.into());
        assert_eq!(st.load(124.into()), 0.into());
        st.store(123.into(), 457.into());
        st.store(124.into(), 458.into());
        assert_eq!(st.load(123.into()), 457.into());
        assert_eq!(st.load(124.into()), 458.into());
        assert_eq!(st.load(125.into()), 0.into());
    }

    #[test]
    fn test_state_rollback() {
        let mut db = MemoryDatabase::new();
        db.set(123.into(), 456.into());
        let mut st = State::new(db);
        st.store(123.into(), 457.into());
        assert_eq!(st.load(123.into()), 457.into());
        st.rollback();
        assert_eq!(st.load(123.into()), 456.into());
    }

    #[test]
    fn test_state_commit() {
        let mut db = MemoryDatabase::new();
        db.set(123.into(), 456.into());
        let mut st = State::new(db);
        st.store(123.into(), 457.into());
        assert_eq!(st.load(123.into()), 457.into());
        st.commit();
        st.rollback();
        assert_eq!(st.load(123.into()), 457.into());
    }
}

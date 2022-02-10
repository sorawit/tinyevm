use ethereum_types::U256;

pub trait Database {
    /// Returns the value at the specified key slot.
    fn get(&self, key: U256) -> U256;

    /// Sets the value at the specified key slot.
    fn set(&mut self, key: U256, value: U256);
}

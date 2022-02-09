use crate::types::Error;
use ethereum_types::U256;

const MAX_SIZE: usize = 65536;
const WORD_SIZE: usize = 32;

pub struct Mem(Vec<u8>);

impl Mem {
    /// Creates a fresh new memory for runtime execution.
    pub fn new() -> Self {
        Self(Vec::with_capacity(MAX_SIZE))
    }

    /// Returns the size of the current memory.
    pub fn size(&self) -> usize {
        return self.0.len();
    }

    /// Resizes the memory buffer to allow accessing the given location.
    pub fn resize_for(&mut self, key: usize) -> Result<(), Error> {
        let bound = (((key - 1) / 32) + 1) * 32;
        if bound > self.0.len() {
            self.0.resize(bound, 0);
        }
        Ok(())
    }

    /// Stores the given u8 value to the location at the specified key.
    pub fn mstores(&mut self, key: usize, value: u8) -> Result<(), Error> {
        if key >= MAX_SIZE {
            return Err(Error::MemoryOverflow);
        }
        self.resize_for(key)?;
        Ok(self.0[key] = value)
    }

    /// Stores the given 256 value to the location at the specified key.
    pub fn mstore(&mut self, key: usize, value: U256) -> Result<(), Error> {
        if key >= MAX_SIZE - WORD_SIZE {
            return Err(Error::MemoryOverflow);
        }
        self.resize_for(key + WORD_SIZE)?;
        Ok(value.to_big_endian(&mut self.0[key..key + WORD_SIZE]))
    }

    /// Loads the value from the location at the specified key.
    pub fn mload(&mut self, key: usize) -> Result<U256, Error> {
        if key >= MAX_SIZE - WORD_SIZE {
            return Err(Error::MemoryOverflow);
        }
        self.resize_for(key + WORD_SIZE)?;
        let slice = &self.0[key..key + WORD_SIZE];
        Ok(U256::from_big_endian(slice))
    }

    /// Returns a view only memory slice for the specified area.
    pub fn mview(&self, start: usize, len: usize) -> Result<&[u8], Error> {
        let end = start.checked_add(len).ok_or(Error::MemoryOutOfBound)?;
        if end <= self.0.len() {
            Ok(&self.0[start..end])
        } else {
            Err(Error::MemoryOutOfBound)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_out_of_range() {
        let mut mem = Mem::new();
        assert_eq!(mem.mload(1000000), Err(Error::MemoryOverflow));
        assert_eq!(mem.mstore(1000000, 1.into()), Err(Error::MemoryOverflow));
    }

    #[test]
    fn test_mstore_mload() {
        let mut mem = Mem::new();
        assert_eq!(mem.mload(1000), Ok(0.into()));
        mem.mstore(1000, 10.into()).unwrap();
        assert_eq!(mem.mload(1000), Ok(10.into()));
        assert_eq!(mem.mload(1001), Ok(2560.into()));
        assert_eq!(mem.size(), 1056);
    }
}

use crate::error::Error;
use ethereum_types::{BigEndianHash, H256, U256};

const MAX_SIZE: usize = 1024;

pub struct Stack(Vec<U256>);

impl Stack {
    /// Creates a fresh new stack for runtime execution.
    pub fn new() -> Self {
        Self(Vec::with_capacity(MAX_SIZE))
    }

    /// Pushes a new usize value to the stack.
    pub fn push_usize(&mut self, value: usize) -> Result<(), Error> {
        self.push_u256(value.into())
    }

    /// Pushes a new h256 value to the stack.
    pub fn push_h256(&mut self, value: H256) -> Result<(), Error> {
        self.push_u256(value.into_uint())
    }

    /// Pushes a new u256 value to the stack.
    pub fn push_u256(&mut self, value: U256) -> Result<(), Error> {
        if self.0.len() < MAX_SIZE {
            Ok(self.0.push(value))
        } else {
            Err(Error::StackOverflow)
        }
    }

    /// Pops a value and throws it away.
    pub fn pop(&mut self) -> Result<(), Error> {
        self.pop_usize().map(|_| ())
    }

    /// Pops a value from the stack as a usize.
    pub fn pop_usize(&mut self) -> Result<usize, Error> {
        let value_256 = self.pop_u256()?;
        if value_256 <= usize::max_value().into() {
            Ok(value_256.as_usize())
        } else {
            Err(Error::StackValueOutOfRange)
        }
    }

    /// Pops a value from the stack as a u256.
    pub fn pop_u256(&mut self) -> Result<U256, Error> {
        self.0.pop().ok_or(Error::StackUnderflow)
    }

    /// Duplicates the N^th value of the stack.
    pub fn dup<const N: usize>(&mut self) -> Result<(), Error> {
        // TODO: Asserts N <= 32 at compile time
        if self.0.len() >= N {
            let len = self.0.len();
            self.push_u256(self.0[len - N])
        } else {
            Err(Error::StackUnderflow)
        }
    }

    /// Swaps the first value and the (N+1)^th value of the stack.
    pub fn swap<const N: usize>(&mut self) -> Result<(), Error> {
        // TODO: Asserts N <= 32 at compile time
        if self.0.len() >= N + 1 {
            let len = self.0.len();
            Ok(self.0.swap(len - 1, len - N - 1))
        } else {
            Err(Error::StackUnderflow)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut stk = Stack::new();
        let bv = U256::from_dec_str("9999999999999999999999").unwrap();
        stk.push_u256(456.into()).unwrap();
        stk.push_usize(15).unwrap();
        stk.push_u256(bv).unwrap();
        stk.push_u256(bv).unwrap();
        assert_eq!(stk.pop_u256(), Ok(bv));
        assert_eq!(stk.pop_usize(), Err(Error::StackValueOutOfRange));
        assert_eq!(stk.pop_u256(), Ok(15.into()));
        assert_eq!(stk.pop_usize(), Ok(456));
        assert_eq!(stk.pop(), Err(Error::StackUnderflow));
        stk.push_usize(20).unwrap();
        stk.push_usize(21).unwrap();
        assert_eq!(stk.pop(), Ok(()));
        assert_eq!(stk.pop_usize(), Ok(20));
    }

    #[test]
    fn test_dup() {
        let mut stk = Stack::new();
        stk.push_usize(1).unwrap();
        stk.push_usize(2).unwrap();
        stk.push_usize(3).unwrap();
        stk.push_usize(4).unwrap();
        stk.dup::<1>().unwrap();
        stk.dup::<4>().unwrap();
        assert_eq!(stk.dup::<20>(), Err(Error::StackUnderflow));
        assert_eq!(stk.pop_usize(), Ok(2));
        assert_eq!(stk.pop_usize(), Ok(4));
        assert_eq!(stk.pop_usize(), Ok(4));
        assert_eq!(stk.pop_usize(), Ok(3));
        assert_eq!(stk.pop(), Ok(()));
        assert_eq!(stk.pop_usize(), Ok(1));
    }

    #[test]
    fn test_swap() {
        let mut stk = Stack::new();
        stk.push_usize(1).unwrap();
        stk.push_usize(2).unwrap();
        stk.push_usize(3).unwrap();
        stk.push_usize(4).unwrap();
        stk.swap::<1>().unwrap();
        stk.swap::<3>().unwrap();
        assert_eq!(stk.swap::<4>(), Err(Error::StackUnderflow));
        assert_eq!(stk.pop_usize(), Ok(1));
        assert_eq!(stk.pop_usize(), Ok(4));
        assert_eq!(stk.pop_usize(), Ok(2));
        assert_eq!(stk.pop_usize(), Ok(3));
    }
}

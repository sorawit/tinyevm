use ethereum_types::{Address, H256, U256};

#[derive(Debug)]
pub struct Log {
    pub topics: Vec<H256>,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct Env {
    pub caller: Address,
    pub timestamp: U256,
    pub number: U256,
    pub chainid: U256,
}

#[derive(PartialEq, Debug)]
pub enum Error {
    Revert(Vec<u8>),
    InvalidOpcode(u8),
    CodeOutOfBound,
    StackOverflow,
    StackUnderflow,
    StackValueOutOfRange,
    MemoryOverflow,
    MemoryOutOfBound,
}

#[derive(PartialEq, Debug)]
pub enum OpStep {
    Continue,
    Return(Vec<u8>),
}

pub type OpResult = Result<OpStep, Error>;
pub type RunResult = Result<(Vec<u8>, Vec<Log>), Error>;

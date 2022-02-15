use ethereum_types::{Address, H256, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Log {
    pub topics: Vec<H256>,
    pub data: Vec<u8>,
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Env {
    pub caller: Address,
    pub timestamp: U256,
    pub number: U256,
    pub chainid: U256,
    #[serde_as(as = "serde_with::hex::Hex")]
    pub calldata: Vec<u8>,
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

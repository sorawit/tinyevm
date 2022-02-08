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
pub type RunResult = Result<Vec<u8>, Error>;

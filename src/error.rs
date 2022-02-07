#[derive(PartialEq, Debug)]
pub enum Error {
    InvalidOpcode,
    StackOverflow,
    StackUnderflow,
    StackValueOutOfRange,
    MemoryOverflow,
    MemoryOutOfBound,
}

#[derive(PartialEq, Debug)]
pub enum Error {
    InvalidOpcode(u8),
    StackOverflow,
    StackUnderflow,
    StackValueOutOfRange,
    MemoryOverflow,
    MemoryOutOfBound,
}

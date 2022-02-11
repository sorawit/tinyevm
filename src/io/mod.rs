pub trait IO<'a> {
    /// Returns the EVM code to process.
    fn get_code(&self) -> &'a [u8];
}

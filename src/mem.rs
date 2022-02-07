pub struct Mem(Vec<u8>);

impl Mem {
    pub fn new() -> Self {
        Self(Vec::with_capacity(1024))
    }
}

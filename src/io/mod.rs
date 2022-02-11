use crate::types::Env;
use serde::Deserialize;
use std::path::Path;

pub trait IO {
    /// Returns the EVM code to process.
    fn get_code(&self) -> Vec<u8>;

    /// Returns the next environment to execute in the VM.
    fn get_next_env(&mut self) -> Option<Env>;
}

pub struct FileIO {}

impl FileIO {
    pub fn new(path: &Path) -> Self {
        let file = std::fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        // let data: FileIOData = serde_json::from_reader(reader).unwrap();
        // TODO
        Self {}
    }
}

impl IO for FileIO {
    fn get_code(&self) -> Vec<u8> {
        unimplemented!();
    }

    fn get_next_env(&mut self) -> Option<Env> {
        unimplemented!();
    }
}

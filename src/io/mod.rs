use crate::types::Env;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub trait IO {
    /// Returns the EVM code to process.
    fn get_code(&self) -> Vec<u8>;

    /// Returns the next environment to execute in the VM.
    fn get_next_env(&mut self) -> Option<Env>;
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize)]
pub struct FileIO {
    #[serde_as(as = "serde_with::hex::Hex")]
    code: Vec<u8>,
    envs: Vec<Env>,
}

impl FileIO {
    pub fn new(path: &Path) -> Self {
        let file = std::fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        let mut data: FileIO = serde_json::from_reader(reader).unwrap();
        data.envs = data.envs.into_iter().rev().collect();
        data
    }
}

impl IO for FileIO {
    fn get_code(&self) -> Vec<u8> {
        self.code.to_owned()
    }

    fn get_next_env(&mut self) -> Option<Env> {
        self.envs.pop()
    }
}

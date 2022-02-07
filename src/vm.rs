use ethereum_types::{Address, U256};

use crate::database::Database;
use crate::runtime::Runtime;
use crate::state::State;

pub struct VM<'a, DB> {
    code: &'a [u8],
    state: State<DB>,
}

impl<'a, DB: Database> VM<'a, DB> {
    pub fn new(db: DB, code: &'a [u8]) -> Self {
        Self {
            code,
            state: State::new(db),
        }
    }

    pub fn run(&mut self, caller: Address, data: &[u8]) {
        let mut runtime =
            Runtime::new(self.code, &mut self.state, data, caller);
        runtime.run()
    }
}

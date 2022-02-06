use ethereum_types::Address;

use crate::database::Database;
use crate::state::State;

pub struct VM<'a, DB> {
    state: State<DB>,
    code: &'a [u8],
}

impl<'a, DB: Database> VM<'a, DB> {
    pub fn new(db: DB, code: &'a [u8]) -> Self {
        Self {
            state: State::new(db),
            code,
        }
    }

    pub fn run(&mut self, caller: Address, data: &[u8]) {
        unimplemented!();
    }
}

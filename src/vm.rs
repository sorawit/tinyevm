use crate::database::Database;
use crate::runtime;
use crate::state::State;
use ethereum_types::Address;

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
        println!(
            "{:?}",
            runtime::run(self.code, &mut self.state, data, caller)
        );
    }

    pub fn call(&mut self, caller: Address, data: &[u8]) {
        let _ = runtime::run(self.code, &mut self.state, data, caller);
        self.state.rollback();
    }
}

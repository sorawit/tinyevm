use crate::db::Database;
use crate::runtime;
use crate::state::State;
use crate::types::{Env, RunResult};

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

    /// Runs a transaction and returns the result + updates the state.
    pub fn run(&mut self, env: &Env) -> RunResult {
        let res = runtime::run(self.code, &mut self.state, env);
        match res {
            Ok(_) => self.state.commit(),
            Err(_) => self.state.rollback(),
        }
        res
    }

    /// Runs a transaction and returns the result + discards state changes.
    pub fn call(&mut self, env: &Env) -> RunResult {
        let res = runtime::run(self.code, &mut self.state, env);
        self.state.rollback();
        res
    }
}

use crate::database::Database;
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

    pub fn run(&mut self, env: &Env, data: &[u8]) -> RunResult {
        let res = runtime::run(self.code, &mut self.state, data, env);
        match res {
            Ok(_) => self.state.commit(),
            Err(_) => self.state.rollback(),
        }
        res
    }

    pub fn call(&mut self, env: &Env, data: &[u8]) -> RunResult {
        let res = runtime::run(self.code, &mut self.state, data, env);
        self.state.rollback();
        res
    }
}

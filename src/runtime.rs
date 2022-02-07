use ethereum_types::{Address, U256};

use crate::database::Database;
use crate::error::Error;
use crate::mem::Mem;
use crate::stack::Stack;
use crate::state::State;

pub struct Runtime<'a, 'b, DB> {
    code: &'a [u8],
    state: &'b mut State<DB>,
    data: &'b [u8],
    caller: Address,
    pc: usize,

    mem: Mem,
    stack: Stack,
}

pub enum OpStep {
    Continue,
    Return(Vec<u8>),
    Revert(Vec<u8>),
}

type OpResult = Result<OpStep, Error>;

fn handle_0x00_stop<DB>(_ctx: &mut Runtime<DB>) -> OpResult {
    Ok(OpStep::Revert(Vec::new()))
}

fn handle_0x01_add<DB>(ctx: &mut Runtime<DB>) -> OpResult {
    let lhs = ctx.stack.pop_u256()?;
    let rhs = ctx.stack.pop_u256()?;
    ctx.stack.push_u256(lhs + rhs)?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x02_mul<DB>(ctx: &mut Runtime<DB>) -> OpResult {
    let lhs = ctx.stack.pop_u256()?;
    let rhs = ctx.stack.pop_u256()?;
    ctx.stack.push_u256(lhs * rhs)?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x03_sub<DB>(ctx: &mut Runtime<DB>) -> OpResult {
    let lhs = ctx.stack.pop_u256()?;
    let rhs = ctx.stack.pop_u256()?;
    ctx.stack.push_u256(lhs - rhs)?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x10_lt<DB>(ctx: &mut Runtime<DB>) -> OpResult {
    let lhs = ctx.stack.pop_u256()?;
    let rhs = ctx.stack.pop_u256()?;
    ctx.stack.push_usize(if lhs < rhs { 1 } else { 0 })?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x14_eq<DB>(ctx: &mut Runtime<DB>) -> OpResult {
    let lhs = ctx.stack.pop_u256()?;
    let rhs = ctx.stack.pop_u256()?;
    ctx.stack.push_usize(if lhs == rhs { 1 } else { 0 })?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x15_iszero<DB>(ctx: &mut Runtime<DB>) -> OpResult {
    let value = ctx.stack.pop_u256()?;
    ctx.stack.push_usize(if value.is_zero() { 1 } else { 0 })?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x33_caller<DB>(ctx: &mut Runtime<DB>) -> OpResult {
    ctx.stack.push_h256(ctx.caller.into())?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x34_callvalue<DB>(ctx: &mut Runtime<DB>) -> OpResult {
    ctx.stack.push_usize(0)?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x36_calldatasize<DB>(ctx: &mut Runtime<DB>) -> OpResult {
    ctx.stack.push_usize(ctx.data.len())?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x50_pop<DB>(ctx: &mut Runtime<DB>) -> OpResult {
    ctx.stack.pop()?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x51_mload<DB>(ctx: &mut Runtime<DB>) -> OpResult {
    let loc = ctx.stack.pop_usize()?;
    ctx.stack.push_u256(ctx.mem.mload(loc)?)?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x54_sload<DB: Database>(ctx: &mut Runtime<DB>) -> OpResult {
    let key = ctx.stack.pop_u256()?;
    ctx.stack.push_u256(ctx.state.load(key))?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x55_sstore<DB: Database>(ctx: &mut Runtime<DB>) -> OpResult {
    let key = ctx.stack.pop_u256()?;
    let value = ctx.stack.pop_u256()?;
    ctx.state.store(key, value);
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x56_jump<DB>(ctx: &mut Runtime<DB>) -> OpResult {
    let loc = ctx.stack.pop_usize()?;
    ctx.pc = loc;
    Ok(OpStep::Continue)
}

fn handle_0x60_push<DB, const N: usize>(ctx: &mut Runtime<DB>) -> OpResult {
    let slice = &ctx.code[ctx.pc + 1..ctx.pc + N + 1];
    let value = U256::from_big_endian(slice);
    ctx.stack.push_u256(value)?;
    ctx.pc += N + 1;
    Ok(OpStep::Continue)
}

fn handle_0x80_dup<DB, const N: usize>(ctx: &mut Runtime<DB>) -> OpResult {
    ctx.stack.dup::<N>()?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

impl<'a, 'b, DB: Database> Runtime<'a, 'b, DB> {
    pub fn new(
        code: &'a [u8],
        state: &'b mut State<DB>,
        data: &'b [u8],
        caller: Address,
    ) -> Self {
        return Self {
            code,
            state,
            data,
            caller,
            pc: 0,
            mem: Mem::new(),
            stack: Stack::new(),
        };
    }
    pub fn run(&mut self) {
        loop {
            let opcode = self.code[self.pc];
            match self.next(opcode) {
                Err(_) => panic!("error"),
                Ok(OpStep::Continue) => (),
                Ok(OpStep::Return(v)) => {
                    println!("Return {:?}", v);
                    break;
                }
                Ok(OpStep::Revert(v)) => {
                    println!("Revert {:?}", v);
                    break;
                }
            }
        }
    }

    pub fn next(&mut self, opcode: u8) -> OpResult {
        match opcode {
            0x00 => handle_0x00_stop(self),
            0x01 => handle_0x01_add(self),
            0x02 => handle_0x02_mul(self),
            0x03 => handle_0x03_sub(self),
            0x10 => handle_0x10_lt(self),
            0x14 => handle_0x14_eq(self),
            0x15 => handle_0x15_iszero(self),
            0x33 => handle_0x33_caller(self),
            0x34 => handle_0x34_callvalue(self),
            0x36 => handle_0x36_calldatasize(self),
            0x50 => handle_0x50_pop(self),
            0x51 => handle_0x51_mload(self),
            0x54 => handle_0x54_sload(self),
            0x55 => handle_0x55_sstore(self),
            0x56 => handle_0x56_jump(self),
            0x60 => handle_0x60_push::<_, 1>(self),
            0x61 => handle_0x60_push::<_, 2>(self),
            0x62 => handle_0x60_push::<_, 3>(self),
            0x63 => handle_0x60_push::<_, 4>(self),
            0x64 => handle_0x60_push::<_, 5>(self),
            0x65 => handle_0x60_push::<_, 6>(self),
            0x66 => handle_0x60_push::<_, 7>(self),
            0x67 => handle_0x60_push::<_, 8>(self),
            0x68 => handle_0x60_push::<_, 9>(self),
            0x69 => handle_0x60_push::<_, 10>(self),
            0x6a => handle_0x60_push::<_, 11>(self),
            0x6b => handle_0x60_push::<_, 12>(self),
            0x6c => handle_0x60_push::<_, 13>(self),
            0x6d => handle_0x60_push::<_, 14>(self),
            0x6e => handle_0x60_push::<_, 15>(self),
            0x6f => handle_0x60_push::<_, 16>(self),
            0x70 => handle_0x60_push::<_, 17>(self),
            0x71 => handle_0x60_push::<_, 18>(self),
            0x72 => handle_0x60_push::<_, 19>(self),
            0x73 => handle_0x60_push::<_, 20>(self),
            0x74 => handle_0x60_push::<_, 21>(self),
            0x75 => handle_0x60_push::<_, 22>(self),
            0x76 => handle_0x60_push::<_, 23>(self),
            0x77 => handle_0x60_push::<_, 24>(self),
            0x78 => handle_0x60_push::<_, 25>(self),
            0x79 => handle_0x60_push::<_, 26>(self),
            0x7a => handle_0x60_push::<_, 27>(self),
            0x7b => handle_0x60_push::<_, 28>(self),
            0x7c => handle_0x60_push::<_, 29>(self),
            0x7d => handle_0x60_push::<_, 30>(self),
            0x7e => handle_0x60_push::<_, 31>(self),
            0x7f => handle_0x60_push::<_, 32>(self),
            0x80 => handle_0x80_dup::<_, 1>(self),
            0x81 => handle_0x80_dup::<_, 2>(self),
            0x82 => handle_0x80_dup::<_, 3>(self),
            0x83 => handle_0x80_dup::<_, 4>(self),
            0x84 => handle_0x80_dup::<_, 5>(self),
            0x85 => handle_0x80_dup::<_, 6>(self),
            0x86 => handle_0x80_dup::<_, 7>(self),
            0x87 => handle_0x80_dup::<_, 8>(self),
            0x88 => handle_0x80_dup::<_, 9>(self),
            0x89 => handle_0x80_dup::<_, 10>(self),
            0x8a => handle_0x80_dup::<_, 11>(self),
            0x8b => handle_0x80_dup::<_, 12>(self),
            0x8c => handle_0x80_dup::<_, 13>(self),
            0x8d => handle_0x80_dup::<_, 14>(self),
            0x8e => handle_0x80_dup::<_, 15>(self),
            0x8f => handle_0x80_dup::<_, 16>(self),

            0x35 => {
                // CALLDATALOAD
                let loc = self.stack.pop_usize()?;
                // TODO: Make it better
                let mut rawdata = [0u8; 32];
                for idx in 0..32 {
                    if loc + idx < self.data.len() {
                        rawdata[idx] = self.data[loc + idx];
                    }
                }
                self.stack.push_u256(U256::from_big_endian(&rawdata))?;
                self.pc += 1;
                Ok(OpStep::Continue)
            }
            0x57 => {
                // JUMPI
                let loc = self.stack.pop_usize()?;
                let cond = self.stack.pop_u256()?;
                if cond.is_zero() {
                    self.pc += 1;
                } else {
                    self.pc = loc;
                }
                Ok(OpStep::Continue)
            }
            0x5B => {
                // JUMPDEST
                self.pc += 1;
                Ok(OpStep::Continue)
            }
            0x52 => {
                // MSTORE
                let key = self.stack.pop_usize()?;
                let value = self.stack.pop_u256()?;
                self.mem.mstore(key, value)?;
                self.pc += 1;
                Ok(OpStep::Continue)
            }
            0x90 => {
                // SWAP1
                self.stack.swap::<1>()?;
                self.pc += 1;
                Ok(OpStep::Continue)
            }
            0x91 => {
                // SWAP2
                self.stack.swap::<2>()?;
                self.pc += 1;
                Ok(OpStep::Continue)
            }
            0x1B => {
                // SHL
                let shift = self.stack.pop_u256()?;
                let value = self.stack.pop_u256()?;
                if shift >= 256.into() {
                    self.stack.push_usize(0)?;
                } else {
                    self.stack.push_u256(value << shift.as_u64())?;
                }
                self.pc += 1;
                Ok(OpStep::Continue)
            }
            0x1C => {
                // SHR
                let shift = self.stack.pop_u256()?;
                let value = self.stack.pop_u256()?;
                if shift >= 256.into() {
                    self.stack.push_usize(0)?;
                } else {
                    self.stack.push_u256(value >> shift.as_u64())?;
                }
                self.pc += 1;
                Ok(OpStep::Continue)
            }
            0xF3 => {
                // RETURN
                let start = self.stack.pop_usize()?;
                let len = self.stack.pop_usize()?;
                Ok(OpStep::Return(self.mem.mview(start, len)?.to_vec()))
            }
            0xFD => {
                // REVERT
                let start = self.stack.pop_usize()?;
                let len = self.stack.pop_usize()?;
                Ok(OpStep::Revert(self.mem.mview(start, len)?.to_vec()))
            }
            _ => panic!("unknown opcode 0x{:x}", opcode),
        }
    }
}

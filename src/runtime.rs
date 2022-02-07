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

pub enum OpResult {
    Continue,
    Return,
    Revert,
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
                Ok(OpResult::Continue) => (),
                Ok(OpResult::Return) => break,
                Ok(OpResult::Revert) => break,
            }
        }
    }

    pub fn next(&mut self, opcode: u8) -> Result<OpResult, Error> {
        match opcode {
            0x01 => {
                // ADD
                let lhs = self.stack.pop_u256()?;
                let rhs = self.stack.pop_u256()?;
                self.stack.push_u256(lhs + rhs)?;
                self.pc += 1;
                Ok(OpResult::Continue)
            }
            0x03 => {
                // SUB
                let lhs = self.stack.pop_u256()?;
                let rhs = self.stack.pop_u256()?;
                self.stack.push_u256(lhs - rhs)?;
                self.pc += 1;
                Ok(OpResult::Continue)
            }
            0x10 => {
                // LT
                let lhs = self.stack.pop_u256()?;
                let rhs = self.stack.pop_u256()?;
                self.stack.push_usize(if lhs < rhs { 1 } else { 0 });
                self.pc += 1;
                Ok(OpResult::Continue)
            }
            0x14 => {
                // EQ
                let lhs = self.stack.pop_u256()?;
                let rhs = self.stack.pop_u256()?;
                self.stack.push_usize(if lhs == rhs { 1 } else { 0 });
                self.pc += 1;
                Ok(OpResult::Continue)
            }
            0x15 => {
                // ISZERO
                let value = self.stack.pop_u256()?;
                self.stack.push_usize(if value.is_zero() { 1 } else { 0 })?;
                self.pc += 1;
                Ok(OpResult::Continue)
            }
            0x34 => {
                // CALLVALUE
                self.stack.push_usize(0);
                self.pc += 1;
                Ok(OpResult::Continue)
            }
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
                Ok(OpResult::Continue)
            }
            0x36 => {
                // CALLDATASIZE
                self.stack.push_usize(self.data.len());
                self.pc += 1;
                Ok(OpResult::Continue)
            }
            0x50 => {
                // POP
                self.stack.pop()?;
                self.pc += 1;
                Ok(OpResult::Continue)
            }
            0x51 => {
                // MLOAD
                let loc = self.stack.pop_usize()?;
                self.stack.push_u256(self.mem.mload(loc)?)?;
                self.pc += 1;
                Ok(OpResult::Continue)
            }
            0x56 => {
                // JUMP
                let loc = self.stack.pop_usize()?;
                self.pc = loc;
                Ok(OpResult::Continue)
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
                Ok(OpResult::Continue)
            }
            0x5B => {
                // JUMPDEST
                self.pc += 1;
                Ok(OpResult::Continue)
            }
            0x60 => {
                // PUSH1
                let slice = &self.code[self.pc + 1..self.pc + 2];
                let value = U256::from_big_endian(slice);
                self.stack.push_u256(value)?;
                self.pc += 2;
                Ok(OpResult::Continue)
            }
            0x62 => {
                // PUSH3
                let slice = &self.code[self.pc + 1..self.pc + 4];
                let value = U256::from_big_endian(slice);
                self.stack.push_u256(value)?;
                self.pc += 4;
                Ok(OpResult::Continue)
            }
            0x63 => {
                // PUSH4
                let slice = &self.code[self.pc + 1..self.pc + 5];
                let value = U256::from_big_endian(slice);
                self.stack.push_u256(value)?;
                self.pc += 5;
                Ok(OpResult::Continue)
            }
            0x52 => {
                // MSTORE
                let key = self.stack.pop_usize()?;
                let value = self.stack.pop_u256()?;
                self.mem.mstore(key, value)?;
                self.pc += 1;
                Ok(OpResult::Continue)
            }
            0x80 => {
                // DUP1
                self.stack.dup::<1>()?;
                self.pc += 1;
                Ok(OpResult::Continue)
            }
            0x81 => {
                // DUP2
                self.stack.dup::<2>()?;
                self.pc += 1;
                Ok(OpResult::Continue)
            }
            0x82 => {
                // DUP3
                self.stack.dup::<3>()?;
                self.pc += 1;
                Ok(OpResult::Continue)
            }
            0x90 => {
                // SWAP1
                self.stack.swap::<1>()?;
                self.pc += 1;
                Ok(OpResult::Continue)
            }
            0x91 => {
                // SWAP2
                self.stack.swap::<2>()?;
                self.pc += 1;
                Ok(OpResult::Continue)
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
                Ok(OpResult::Continue)
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
                Ok(OpResult::Continue)
            }
            0xF3 => {
                // RETURN
                let start = self.stack.pop_usize()?;
                let len = self.stack.pop_usize()?;
                println!("RETURN {:?}", &self.mem.mview(start, len)?);
                Ok(OpResult::Return)
            }
            0xFD => {
                // REVERT
                let start = self.stack.pop_usize()?;
                let len = self.stack.pop_usize()?;
                println!("REVERT {:?}", &self.mem.mview(start, len)?);
                Ok(OpResult::Revert)
            }
            _ => panic!("unknown opcode 0x{:x}", opcode),
        }
    }
}

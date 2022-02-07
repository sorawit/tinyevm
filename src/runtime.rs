use ethereum_types::{Address, U256};

use crate::database::Database;
use crate::state::State;

pub struct Runtime<'a, 'b, DB> {
    code: &'a [u8],
    state: &'b mut State<DB>,
    data: &'b [u8],
    caller: Address,
    pc: usize,
    stk: Vec<U256>,
    mem: Vec<u8>,
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
            stk: Vec::with_capacity(1024),
            mem: Vec::with_capacity(1024),
        };
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.code[self.pc];
            match opcode {
                0x01 => {
                    // ADD
                    let lhs = self.stk.pop().unwrap();
                    let rhs = self.stk.pop().unwrap();
                    self.stk.push(lhs + rhs);
                    self.pc += 1;
                }
                0x03 => {
                    // ADD
                    let lhs = self.stk.pop().unwrap();
                    let rhs = self.stk.pop().unwrap();
                    self.stk.push(lhs - rhs);
                    self.pc += 1
                }
                0x10 => {
                    // LT
                    let lhs = self.stk.pop().unwrap();
                    let rhs = self.stk.pop().unwrap();
                    let res = if lhs < rhs { 1 } else { 0 };
                    self.stk.push(res.into());
                    self.pc += 1
                }
                0x14 => {
                    // EQ
                    let lhs = self.stk.pop().unwrap();
                    let rhs = self.stk.pop().unwrap();
                    let res = if lhs == rhs { 1 } else { 0 };
                    self.stk.push(res.into());
                    self.pc += 1
                }
                0x15 => {
                    // ISZERO
                    let value = self.stk.pop().unwrap();
                    let res = if value.is_zero() { 1 } else { 0 };
                    self.stk.push(res.into());
                    self.pc += 1
                }
                0x34 => {
                    // CALLVALUE
                    self.stk.push(0.into());
                    self.pc += 1
                }
                0x35 => {
                    // CALLDATALOAD
                    let loc = self.stk.pop().unwrap().as_usize();
                    // TODO: Make it better
                    let mut rawdata = [0u8; 32];
                    for idx in 0..32 {
                        if loc + idx < self.data.len() {
                            rawdata[idx] = self.data[loc + idx];
                        }
                    }
                    self.stk.push(U256::from_big_endian(&rawdata));
                    self.pc += 1
                }
                0x36 => {
                    // CALLDATASIZE
                    self.stk.push(self.data.len().into());
                    self.pc += 1
                }
                0x50 => {
                    // POP
                    self.stk.pop();
                    self.pc += 1
                }
                0x51 => {
                    // MLOAD
                    let loc = self.stk.pop().unwrap().as_usize();
                    let slice = &self.mem[loc..loc + 32];
                    self.stk.push(U256::from_big_endian(slice));
                    self.pc += 1
                }
                0x56 => {
                    // JUMP
                    let loc = self.stk.pop().unwrap().as_usize();
                    self.pc = loc
                }
                0x57 => {
                    // JUMPI
                    let loc = self.stk.pop().unwrap().as_usize();
                    let cond = self.stk.pop().unwrap();
                    if cond.is_zero() {
                        self.pc += 1
                    } else {
                        self.pc = loc
                    }
                }
                0x5B => {
                    // JUMPDEST
                    self.pc += 1
                }
                0x60 => {
                    // PUSH1
                    let slice = &self.code[self.pc + 1..self.pc + 2];
                    let value = U256::from_big_endian(slice);
                    self.stk.push(value);
                    self.pc += 2
                }
                0x62 => {
                    // PUSH3
                    let slice = &self.code[self.pc + 1..self.pc + 4];
                    let value = U256::from_big_endian(slice);
                    self.stk.push(value);
                    self.pc += 4
                }
                0x63 => {
                    // PUSH4
                    let slice = &self.code[self.pc + 1..self.pc + 5];
                    let value = U256::from_big_endian(slice);
                    self.stk.push(value);
                    self.pc += 5
                }
                0x52 => {
                    // MSTORE
                    let key = self.stk.pop().unwrap().as_usize();
                    let value = self.stk.pop().unwrap();
                    self.mem.resize(1024, 0); // TODO: Make it proper
                    value.to_big_endian(&mut self.mem[key..key + 32]);
                    println!("{:x}, {:?}", self.pc, value);
                    self.pc += 1
                }
                0x80 => {
                    // DUP1
                    self.stk.push(self.stk[self.stk.len() - 1]);
                    self.pc += 1
                }
                0x81 => {
                    // DUP2
                    self.stk.push(self.stk[self.stk.len() - 2]);
                    self.pc += 1
                }
                0x82 => {
                    // DUP3
                    self.stk.push(self.stk[self.stk.len() - 3]);
                    self.pc += 1
                }
                0x90 => {
                    // SWAP1
                    let len = self.stk.len();
                    self.stk.swap(len - 1, len - 2);
                    self.pc += 1
                }
                0x91 => {
                    // SWAP1
                    let len = self.stk.len();
                    self.stk.swap(len - 1, len - 3);
                    self.pc += 1
                }
                0x1B => {
                    // SHL
                    let shift = self.stk.pop().unwrap();
                    let value = self.stk.pop().unwrap();
                    if shift >= 256.into() {
                        self.stk.push(0.into());
                    } else {
                        self.stk.push(value << shift.as_u64());
                    }
                    self.pc += 1
                }
                0x1C => {
                    // SHR
                    let shift = self.stk.pop().unwrap();
                    let value = self.stk.pop().unwrap();
                    if shift >= 256.into() {
                        self.stk.push(0.into());
                    } else {
                        self.stk.push(value >> shift.as_u64());
                    }
                    self.pc += 1
                }
                0xF3 => {
                    // RETURN
                    let staself = self.stk.pop().unwrap().as_usize();
                    let end = staself + self.stk.pop().unwrap().as_usize();
                    println!("RETURN {:?}", &self.mem[staself..end]);
                    break;
                }
                0xFD => {
                    // REVERT
                    let staself = self.stk.pop().unwrap().as_usize();
                    let end = staself + self.stk.pop().unwrap().as_usize();
                    println!("REVERT {:?}", &self.mem[staself..end]);
                    break;
                }
                _ => panic!("unknown opcode 0x{:x}", opcode),
            };
        }
    }
}

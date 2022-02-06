use ethereum_types::{Address, U256};

use crate::database::Database;
use crate::state::State;

pub struct VM<'a, DB> {
    state: State<DB>,
    code: &'a [u8],
}

struct Runtime {
    pc: usize,
    mem: Vec<u8>,
    stk: Vec<U256>,
}

impl<'a, DB: Database> VM<'a, DB> {
    pub fn new(db: DB, code: &'a [u8]) -> Self {
        Self {
            state: State::new(db),
            code,
        }
    }

    pub fn run(&mut self, caller: Address, data: &[u8]) {
        let mut rt = Runtime {
            pc: 0,
            mem: Vec::with_capacity(1024),
            stk: Vec::with_capacity(1024),
        };
        loop {
            let opcode = self.code[rt.pc];
            // println!("opcode {} 0x{:x}", rt.pc, opcode);
            match opcode {
                0x01 => {
                    // ADD
                    let lhs = rt.stk.pop().unwrap();
                    let rhs = rt.stk.pop().unwrap();
                    rt.stk.push(lhs + rhs);
                    rt.pc += 1
                }
                0x03 => {
                    // ADD
                    let lhs = rt.stk.pop().unwrap();
                    let rhs = rt.stk.pop().unwrap();
                    rt.stk.push(lhs - rhs);
                    rt.pc += 1
                }
                0x10 => {
                    // LT
                    let lhs = rt.stk.pop().unwrap();
                    let rhs = rt.stk.pop().unwrap();
                    let res = if lhs < rhs { 1 } else { 0 };
                    rt.stk.push(res.into());
                    rt.pc += 1
                }
                0x14 => {
                    // EQ
                    let lhs = rt.stk.pop().unwrap();
                    let rhs = rt.stk.pop().unwrap();
                    let res = if lhs == rhs { 1 } else { 0 };
                    rt.stk.push(res.into());
                    rt.pc += 1
                }
                0x15 => {
                    // ISZERO
                    let value = rt.stk.pop().unwrap();
                    let res = if value.is_zero() { 1 } else { 0 };
                    rt.stk.push(res.into());
                    rt.pc += 1
                }
                0x34 => {
                    // CALLVALUE
                    rt.stk.push(0.into());
                    rt.pc += 1
                }
                0x35 => {
                    // CALLDATALOAD
                    let loc = rt.stk.pop().unwrap().as_usize();
                    // TODO: Make it better
                    let mut rawdata = [0u8; 32];
                    for idx in 0..32 {
                        if loc + idx < data.len() {
                            rawdata[idx] = data[loc + idx];
                        }
                    }
                    rt.stk.push(U256::from_big_endian(&rawdata));
                    rt.pc += 1
                }
                0x36 => {
                    // CALLDATASIZE
                    rt.stk.push(data.len().into());
                    rt.pc += 1
                }
                0x50 => {
                    // POP
                    rt.stk.pop();
                    rt.pc += 1
                }
                0x51 => {
                    // MLOAD
                    let loc = rt.stk.pop().unwrap().as_usize();
                    let slice = &rt.mem[loc..loc + 32];
                    rt.stk.push(U256::from_big_endian(slice));
                    rt.pc += 1
                }
                0x57 => {
                    // JUMPI
                    let loc = rt.stk.pop().unwrap().as_usize();
                    let cond = rt.stk.pop().unwrap();
                    if cond.is_zero() {
                        rt.pc += 1
                    } else {
                        rt.pc = loc
                    }
                }
                0x5B => {
                    // JUMPDEST
                    rt.pc += 1
                }
                0x60 => {
                    // PUSH1
                    let slice = &self.code[rt.pc + 1..rt.pc + 2];
                    let value = U256::from_big_endian(slice);
                    rt.stk.push(value);
                    rt.pc += 2
                }
                0x63 => {
                    // PUSH4
                    let slice = &self.code[rt.pc + 1..rt.pc + 5];
                    let value = U256::from_big_endian(slice);
                    rt.stk.push(value);
                    rt.pc += 5
                }
                0x52 => {
                    // MSTORE
                    let key = rt.stk.pop().unwrap().as_usize();
                    let value = rt.stk.pop().unwrap();
                    rt.mem.resize(1024, 0); // TODO: Make it proper
                    value.to_big_endian(&mut rt.mem[key..key + 32]);
                    rt.pc += 1
                }
                0x80 => {
                    // DUP1
                    rt.stk.push(rt.stk[rt.stk.len() - 1]);
                    rt.pc += 1
                }
                0x81 => {
                    // DUP2
                    rt.stk.push(rt.stk[rt.stk.len() - 2]);
                    rt.pc += 1
                }
                0x90 => {
                    // SWAP1
                    let len = rt.stk.len();
                    rt.stk.swap(len - 1, len - 2);
                    rt.pc += 1
                }
                0x91 => {
                    // SWAP1
                    let len = rt.stk.len();
                    rt.stk.swap(len - 1, len - 3);
                    rt.pc += 1
                }
                0x1C => {
                    // SHR
                    let shift = rt.stk.pop().unwrap();
                    let value = rt.stk.pop().unwrap();
                    if shift >= 256.into() {
                        rt.stk.push(0.into());
                    } else {
                        rt.stk.push(value >> shift.as_u64());
                    }
                    rt.pc += 2
                }
                0xF3 => {
                    // RETURN
                    let start = rt.stk.pop().unwrap().as_usize();
                    let end = start + rt.stk.pop().unwrap().as_usize();
                    println!("RETURN {:?}", &rt.mem[start..end]);
                    break;
                }
                0xFD => {
                    // REVERT
                    let start = rt.stk.pop().unwrap().as_usize();
                    let end = start + rt.stk.pop().unwrap().as_usize();
                    println!("REVERT {:?}", &rt.mem[start..end]);
                    break;
                }
                _ => panic!("unknown opcode 0x{:x}", opcode),
            }
        }
    }
}

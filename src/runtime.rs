use ethereum_types::{Address, H256, U256};
use sha3::{Digest, Keccak256};

use crate::database::Database;
use crate::mem::Mem;
use crate::stack::Stack;
use crate::state::State;
use crate::types::{Error, OpResult, OpStep, RunResult};

struct Context<'a, 'b, DB> {
    code: &'a [u8],
    state: &'b mut State<DB>,
    data: &'b [u8],
    caller: Address,
    pc: usize,
    mem: Mem,
    stack: Stack,
}

fn handle_0x00_stop<DB>(_ctx: &mut Context<DB>) -> OpResult {
    Ok(OpStep::Return(Vec::new()))
}

fn handle_0x01_add<DB>(ctx: &mut Context<DB>) -> OpResult {
    let lhs = ctx.stack.pop_u256()?;
    let rhs = ctx.stack.pop_u256()?;
    ctx.stack.push_u256(lhs + rhs)?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x02_mul<DB>(ctx: &mut Context<DB>) -> OpResult {
    let lhs = ctx.stack.pop_u256()?;
    let rhs = ctx.stack.pop_u256()?;
    ctx.stack.push_u256(lhs * rhs)?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x03_sub<DB>(ctx: &mut Context<DB>) -> OpResult {
    let lhs = ctx.stack.pop_u256()?;
    let rhs = ctx.stack.pop_u256()?;
    ctx.stack.push_u256(lhs - rhs)?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x10_lt<DB>(ctx: &mut Context<DB>) -> OpResult {
    let lhs = ctx.stack.pop_u256()?;
    let rhs = ctx.stack.pop_u256()?;
    ctx.stack.push_usize(if lhs < rhs { 1 } else { 0 })?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x14_eq<DB>(ctx: &mut Context<DB>) -> OpResult {
    let lhs = ctx.stack.pop_u256()?;
    let rhs = ctx.stack.pop_u256()?;
    ctx.stack.push_usize(if lhs == rhs { 1 } else { 0 })?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x15_iszero<DB>(ctx: &mut Context<DB>) -> OpResult {
    let value = ctx.stack.pop_u256()?;
    ctx.stack.push_usize(if value.is_zero() { 1 } else { 0 })?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x19_not<DB>(ctx: &mut Context<DB>) -> OpResult {
    let value = ctx.stack.pop_u256()?;
    ctx.stack.push_u256(!value)?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x1b_shl<DB>(ctx: &mut Context<DB>) -> OpResult {
    let shift = ctx.stack.pop_u256()?;
    let value = ctx.stack.pop_u256()?;
    ctx.stack.push_u256(value << shift)?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x1c_shr<DB>(ctx: &mut Context<DB>) -> OpResult {
    let shift = ctx.stack.pop_u256()?;
    let value = ctx.stack.pop_u256()?;
    ctx.stack.push_u256(value >> shift)?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x20_keccak256<DB>(ctx: &mut Context<DB>) -> OpResult {
    let start = ctx.stack.pop_usize()?;
    let len = ctx.stack.pop_usize()?;
    let res = Keccak256::digest(ctx.mem.mview(start, len)?);
    ctx.stack.push_h256(H256::from_slice(&res))?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x33_caller<DB>(ctx: &mut Context<DB>) -> OpResult {
    ctx.stack.push_h256(ctx.caller.into())?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x34_callvalue<DB>(ctx: &mut Context<DB>) -> OpResult {
    ctx.stack.push_usize(0)?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x35_calldataload<DB>(ctx: &mut Context<DB>) -> OpResult {
    let loc = ctx.stack.pop_usize()?;
    // TODO: Make it better
    let mut rawdata = [0u8; 32];
    for idx in 0..32 {
        if loc + idx < ctx.data.len() {
            rawdata[idx] = ctx.data[loc + idx];
        }
    }
    ctx.stack.push_u256(U256::from_big_endian(&rawdata))?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x36_calldatasize<DB>(ctx: &mut Context<DB>) -> OpResult {
    ctx.stack.push_usize(ctx.data.len())?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x50_pop<DB>(ctx: &mut Context<DB>) -> OpResult {
    ctx.stack.pop()?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x51_mload<DB>(ctx: &mut Context<DB>) -> OpResult {
    let key = ctx.stack.pop_usize()?;
    ctx.stack.push_u256(ctx.mem.mload(key)?)?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x52_mstore<DB>(ctx: &mut Context<DB>) -> OpResult {
    let key = ctx.stack.pop_usize()?;
    let value = ctx.stack.pop_u256()?;
    ctx.mem.mstore(key, value)?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x54_sload<DB: Database>(ctx: &mut Context<DB>) -> OpResult {
    let key = ctx.stack.pop_u256()?;
    ctx.stack.push_u256(ctx.state.load(key))?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x55_sstore<DB: Database>(ctx: &mut Context<DB>) -> OpResult {
    let key = ctx.stack.pop_u256()?;
    let value = ctx.stack.pop_u256()?;
    ctx.state.store(key, value);
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x56_jump<DB>(ctx: &mut Context<DB>) -> OpResult {
    let loc = ctx.stack.pop_usize()?;
    ctx.pc = loc;
    Ok(OpStep::Continue)
}

fn handle_0x57_jumpi<DB>(ctx: &mut Context<DB>) -> OpResult {
    let loc = ctx.stack.pop_usize()?;
    let cond = ctx.stack.pop_u256()?;
    ctx.pc = if cond.is_zero() { ctx.pc + 1 } else { loc };
    Ok(OpStep::Continue)
}

fn handle_0x5b_jumpdest<DB>(ctx: &mut Context<DB>) -> OpResult {
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x60_push<DB, const N: usize>(ctx: &mut Context<DB>) -> OpResult {
    if N < ctx.code.len() - ctx.pc {
        let slice = &ctx.code[ctx.pc + 1..ctx.pc + N + 1];
        let value = U256::from_big_endian(slice);
        ctx.stack.push_u256(value)?;
        ctx.pc += N + 1;
        Ok(OpStep::Continue)
    } else {
        Err(Error::CodeOutOfBound)
    }
}

fn handle_0x80_dup<DB, const N: usize>(ctx: &mut Context<DB>) -> OpResult {
    ctx.stack.dup::<N>()?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0x90_swap<DB, const N: usize>(ctx: &mut Context<DB>) -> OpResult {
    ctx.stack.swap::<N>()?;
    ctx.pc += 1;
    Ok(OpStep::Continue)
}

fn handle_0xf3_return<DB>(ctx: &mut Context<DB>) -> OpResult {
    let start = ctx.stack.pop_usize()?;
    let len = ctx.stack.pop_usize()?;
    Ok(OpStep::Return(ctx.mem.mview(start, len)?.to_vec()))
}

fn handle_0xfd_revert<DB>(ctx: &mut Context<DB>) -> OpResult {
    let start = ctx.stack.pop_usize()?;
    let len = ctx.stack.pop_usize()?;
    Err(Error::Revert(ctx.mem.mview(start, len)?.to_vec()))
}

fn next<DB: Database>(ctx: &mut Context<DB>) -> OpResult {
    match ctx.code[ctx.pc] {
        0x00 => handle_0x00_stop(ctx),
        0x01 => handle_0x01_add(ctx),
        0x02 => handle_0x02_mul(ctx),
        0x03 => handle_0x03_sub(ctx),
        0x10 => handle_0x10_lt(ctx),
        0x14 => handle_0x14_eq(ctx),
        0x15 => handle_0x15_iszero(ctx),
        0x19 => handle_0x19_not(ctx),
        0x1b => handle_0x1b_shl(ctx),
        0x1c => handle_0x1c_shr(ctx),
        0x20 => handle_0x20_keccak256(ctx),
        0x33 => handle_0x33_caller(ctx),
        0x34 => handle_0x34_callvalue(ctx),
        0x35 => handle_0x35_calldataload(ctx),
        0x36 => handle_0x36_calldatasize(ctx),
        0x50 => handle_0x50_pop(ctx),
        0x51 => handle_0x51_mload(ctx),
        0x52 => handle_0x52_mstore(ctx),
        0x54 => handle_0x54_sload(ctx),
        0x55 => handle_0x55_sstore(ctx),
        0x56 => handle_0x56_jump(ctx),
        0x57 => handle_0x57_jumpi(ctx),
        0x5b => handle_0x5b_jumpdest(ctx),
        0x60 => handle_0x60_push::<_, 1>(ctx),
        0x61 => handle_0x60_push::<_, 2>(ctx),
        0x62 => handle_0x60_push::<_, 3>(ctx),
        0x63 => handle_0x60_push::<_, 4>(ctx),
        0x64 => handle_0x60_push::<_, 5>(ctx),
        0x65 => handle_0x60_push::<_, 6>(ctx),
        0x66 => handle_0x60_push::<_, 7>(ctx),
        0x67 => handle_0x60_push::<_, 8>(ctx),
        0x68 => handle_0x60_push::<_, 9>(ctx),
        0x69 => handle_0x60_push::<_, 10>(ctx),
        0x6a => handle_0x60_push::<_, 11>(ctx),
        0x6b => handle_0x60_push::<_, 12>(ctx),
        0x6c => handle_0x60_push::<_, 13>(ctx),
        0x6d => handle_0x60_push::<_, 14>(ctx),
        0x6e => handle_0x60_push::<_, 15>(ctx),
        0x6f => handle_0x60_push::<_, 16>(ctx),
        0x70 => handle_0x60_push::<_, 17>(ctx),
        0x71 => handle_0x60_push::<_, 18>(ctx),
        0x72 => handle_0x60_push::<_, 19>(ctx),
        0x73 => handle_0x60_push::<_, 20>(ctx),
        0x74 => handle_0x60_push::<_, 21>(ctx),
        0x75 => handle_0x60_push::<_, 22>(ctx),
        0x76 => handle_0x60_push::<_, 23>(ctx),
        0x77 => handle_0x60_push::<_, 24>(ctx),
        0x78 => handle_0x60_push::<_, 25>(ctx),
        0x79 => handle_0x60_push::<_, 26>(ctx),
        0x7a => handle_0x60_push::<_, 27>(ctx),
        0x7b => handle_0x60_push::<_, 28>(ctx),
        0x7c => handle_0x60_push::<_, 29>(ctx),
        0x7d => handle_0x60_push::<_, 30>(ctx),
        0x7e => handle_0x60_push::<_, 31>(ctx),
        0x7f => handle_0x60_push::<_, 32>(ctx),
        0x80 => handle_0x80_dup::<_, 1>(ctx),
        0x81 => handle_0x80_dup::<_, 2>(ctx),
        0x82 => handle_0x80_dup::<_, 3>(ctx),
        0x83 => handle_0x80_dup::<_, 4>(ctx),
        0x84 => handle_0x80_dup::<_, 5>(ctx),
        0x85 => handle_0x80_dup::<_, 6>(ctx),
        0x86 => handle_0x80_dup::<_, 7>(ctx),
        0x87 => handle_0x80_dup::<_, 8>(ctx),
        0x88 => handle_0x80_dup::<_, 9>(ctx),
        0x89 => handle_0x80_dup::<_, 10>(ctx),
        0x8a => handle_0x80_dup::<_, 11>(ctx),
        0x8b => handle_0x80_dup::<_, 12>(ctx),
        0x8c => handle_0x80_dup::<_, 13>(ctx),
        0x8d => handle_0x80_dup::<_, 14>(ctx),
        0x8e => handle_0x80_dup::<_, 15>(ctx),
        0x8f => handle_0x80_dup::<_, 16>(ctx),
        0x90 => handle_0x90_swap::<_, 1>(ctx),
        0x91 => handle_0x90_swap::<_, 2>(ctx),
        0x92 => handle_0x90_swap::<_, 3>(ctx),
        0x93 => handle_0x90_swap::<_, 4>(ctx),
        0x94 => handle_0x90_swap::<_, 5>(ctx),
        0x95 => handle_0x90_swap::<_, 6>(ctx),
        0x96 => handle_0x90_swap::<_, 7>(ctx),
        0x97 => handle_0x90_swap::<_, 8>(ctx),
        0x98 => handle_0x90_swap::<_, 9>(ctx),
        0x99 => handle_0x90_swap::<_, 10>(ctx),
        0x9a => handle_0x90_swap::<_, 11>(ctx),
        0x9b => handle_0x90_swap::<_, 12>(ctx),
        0x9c => handle_0x90_swap::<_, 13>(ctx),
        0x9d => handle_0x90_swap::<_, 14>(ctx),
        0x9e => handle_0x90_swap::<_, 15>(ctx),
        0x9f => handle_0x90_swap::<_, 16>(ctx),
        0xf3 => handle_0xf3_return(ctx),
        0xfd => handle_0xfd_revert(ctx),
        opcode => Err(Error::InvalidOpcode(opcode)),
    }
}

pub fn run<'a, 'b, DB: Database>(
    code: &'a [u8],
    state: &'b mut State<DB>,
    data: &'b [u8],
    caller: Address,
) -> RunResult {
    let mut ctx = Context {
        code,
        state,
        data,
        caller,
        pc: 0,
        mem: Mem::new(),
        stack: Stack::new(),
    };
    loop {
        if ctx.pc >= ctx.code.len() {
            return Err(Error::CodeOutOfBound);
        }
        match next(&mut ctx) {
            Err(err) => return Err(err),
            Ok(OpStep::Continue) => (),
            Ok(OpStep::Return(v)) => return Ok(v),
        }
    }
}

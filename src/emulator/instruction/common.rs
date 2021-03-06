//
// Copyright 2017 hassel_emu Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use emulator::memory::MemoryMap;
use emulator::opcode::{CpuAddressMode, OpAddressMode};
use emulator::registers::Registers;
use emulator::instruction::executor::InstructionResult;
use emulator::instruction::executor::Write;

#[cfg(test)]
use emulator::opcode::OpParam;
#[cfg(test)]
use emulator::instruction::executor::InstructionFn;

const STACK_ADDR: u16 = 0x0100;

#[doc(hidden)]
#[macro_export]
macro_rules! impl_instruction {
    ($const_name:ident => $name:ident [$mode:ident, $params:ident, $reg:ident, $memory:ident, $result:ident] $block:block) => {
        pub const $const_name: InstructionFn = &$name;
        #[allow(unused_mut)]
        fn $name(
                $mode: OpAddressMode,
                $params: &OpParam,
                $reg: &Registers,
                $memory: &mut ::emulator::memory::MemoryMap,
                mut $result: InstructionResult)
        -> InstructionResult {
            $block
            $result
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! test_instruction {
    ($test_name:ident, $func:ident, [$reg:ident, $memory:ident] $block:block) => {
        #[test]
        #[allow(unused_imports)]
        fn $test_name() {
            use super::$func;

            let $reg = &mut ::emulator::registers::Registers::new();
            let $memory = &mut ::emulator::memory::MemoryMap::builder()
                .ram(0x0000, 0xFFFF)
                .build();

            $block
        }
    }
}

#[cfg(test)]
pub fn new_result() -> InstructionResult {
    InstructionResult::new()
}

#[cfg(test)]
pub fn execute(
    func: InstructionFn,
    mode: OpAddressMode,
    param: &OpParam,
    reg: &Registers,
    memory: &mut MemoryMap,
    mut result: InstructionResult,
) -> InstructionResult {
    result.writes.clear();
    result.reg = *reg;

    func(mode, param, reg, memory, result)
}

#[inline]
pub fn push(mut result: InstructionResult, val: u8) -> InstructionResult {
    result
        .writes
        .push(Write::new(STACK_ADDR + result.reg.sp as u16, val));
    result.reg.sp = result.reg.sp.wrapping_sub(1);
    result
}

#[inline]
pub fn pop(result: &mut InstructionResult, memory: &mut MemoryMap) -> u8 {
    result.reg.sp = result.reg.sp.wrapping_add(1);
    memory.read().byte(STACK_ADDR + result.reg.sp as u16)
}

// TODO: unit test
#[inline]
pub fn compare(registers: &mut Registers, value: u8, against: u8) {
    let cmp = value.wrapping_sub(against);
    registers.status.set_nz_from(cmp);
    registers.status.set_carry(value >= against);
}

// TODO: unit test
#[inline]
pub fn branch(cond: bool, reg: &Registers, offset: u8, mut result: InstructionResult) -> InstructionResult {
    if cond {
        let new_pc = (((reg.pc as u32) as i32) + ((offset as i8) as i32)) as u16;
        // Add 2 if page boundary, 1 otherwise
        result.cycles += (!OpAddressMode::same_page(reg.pc, new_pc)) as usize + 1;
        result.reg.pc = new_pc;
    }
    result
}

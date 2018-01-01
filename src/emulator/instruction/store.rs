//
// Copyright 2017 hassel_lib6502 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use emulator::opcode::{CpuAddressMode, OpAddressMode, OpParam};
use emulator::registers::Registers;
use emulator::instruction::executor::InstructionResult;
use emulator::instruction::executor::InstructionFn;
use emulator::instruction::executor::Write;

// TODO: unit test
impl_instruction!(STA => execute_sta [mode, params, reg, bus, result] {
    let addr = mode.address(params, reg, bus).1;
    result.writes.push(Write::new(addr, reg.a));
});

// TODO: unit test
impl_instruction!(STX => execute_stx [mode, params, reg, bus, result] {
    let addr = mode.address(params, reg, bus).1;
    result.writes.push(Write::new(addr, reg.x));
});

// TODO: unit test
impl_instruction!(STY => execute_sty [mode, params, reg, bus, result] {
    let addr = mode.address(params, reg, bus).1;
    result.writes.push(Write::new(addr, reg.y));
});

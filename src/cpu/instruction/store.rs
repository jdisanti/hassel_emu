//
// Copyright 2017 hassel_lib6502 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use bus::Bus;
use cpu::opcode::{CpuAddressMode, OpAddressMode, OpParam};
use cpu::registers::Registers;
use cpu::instruction::executor::InstructionResult;
use cpu::instruction::executor::InstructionFn;
use cpu::instruction::executor::Write;

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

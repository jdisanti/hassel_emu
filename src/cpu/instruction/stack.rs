//
// Copyright 2017 hassel_lib6502 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use cpu::opcode::OpParam;
use cpu::opcode::OpAddressMode;
use cpu::registers::Registers;
use cpu::instruction::executor::InstructionResult;
use cpu::instruction::executor::InstructionFn;
use cpu::instruction::common::pop;
use cpu::instruction::common::push;

// TODO: unit test
impl_instruction!(PHA => execute_pha [_mode, _params, reg, _bus, result] {
    result = push(result, reg.a);
});

// TODO: unit test
impl_instruction!(PHP => execute_php [_mode, _params, reg, _bus, result] {
    // The PHP instruction sets bit 4 when pushing the status register to the stack
    result = push(result, reg.status.value() | 0x10);
});

// TODO: unit test
impl_instruction!(PLA => execute_pla [_mode, _params, _reg, bus, result] {
    let val = pop(&mut result, bus);
    result.reg.set_reg_a(val);
});

// TODO: unit test
impl_instruction!(PLP => execute_plp [_mode, _params, _reg, bus, result] {
    let val = pop(&mut result, bus);
    result.reg.status.set_value(val);
});

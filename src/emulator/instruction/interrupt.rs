//
// Copyright 2017 hassel_emu Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use emulator::opcode::OpParam;
use emulator::opcode::OpAddressMode;
use emulator::registers::Registers;
use emulator::instruction::executor::InstructionResult;
use emulator::instruction::executor::InstructionFn;
use emulator::instruction::common::push;

const BRK_VECTOR: u16 = 0xFFFE;

// TODO: unit test
impl_instruction!(BRK => execute_brk [_mode, _params, reg, memory, result] {
    let reg_pc = reg.pc + 1;
    let reg_status = reg.status.value() | 0x10;
    result = push(result, (reg_pc >> 8) as u8);
    result = push(result, (reg_pc & 0x0F) as u8);
    result = push(result, reg_status);

    result.reg.status.set_brk(true);
    result.reg.pc = memory.read().word(BRK_VECTOR);
});

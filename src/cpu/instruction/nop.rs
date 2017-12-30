//
// Copyright 2017 hassel_emu Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use cpu::bus::Bus;
use cpu::opcode::{CpuAddressMode, OpAddressMode, OpParam};
use cpu::registers::Registers;
use cpu::instruction::executor::InstructionResult;
use cpu::instruction::executor::InstructionFn;

impl_instruction!(NOP => execute_nop [_mode, _params, _reg, _bus, result] {
});

impl_instruction!(TOP => execute_top [mode, params, reg, bus, result] {
    result.cycles += mode.address(params, reg, bus).0 as usize;
});

#[cfg(test)]
mod tests {
    use cpu::instruction::common::{execute, new_result};
    use cpu::opcode::OpAddressMode::*;
    use cpu::bus::TestBus;
    use cpu::opcode::OpParam;
    use cpu::registers::Registers;

    test_instruction!(test_execute_top_abs, TOP, [reg, bus] {
        let result = execute(TOP, Absolute, &OpParam::Word(0), reg, bus, new_result());
        assert_eq!(0, result.cycles);
    });

    test_instruction!(test_execute_top_abs_x_cycle, TOP, [reg, bus] {
        reg.x = 10;
        let result = execute(TOP, AbsoluteOffsetX, &OpParam::Word(0xFE), reg, bus, new_result());
        assert_eq!(1, result.cycles);
    });

    test_instruction!(test_execute_top_abs_x_no_cycle, TOP, [reg, bus] {
        let result = execute(TOP, AbsoluteOffsetX, &OpParam::Word(0x20), reg, bus, new_result());
        assert_eq!(0, result.cycles);
    });
}

//
// Copyright 2017 hassel_lib6502 Developers
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

impl_instruction!(TAX => execute_tax [_mode, _params, reg, _bus, result] {
    result.reg.set_reg_x(reg.a);
});

impl_instruction!(TAY => execute_tay [_mode, _params, reg, _bus, result] {
    result.reg.set_reg_y(reg.a);
});

// TODO: unit test
impl_instruction!(TSX => execute_tsx [_mode, _params, reg, _bus, result] {
    result.reg.set_reg_x(reg.sp);
});

// TODO: unit test
impl_instruction!(TXA => execute_txa [_mode, _params, reg, _bus, result] {
    result.reg.set_reg_a(reg.x);
});

// TODO: unit test
impl_instruction!(TXS => execute_txs [_mode, _params, reg, _bus, result] {
    result.reg.sp = reg.x;
});

// TODO: unit test
impl_instruction!(TYA => execute_tya [_mode, _params, reg, _bus, result] {
    result.reg.set_reg_a(reg.y);
});

#[cfg(test)]
mod tests {
    use emulator::instruction::common::{execute, new_result};
    use emulator::opcode::OpAddressMode::*;
    use emulator::opcode::OpParam;

    test_instruction!(test_tax_and_tay, TAX, [reg, bus] {
        use super::{TAX, TAY};

        reg.a = 6;
        let result = execute(TAX, Implied, &OpParam::None, reg, bus, new_result());
        assert_eq!(6, result.reg.x);

        let result = execute(TAY, Implied, &OpParam::None, reg, bus, new_result());
        assert_eq!(6, result.reg.y);
    });
}

//
// Copyright 2017 hassel_emu Developers
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

impl_instruction!(ADC => execute_adc [mode, params, reg, memory, result] {
    let (page_boundary, operand) = mode.address_and_read_byte(params, reg, memory);

    let mut val = reg.a as u16;
    val += operand as u16;
    if reg.status.carry() {
        val += 1;
    }

    result.reg.status.set_overflow((reg.a ^ val as u8) & (operand ^ val as u8) & 0x80 > 0);
    result.reg.status.set_carry(val > 0xFF);
    result.reg.status.set_nz_from(val as u8);
    result.reg.a = val as u8;

    result.cycles += page_boundary as usize;
});

impl_instruction!(SBC => execute_sbc [mode, params, reg, memory, result] {
    let (page_boundary, operand) = mode.address_and_read_byte(params, reg, memory);

    let mut val = reg.a as u16;
    val = val.wrapping_sub(operand as u16);
    if !reg.status.carry() {
        val = val.wrapping_sub(1);
    }

    result.reg.status.set_overflow((reg.a ^ val as u8) & (!operand ^ val as u8) & 0x80 > 0);
    result.reg.status.set_carry(val <= 0xFF);
    result.reg.status.set_nz_from(val as u8);
    result.reg.a = val as u8;

    result.cycles += page_boundary as usize;
});

// TODO: unit test
impl_instruction!(DEC => execute_dec [mode, params, reg, memory, result] {
    let address = mode.address(params, reg, memory).1;
    let val = memory.read().byte(address).wrapping_sub(1);
    result.reg.status.set_nz_from(val);
    result.writes.push(Write::new(address, val));
});

// TODO: unit test
impl_instruction!(DEX => execute_dex [_mode, _params, reg, _memory, result] {
    result.reg.set_reg_x(reg.x.wrapping_sub(1));
});

// TODO: unit test
impl_instruction!(DEY => execute_dey [_mode, _params, reg, _memory, result] {
    result.reg.set_reg_y(reg.y.wrapping_sub(1));
});

// TODO: unit test
impl_instruction!(INC => execute_inc [mode, params, reg, memory, result] {
    let address = mode.address(params, reg, memory).1;
    let val = memory.read().byte(address).wrapping_add(1);
    result.reg.status.set_nz_from(val);
    result.writes.push(Write::new(address, val));
});

// TODO: unit test
impl_instruction!(INX => execute_inx [_mode, _params, reg, _memory, result] {
    result.reg.set_reg_x(reg.x.wrapping_add(1));
});

// TODO: unit test
impl_instruction!(INY => execute_iny [_mode, _params, reg, _memory, result] {
    result.reg.set_reg_y(reg.y.wrapping_add(1));
});

#[cfg(test)]
mod tests {
    use emulator::instruction::common::{execute, new_result};
    use emulator::opcode::OpAddressMode::*;
    use emulator::opcode::OpParam;

    test_instruction!(test_adc_simple, ADC, [reg, memory] {
        reg.a = 1;
        let result = execute(ADC, Immediate, &OpParam::Byte(1), reg, memory, new_result());
        assert_eq!(2, result.reg.a);
        assert_eq!(false, result.reg.status.carry());
        assert_eq!(false, result.reg.status.negative());
        assert_eq!(false, result.reg.status.zero());
        assert_eq!(false, result.reg.status.overflow());
    });

    test_instruction!(test_adc_carry, ADC, [reg, memory] {
        reg.a = 1;
        reg.status.set_carry(true);
        let result = execute(ADC, Immediate, &OpParam::Byte(1), reg, memory, new_result());
        assert_eq!(3, result.reg.a);
        assert_eq!(false, result.reg.status.carry());
        assert_eq!(false, result.reg.status.negative());
        assert_eq!(false, result.reg.status.zero());
        assert_eq!(false, result.reg.status.overflow());
    });

    test_instruction!(test_adc_carry_and_zero, ADC, [reg, memory] {
        reg.a = 0xFF;
        let result = execute(ADC, Immediate, &OpParam::Byte(1), reg, memory, new_result());
        assert_eq!(0, result.reg.a);
        assert_eq!(true, result.reg.status.carry());
        assert_eq!(false, result.reg.status.negative());
        assert_eq!(true, result.reg.status.zero());
        assert_eq!(false, result.reg.status.overflow());
    });

    test_instruction!(test_adc_negative, ADC, [reg, memory] {
        reg.a = 0x01;
        let result = execute(ADC, Immediate, &OpParam::Byte(0xF0), reg, memory, new_result());
        assert_eq!(0xF1, result.reg.a);
        assert_eq!(false, result.reg.status.carry());
        assert_eq!(true, result.reg.status.negative());
        assert_eq!(false, result.reg.status.zero());
        assert_eq!(false, result.reg.status.overflow());
    });

    test_instruction!(test_adc_overflow, ADC, [reg, memory] {
        reg.a = 0x01;
        let result = execute(ADC, Immediate, &OpParam::Byte(0x7F), reg, memory, new_result());
        assert_eq!(0x80, result.reg.a);
        assert_eq!(false, result.reg.status.carry());
        assert_eq!(true, result.reg.status.negative());
        assert_eq!(false, result.reg.status.zero());
        assert_eq!(true, result.reg.status.overflow());
    });

    test_instruction!(test_sbc_simple, SBC, [reg, memory] {
        reg.a = 5;
        reg.status.set_carry(true);
        let result = execute(SBC, Immediate, &OpParam::Byte(1), reg, memory, new_result());
        assert_eq!(4, result.reg.a);
        assert_eq!(true, result.reg.status.carry());
        assert_eq!(false, result.reg.status.negative());
        assert_eq!(false, result.reg.status.zero());
        assert_eq!(false, result.reg.status.overflow());
    });

    test_instruction!(test_sbc_carry, SBC, [reg, memory] {
        reg.a = 5;
        reg.status.set_carry(false);
        let result = execute(SBC, Immediate, &OpParam::Byte(1), reg, memory, new_result());
        assert_eq!(3, result.reg.a);
        assert_eq!(true, result.reg.status.carry());
        assert_eq!(false, result.reg.status.negative());
        assert_eq!(false, result.reg.status.zero());
        assert_eq!(false, result.reg.status.overflow());
    });

    test_instruction!(test_sbc_negative, SBC, [reg, memory] {
        reg.a = 1;
        reg.status.set_carry(true);
        let result = execute(SBC, Immediate, &OpParam::Byte(2), reg, memory, new_result());
        assert_eq!(0xFF, result.reg.a);
        assert_eq!(false, result.reg.status.carry());
        assert_eq!(true, result.reg.status.negative());
        assert_eq!(false, result.reg.status.zero());
        assert_eq!(false, result.reg.status.overflow());
    });

    test_instruction!(test_sbc_overflow, SBC, [reg, memory] {
        reg.a = 1;
        reg.status.set_carry(true);
        let result = execute(SBC, Immediate, &OpParam::Byte(0x80), reg, memory, new_result());
        assert_eq!(0x81, result.reg.a);
        assert_eq!(false, result.reg.status.carry());
        assert_eq!(true, result.reg.status.negative());
        assert_eq!(false, result.reg.status.zero());
        assert_eq!(true, result.reg.status.overflow());
    });

    test_instruction!(test_sbc_overflow_carry, SBC, [reg, memory] {
        reg.a = 1;
        reg.status.set_carry(false);
        let result = execute(SBC, Immediate, &OpParam::Byte(0x80), reg, memory, new_result());
        assert_eq!(0x80, result.reg.a);
        assert_eq!(false, result.reg.status.carry());
        assert_eq!(true, result.reg.status.negative());
        assert_eq!(false, result.reg.status.zero());
        assert_eq!(true, result.reg.status.overflow());
    });
}

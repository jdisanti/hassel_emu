//
// Copyright 2017 hassel_emu Developers
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
impl_instruction!(AND => execute_and [mode, params, reg, bus, result] {
    let (page_boundary, operand) = mode.address_and_read_byte(params, reg, bus);
    result.reg.set_reg_a(reg.a & operand);
    result.cycles += page_boundary as usize;
});

// TODO: unit test
impl_instruction!(ASL => execute_asl [mode, params, reg, bus, result] {
    let (_, val) = mode.address_and_read_byte(params, reg, bus);
    result.reg.status.set_carry((val & 0x80) > 0);

    let val = val << 1;
    result.reg.status.set_nz_from(val);

    match mode {
        OpAddressMode::Implied => result.reg.a = val,
        _ => result.writes.push(Write::new(mode.address(params, reg, bus).1, val)),
    }
});

// TODO: unit test
impl_instruction!(LSR => execute_lsr [mode, params, reg, bus, result] {
    let (_, val) = mode.address_and_read_byte(params, reg, bus);
    result.reg.status.set_negative(false);
    result.reg.status.set_carry((val & 1) > 0);

    let val = val >> 1;
    result.reg.status.set_zero(val == 0);

    match mode {
        OpAddressMode::Implied => result.reg.a = val,
        _ => result.writes.push(Write::new(mode.address(params, reg, bus).1, val)),
    }
});

// TODO: unit test
impl_instruction!(EOR => execute_eor [mode, params, reg, bus, result] {
    let (page_boundary, operand) = mode.address_and_read_byte(params, reg, bus);
    result.reg.set_reg_a(reg.a ^ operand);
    result.cycles += page_boundary as usize;
});

// TODO: unit test
impl_instruction!(ORA => execute_ora [mode, params, reg, bus, result] {
    let (page_boundary, operand) = mode.address_and_read_byte(params, reg, bus);
    result.reg.set_reg_a(reg.a | operand);
    result.cycles += page_boundary as usize;
});

// TODO: unit test
impl_instruction!(ROL => execute_rol [mode, params, reg, bus, result] {
    let (_, operand) = mode.address_and_read_byte(params, reg, bus);
    let val = (operand << 1) | (reg.status.carry() as u8);
    result.reg.status.set_carry((operand & 0x80) > 0);
    result.reg.status.set_nz_from(val);
    match mode {
        OpAddressMode::Implied => result.reg.a = val,
        _ => result.writes.push(Write::new(mode.address(params, reg, bus).1, val)),
    }
});

// TODO: unit test
impl_instruction!(ROR => execute_ror [mode, params, reg, bus, result] {
    let (_, operand) = mode.address_and_read_byte(params, reg, bus);
    let new_carry = (operand & 1) > 0;
    let val = (operand >> 1) | ((reg.status.carry() as u8) << 7);
    result.reg.status.set_carry(new_carry);
    result.reg.status.set_nz_from(val);
    match mode {
        OpAddressMode::Implied => result.reg.a = val,
        _ => result.writes.push(Write::new(mode.address(params, reg, bus).1, val)),
    }
});

use bus::Bus;
use cpu::opcode::OpParam;
use cpu::opcode::OpAddressMode;
use cpu::registers::Registers;
use cpu::instruction::executor::InstructionResult;
use cpu::instruction::executor::InstructionFn;
use cpu::instruction::common::compare;

// TODO: unit test
impl_instruction!(BIT => execute_bit [_mode, params, reg, bus, result] {
    let mem = bus.read_byte_mut(params.word());
    let val = reg.a & mem;
    result.reg.status.set_negative((mem & 0x80) > 0);
    result.reg.status.set_overflow((mem & 0x40) > 0);
    result.reg.status.set_zero(val == 0);
});

// TODO: unit test
impl_instruction!(CMP => execute_cmp [mode, params, reg, bus, result] {
    let (page_boundary, val) = mode.address_and_read_byte(params, reg, bus);
    compare(&mut result.reg, reg.a, val);
    result.cycles += page_boundary as usize;
});

// TODO: unit test
impl_instruction!(CPX => execute_cpx [mode, params, reg, bus, result] {
    let val = mode.address_and_read_byte(params, reg, bus).1;
    compare(&mut result.reg, reg.x, val);
});

// TODO: unit test
impl_instruction!(CPY => execute_cpy [mode, params, reg, bus, result] {
    let val = mode.address_and_read_byte(params, reg, bus).1;
    compare(&mut result.reg, reg.y, val);
});

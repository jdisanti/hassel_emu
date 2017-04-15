use bus::Bus;
use cpu::opcode::OpParam;
use cpu::opcode::OpAddressMode;
use cpu::registers::Registers;
use cpu::instruction::executor::InstructionResult;
use cpu::instruction::executor::InstructionFn;

// TODO: unit test
impl_instruction!(LDA => execute_lda [mode, params, reg, bus, result] {
    let (page_boundary, val) = mode.address_and_read_byte(params, reg, bus);
    result.reg.set_reg_a(val);
    result.cycles += page_boundary as usize;
});

// TODO: unit test
impl_instruction!(LDX => execute_ldx [mode, params, reg, bus, result] {
    let (page_boundary, val) = mode.address_and_read_byte(params, reg, bus);
    result.reg.set_reg_x(val);
    result.cycles += page_boundary as usize;
});

// TODO: unit test
impl_instruction!(LDY => execute_ldy [mode, params, reg, bus, result] {
    let (page_boundary, val) = mode.address_and_read_byte(params, reg, bus);
    result.reg.set_reg_y(val);
    result.cycles += page_boundary as usize;
});

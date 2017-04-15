use bus::Bus;
use cpu::opcode::OpParam;
use cpu::opcode::OpAddressMode;
use cpu::registers::Registers;
use cpu::instruction::executor::InstructionResult;
use cpu::instruction::executor::InstructionFn;
use cpu::instruction::common::{branch, push, pop};

// TODO: unit test
impl_instruction!(BCC => execute_bcc [_mode, params, reg, _bus, result] {
    result = branch(!reg.status.carry(), reg, params.0, result);
});

// TODO: unit test
impl_instruction!(BCS => execute_bcs [_mode, params, reg, _bus, result] {
    result = branch(reg.status.carry(), reg, params.0, result);
});

// TODO: unit test
impl_instruction!(BEQ => execute_beq [_mode, params, reg, _bus, result] {
    result = branch(reg.status.zero(), reg, params.0, result);
});

// TODO: unit test
impl_instruction!(BMI => execute_bmi [_mode, params, reg, _bus, result] {
    result = branch(reg.status.negative(), reg, params.0, result);
});

// TODO: unit test
impl_instruction!(BNE => execute_bne [_mode, params, reg, _bus, result] {
    result = branch(!reg.status.zero(), reg, params.0, result);
});

// TODO: unit test
impl_instruction!(BPL => execute_bpl [_mode, params, reg, _bus, result] {
    result = branch(!reg.status.negative(), reg, params.0, result);
});

// TODO: unit test
impl_instruction!(BVC => execute_bvc [_mode, params, reg, _bus, result] {
    result = branch(!reg.status.overflow(), reg, params.0, result);
});

// TODO: unit test
impl_instruction!(BVS => execute_bvs [_mode, params, reg, _bus, result] {
    result = branch(reg.status.overflow(), reg, params.0, result);
});

// TODO: unit test
impl_instruction!(JMP_ABS => execute_jmp_abs [_mode, params, _reg, _bus, result] {
    result.reg.pc = params.word();
});

// TODO: unit test
impl_instruction!(JMP_INDIRECT => execute_jmp_indirect [_mode, params, _reg, bus, result] {
    result.reg.pc = Bus::read_word_bugged_boundary_mut(bus, params.word());
});

// TODO: unit test
impl_instruction!(JSR => execute_jsr [_mode, params, reg, _bus, result] {
    let pc = reg.pc.wrapping_sub(1);
    result = push(result, (pc >> 8) as u8);
    result = push(result, (pc & 0xFF) as u8);
    result.reg.pc = params.word();
});

// TODO: unit test
impl_instruction!(RTS => execute_rts [_mode, _params, _reg, bus, result] {
    let lsb = pop(&mut result, bus) as u16;
    let msb = pop(&mut result, bus) as u16;
    result.reg.pc = 1 + (lsb | (msb << 8));
});

// TODO: unit test
impl_instruction!(RTI => execute_rti [_mode, _params, _reg, bus, result] {
    let status = pop(&mut result, bus);
    let lsb = pop(&mut result, bus);
    let msb = pop(&mut result, bus);

    result.reg.status.set_brk(false);
    result.reg.status.set_value(status);
    result.reg.pc = (msb as u16) << 8 | lsb as u16;
});

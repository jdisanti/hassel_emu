use bus::Bus;
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

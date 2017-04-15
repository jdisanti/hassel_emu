use bus::Bus;
use cpu::opcode::OpParam;
use cpu::opcode::OpAddressMode;
use cpu::registers::Registers;
use cpu::instruction::executor::InstructionResult;
use cpu::instruction::executor::InstructionFn;

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
    use cpu::instruction::common::{execute, new_result};
    use cpu::opcode::OpAddressMode::*;
    use bus::TestBus;
    use cpu::opcode::OpParam;
    use cpu::registers::Registers;

    test_instruction!(test_tax_and_tay, TAX, [reg, bus] {
        use super::{TAX, TAY};

        reg.a = 6;
        let result = execute(TAX, Implied, &OpParam(0, 0), reg, bus, new_result());
        assert_eq!(6, result.reg.x);

        let result = execute(TAY, Implied, &OpParam(0, 0), reg, bus, new_result());
        assert_eq!(6, result.reg.y);
    });
}

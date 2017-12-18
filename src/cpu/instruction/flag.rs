use bus::Bus;
use cpu::opcode::OpParam;
use cpu::opcode::OpAddressMode;
use cpu::registers::Registers;
use cpu::instruction::executor::InstructionResult;
use cpu::instruction::executor::InstructionFn;

impl_instruction!(CLC => execute_clc [_mode, _params, _reg, _bus, result] {
    result.reg.status.set_carry(false);
});

impl_instruction!(CLD => execute_cld [_mode, _params, _reg, _bus, result] {
    result.reg.status.set_decimal(false);
});

impl_instruction!(CLI => execute_cli [_mode, _params, _reg, _bus, result] {
    result.reg.status.set_interrupt_inhibit(false);
});

impl_instruction!(CLV => execute_clv [_mode, _params, _reg, _bus, result] {
    result.reg.status.set_overflow(false);
});

impl_instruction!(SEC => execute_sec [_mode, _params, _reg, _bus, result] {
    result.reg.status.set_carry(true);
});

impl_instruction!(SED => execute_sed [_mode, _params, _reg, _bus, result] {
    result.reg.status.set_decimal(true);
});

impl_instruction!(SEI => execute_sei [_mode, _params, _reg, _bus, result] {
    result.reg.status.set_interrupt_inhibit(true);
});

#[cfg(test)]
mod tests {
    use cpu::instruction::common::{execute, new_result};
    use cpu::opcode::OpAddressMode::*;
    use bus::TestBus;
    use cpu::opcode::OpParam;
    use cpu::registers::Registers;

    test_instruction!(test_clear_flags, CLC, [reg, bus] {
        use super::{CLC, CLD, CLI, CLV};

        // Set all flags
        reg.status.set_value(0xFF);
        reg.status.set_brk(true);

        let result = execute(CLC, Implied, &OpParam::None, reg, bus, new_result());
        assert_eq!(0xFE, result.reg.status.value());

        let result = execute(CLD, Implied, &OpParam::None, reg, bus, new_result());
        assert_eq!(0xF7, result.reg.status.value());

        let result = execute(CLI, Implied, &OpParam::None, reg, bus, new_result());
        assert_eq!(0xFB, result.reg.status.value());

        let result = execute(CLV, Implied, &OpParam::None, reg, bus, new_result());
        assert_eq!(0xBF, result.reg.status.value());
    });

    test_instruction!(test_set_flags, SEC, [reg, bus] {
        use super::{SEC, SED, SEI};

        let result = execute(SEC, Implied, &OpParam::None, reg, bus, new_result());
        assert_eq!(0x21, result.reg.status.value());

        let result = execute(SED, Implied, &OpParam::None, reg, bus, new_result());
        assert_eq!(0x28, result.reg.status.value());

        let result = execute(SEI, Implied, &OpParam::None, reg, bus, new_result());
        assert_eq!(0x24, result.reg.status.value());
    });
}

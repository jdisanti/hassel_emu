use bus::Bus;
use cpu::opcode::OpParam;
use cpu::opcode::OpAddressMode;
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
    use bus::TestBus;
    use cpu::opcode::OpParam;
    use cpu::registers::Registers;

    test_instruction!(test_execute_top_abs, TOP, [reg, bus] {
        let result = execute(TOP, Absolute, &OpParam(0, 0), reg, bus, new_result());
        assert_eq!(0, result.cycles);
    });

    test_instruction!(test_execute_top_abs_x_cycle, TOP, [reg, bus] {
        reg.x = 10;
        let result = execute(TOP, AbsoluteOffsetX, &OpParam(0xFE, 0), reg, bus, new_result());
        assert_eq!(1, result.cycles);
    });

    test_instruction!(test_execute_top_abs_x_no_cycle, TOP, [reg, bus] {
        let result = execute(TOP, AbsoluteOffsetX, &OpParam(0x20, 0), reg, bus, new_result());
        assert_eq!(0, result.cycles);
    });
}

//
// Copyright 2017 hassel_emu Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use cpu::memory::MemoryMap;
use cpu::opcode::{self, Op, OpAddressMode, OpClass, OpParam};
use cpu::registers::Registers;

#[derive(Copy, Clone)]
pub struct Write {
    pub address: u16,
    pub value: u8,
}

impl Write {
    pub fn new(address: u16, value: u8) -> Write {
        Write {
            address: address,
            value: value,
        }
    }
}

pub struct InstructionResult {
    pub reg: Registers,
    pub writes: Vec<Write>,
    pub cycles: usize,
}

impl InstructionResult {
    pub fn new() -> InstructionResult {
        InstructionResult {
            reg: Registers::new(),
            writes: Vec::new(),
            cycles: 0,
        }
    }
}

pub type InstructionFn = &'static Fn(OpAddressMode, &OpParam, &Registers, &mut MemoryMap, InstructionResult) -> InstructionResult;

struct Instruction {
    pub op: Op,
    pub func: InstructionFn,
}

pub struct Executor {
}

impl Executor {
    pub fn new() -> Executor {
        Executor {
        }
    }

    pub fn execute_instruction(&mut self, reg: &Registers, memory: &mut MemoryMap,
            mut result: InstructionResult) -> InstructionResult {
        let op = opcode::decode_op(memory, reg.pc);
        let instruction = Instruction {
            op: op,
            func: match_impl(op.code.class),
        };

        let op = instruction.op;

        result.writes.clear();
        result.reg = *reg;
        result.reg.pc += op.code.len as u16;
        result.cycles = op.code.base_cycles as usize;

        let reg = result.reg;
        (instruction.func)(op.code.address_mode, &op.param, &reg, memory, result)
    }
}

fn match_impl(op_class: OpClass) -> InstructionFn {
    use cpu::opcode::OpClass::*;

    use cpu::instruction::nop::{NOP, TOP};
    use cpu::instruction::interrupt::BRK;
    use cpu::instruction::flag::{CLC, CLD, CLI, CLV, SEC, SED, SEI};
    use cpu::instruction::load::{LDA, LDX, LDY};
    use cpu::instruction::store::{STA, STX, STY};
    use cpu::instruction::stack::{PHA, PHP, PLA, PLP};
    use cpu::instruction::transfer::{TAX, TAY, TSX, TXA, TXS, TYA};
    use cpu::instruction::compare::{BIT, CMP, CPX, CPY};
    use cpu::instruction::branch::{BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS,
                                   JMP, JSR, RTS, RTI};
    use cpu::instruction::bitwise::{AND, ASL, LSR, EOR, ORA, ROL, ROR};
    use cpu::instruction::arithmetic::{ADC, SBC, DEC, DEX, DEY, INC, INX, INY};

    match op_class {
        Nop => NOP, Top => TOP, Brk => BRK,

        // Flag modifiers
        Clc => CLC, Cld => CLD, Cli => CLI, Clv => CLV,
        Sec => SEC, Sed => SED, Sei => SEI,

        // Load/store
        Lda => LDA, Ldx => LDX, Ldy => LDY,
        Sta => STA, Stx => STX, Sty => STY,

        // Stack
        Pha => PHA, Php => PHP, Pla => PLA, Plp => PLP,

        // Transfer
        Tax => TAX, Tay => TAY, Tsx => TSX, Txa => TXA,
        Txs => TXS, Tya => TYA,

        // Compare
        Bit => BIT, Cmp => CMP, Cpx => CPX, Cpy => CPY,

        // Branch
        Bcc => BCC, Bcs => BCS, Beq => BEQ, Bmi => BMI,
        Bne => BNE, Bpl => BPL, Bvc => BVC, Bvs => BVS,
        Jmp => JMP, Jsr => JSR, Rts => RTS, Rti => RTI,

        // Bitwise
        And => AND, Asl => ASL, Lsr => LSR, Eor => EOR,
        Ora => ORA, Rol => ROL, Ror => ROR,

        // Arithmetic
        Adc => ADC, Sbc => SBC,
        Dec => DEC, Dex => DEX, Dey => DEY,
        Inc => INC, Inx => INX, Iny => INY,
    }
}

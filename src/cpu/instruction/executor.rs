use cpu::opcode::Op;
use cpu::opcode::OpCode;
use cpu::opcode::OpParam;
use cpu::opcode::OpAddressMode;
use bus::Bus;
use cpu::registers::Registers;

use std::collections::HashMap;

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

pub type InstructionFn = &'static Fn(OpAddressMode, &OpParam, &Registers, &mut Bus, InstructionResult) -> InstructionResult;

struct Instruction {
    pub op: Op,
    pub func: InstructionFn,
}

pub struct Executor {
    instruction_cache: HashMap<u16, Instruction>,
}

impl Executor {
    pub fn new() -> Executor {
        Executor {
            instruction_cache: HashMap::new(),
        }
    }

    pub fn execute_instruction(&mut self, reg: &Registers, bus: &mut Bus,
            mut result: InstructionResult) -> InstructionResult {
        if !self.instruction_cache.contains_key(&reg.pc) {
            let op = Op::decode(bus, reg.pc);
            let code = op.code;
            self.instruction_cache.insert(reg.pc, Instruction {
                op: op,
                func: match_impl(code),
            });
        }

        let instruction = self.instruction_cache.get(&reg.pc).expect("cached");
        let op = instruction.op;

        result.writes.clear();
        result.reg = *reg;
        result.reg.pc += op.len;
        result.cycles = op.base_cycles as usize;

        let reg = result.reg;
        (instruction.func)(op.address_mode, &op.param, &reg, bus, result)
    }
}

fn match_impl(op_code: OpCode) -> InstructionFn {
    use cpu::opcode::OpCode::*;

    use cpu::instruction::nop::{NOP, TOP};
    use cpu::instruction::interrupt::BRK;
    use cpu::instruction::flag::{CLC, CLD, CLI, CLV, SEC, SED, SEI};
    use cpu::instruction::load::{LDA, LDX, LDY};
    use cpu::instruction::store::{STA, STX, STY};
    use cpu::instruction::stack::{PHA, PHP, PLA, PLP};
    use cpu::instruction::transfer::{TAX, TAY, TSX, TXA, TXS, TYA};
    use cpu::instruction::compare::{BIT, CMP, CPX, CPY};
    use cpu::instruction::branch::{BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS,
                                   JMP_ABS, JMP_INDIRECT, JSR, RTS, RTI};
    use cpu::instruction::bitwise::{AND, ASL, LSR, EOR, ORA, ROL, ROR};
    use cpu::instruction::arithmetic::{ADC, SBC, DEC, DEX, DEY, INC, INX, INY};

    match op_code {
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
        JmpAbs => JMP_ABS,
        JmpIndirect => JMP_INDIRECT,
        Jsr => JSR, Rts => RTS, Rti => RTI,

        // Bitwise
        And => AND, Asl => ASL, Lsr => LSR, Eor => EOR,
        Ora => ORA, Rol => ROL, Ror => ROR,

        // Arithmetic
        Adc => ADC, Sbc => SBC,
        Dec => DEC, Dex => DEX, Dey => DEY,
        Inc => INC, Inx => INX, Iny => INY,
    }
}

use bus::Bus;
use cpu::registers::Registers;

const ADDR_PAGE_MASK: u16 = 0xFF00;

macro_rules! configure_opcodes {
    ( ($enum_name:ident) {
        $( $code:expr => ($name:expr, $opname:ident, $len:expr, $cycles:expr, $mode:expr) ),*
    } ) => {
        impl Op {
            pub fn decode(bus: &mut Bus, reg_pc: u16) -> Op {
                use cpu::opcode::OpAddressMode::*;

                let instr = bus.read_byte_mut(reg_pc as u16);
                match instr {
                    $( $code => Op::from(OpCode::$opname, $len, $cycles, bus, reg_pc, $code, $name, $mode) ),*
                    , _ => panic!("Unknown opcode ${:02X} at ${:04X}", instr, reg_pc),
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OpParam (pub u8, pub u8);

impl OpParam {
    #[inline]
    pub fn word(&self) -> u16 {
        ((self.1 as u16) << 8) | (self.0 as u16)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OpAddressMode {
    Implied,
    Immediate,
    Absolute,
    AbsoluteOffsetX,
    AbsoluteOffsetY,
    ZeroPage,
    ZeroPageOffsetX,
    ZeroPageOffsetY,
    PCOffset,
    Indirect,
    PreIndirectX,
    PostIndirectY,
}

impl OpAddressMode {
    #[inline]
    pub fn same_page(addr1: u16, addr2: u16) -> bool {
        (addr1 & ADDR_PAGE_MASK) == (addr2 & ADDR_PAGE_MASK)
    }

    #[inline]
    fn offset(addr: u16, offset: u8) -> (bool, u16) {
        let result = addr.wrapping_add(offset as u16);
        let different_page = !OpAddressMode::same_page(result, addr);
        (different_page, result)
    }

    pub fn debug_address(&self, param: &OpParam, reg: &Registers, bus: &Bus) -> u16 {
        use cpu::opcode::OpAddressMode::*;
        let addr = match *self {
            Implied => 0,
            Immediate => param.word(),
            Absolute => param.word(),
            AbsoluteOffsetX => return OpAddressMode::offset(param.word(), reg.x).1,
            AbsoluteOffsetY => return OpAddressMode::offset(param.word(), reg.y).1,
            ZeroPage => param.word(),
            ZeroPageOffsetX => param.0.wrapping_add(reg.x) as u16,
            ZeroPageOffsetY => param.0.wrapping_add(reg.y) as u16,
            PCOffset => (reg.pc & 0xFF00) + (param.0.wrapping_add(reg.pc as u8) as u16),
            Indirect => Bus::read_word(bus, param.word()),
            PreIndirectX => Bus::read_word_zero_page(bus, param.0.wrapping_add(reg.x)),
            PostIndirectY => {
                let addr = Bus::read_word_zero_page(bus, param.0);
                return OpAddressMode::offset(addr, reg.y).1;
            }
        };

        addr
    }

    pub fn address(&self, param: &OpParam, reg: &Registers, bus: &mut Bus) -> (bool, u16) {
        use cpu::opcode::OpAddressMode::*;
        let addr = match *self {
            Implied => 0,
            Immediate => param.word(),
            Absolute => param.word(),
            AbsoluteOffsetX => return OpAddressMode::offset(param.word(), reg.x),
            AbsoluteOffsetY => return OpAddressMode::offset(param.word(), reg.y),
            ZeroPage => param.word(),
            ZeroPageOffsetX => param.0.wrapping_add(reg.x) as u16,
            ZeroPageOffsetY => param.0.wrapping_add(reg.y) as u16,
            PCOffset => unreachable!(),
            Indirect => Bus::read_word_mut(bus, param.word()),
            PreIndirectX => Bus::read_word_zero_page_mut(bus, param.0.wrapping_add(reg.x)),
            PostIndirectY => {
                let addr = Bus::read_word_zero_page_mut(bus, param.0);
                return OpAddressMode::offset(addr, reg.y);
            }
        };

        (false, addr)
    }

    pub fn address_and_read_byte(&self, param: &OpParam, reg: &Registers, bus: &mut Bus) -> (bool, u8) {
        use cpu::opcode::OpAddressMode::*;
        let (different_page, addr) = self.address(param, reg, bus);
        match *self {
            Implied => (false, reg.a),
            PCOffset => unreachable!(),
            Immediate => (false, param.0),
            AbsoluteOffsetX | AbsoluteOffsetY | PostIndirectY => {
                (different_page, bus.read_byte_mut(addr))
            },
            _ => (false, bus.read_byte_mut(addr)),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OpCode {
    Nop, Top, Brk,
    Clc, Cld, Cli, Clv, Sec, Sed, Sei,
    Lda, Ldx, Ldy,
    Sta, Stx, Sty,
    Pha, Php, Pla, Plp,
    Tax, Tay, Tsx, Txa, Txs, Tya,
    Bit,
    Cmp, Cpx, Cpy,
    Bcc, Bcs, Beq, Bmi, Bne, Bpl, Bvc, Bvs,
    JmpAbs, JmpIndirect, Jsr,
    Rti, Rts,
    And, Asl, Eor, Lsr, Ora, Rol, Ror,
    Adc, Dec, Dex, Dey, Inc, Inx, Iny, Sbc,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Op {
    pub code: OpCode,
    pub len: u16,
    pub base_cycles: u16,

    pub address_mode: OpAddressMode,
    pub param: OpParam,

    code_num: u8,
    name_str: &'static str,
}

configure_opcodes!((OpCode) {
    0xEA => ("NOP", Nop, 1, 2, Implied),
    0x1A => ("NOP", Nop, 1, 2, Implied),
    0x3A => ("NOP", Nop, 1, 2, Implied),
    0x5A => ("NOP", Nop, 1, 2, Implied),
    0x7A => ("NOP", Nop, 1, 2, Implied),
    0xDA => ("NOP", Nop, 1, 2, Implied),
    0xFA => ("NOP", Nop, 1, 2, Implied),

    // Undocumented "double NOPs"
    0x04 => ("DOP", Nop, 2, 3, ZeroPage),
    0x14 => ("DOP", Nop, 2, 4, ZeroPageOffsetX),
    0x34 => ("DOP", Nop, 2, 4, ZeroPageOffsetX),
    0x44 => ("DOP", Nop, 2, 3, ZeroPage),
    0x54 => ("DOP", Nop, 2, 4, ZeroPageOffsetX),
    0x64 => ("DOP", Nop, 2, 3, ZeroPage),
    0x74 => ("DOP", Nop, 2, 4, ZeroPageOffsetX),
    0x80 => ("DOP", Nop, 2, 2, Immediate),
    0x82 => ("DOP", Nop, 2, 2, Immediate),
    0x89 => ("DOP", Nop, 2, 2, Immediate),
    0xC2 => ("DOP", Nop, 2, 2, Immediate),
    0xD4 => ("DOP", Nop, 2, 4, ZeroPageOffsetX),
    0xE2 => ("DOP", Nop, 2, 2, Immediate),
    0xF4 => ("DOP", Nop, 2, 4, ZeroPageOffsetX),

    // Undocumented "tripple NOPs"
    0x0C => ("TOP", Nop, 3, 4, Absolute),
    0x1C => ("TOP", Top, 3, 4, AbsoluteOffsetX),
    0x3C => ("TOP", Top, 3, 4, AbsoluteOffsetX),
    0x5C => ("TOP", Top, 3, 4, AbsoluteOffsetX),
    0x7C => ("TOP", Top, 3, 4, AbsoluteOffsetX),
    0xDC => ("TOP", Top, 3, 4, AbsoluteOffsetX),
    0xFC => ("TOP", Top, 3, 4, AbsoluteOffsetX),

    0x00 => ("BRK", Brk, 1, 7, Implied),

    // Flag modifiers
    0x18 => ("CLC", Clc, 1, 2, Implied),
    0xD8 => ("CLD", Cld, 1, 2, Implied),
    0x58 => ("CLI", Cli, 1, 2, Implied),
    0xB8 => ("CLV", Clv, 1, 2, Implied),
    0x38 => ("SEC", Sec, 1, 2, Implied),
    0xF8 => ("SED", Sed, 1, 2, Implied),
    0x78 => ("SEI", Sei, 1, 2, Implied),

    // LDA
    0xA9 => ("LDA", Lda, 2, 2, Immediate),
    0xA5 => ("LDA", Lda, 2, 3, ZeroPage),
    0xB5 => ("LDA", Lda, 2, 4, ZeroPageOffsetX),
    0xAD => ("LDA", Lda, 3, 4, Absolute),
    0xBD => ("LDA", Lda, 3, 4, AbsoluteOffsetX),
    0xB9 => ("LDA", Lda, 3, 4, AbsoluteOffsetY),
    0xA1 => ("LDA", Lda, 2, 6, PreIndirectX),
    0xB1 => ("LDA", Lda, 2, 5, PostIndirectY),

    // LDX
    0xA2 => ("LDX", Ldx, 2, 2, Immediate),
    0xA6 => ("LDX", Ldx, 2, 3, ZeroPage),
    0xB6 => ("LDX", Ldx, 2, 4, ZeroPageOffsetY),
    0xAE => ("LDX", Ldx, 3, 4, Absolute),
    0xBE => ("LDX", Ldx, 3, 4 /* (+1) */, AbsoluteOffsetY),

    // LDY
    0xA0 => ("LDY", Ldy, 2, 2, Immediate),
    0xA4 => ("LDY", Ldy, 2, 3, ZeroPage),
    0xB4 => ("LDY", Ldy, 2, 4, ZeroPageOffsetX),
    0xAC => ("LDY", Ldy, 3, 4, Absolute),
    0xBC => ("LDY", Ldy, 3, 4 /* (+1) */, AbsoluteOffsetX),

    // STA
    0x85 => ("STA", Sta, 2, 3, ZeroPage),
    0x95 => ("STA", Sta, 2, 4, ZeroPageOffsetX),
    0x8D => ("STA", Sta, 3, 4, Absolute),
    0x9D => ("STA", Sta, 3, 5, AbsoluteOffsetX),
    0x99 => ("STA", Sta, 3, 5, AbsoluteOffsetY),
    0x81 => ("STA", Sta, 2, 6, PreIndirectX),
    0x91 => ("STA", Sta, 2, 6, PostIndirectY),

    // STX
    0x86 => ("STX", Stx, 2, 3, ZeroPage),
    0x96 => ("STX", Stx, 2, 4, ZeroPageOffsetY),
    0x8E => ("STX", Stx, 3, 4, Absolute),

    // STY
    0x84 => ("STY", Sty, 2, 3, ZeroPage),
    0x94 => ("STY", Sty, 2, 4, ZeroPageOffsetX),
    0x8C => ("STY", Sty, 3, 4, Absolute),

    // Stack
    0x48 => ("PHA", Pha, 1, 3, Implied),
    0x08 => ("PHP", Php, 1, 3, Implied),
    0x68 => ("PLA", Pla, 1, 4, Implied),
    0x28 => ("PLP", Plp, 1, 4, Implied),

    // Transfer
    0xAA => ("TAX", Tax, 1, 2, Implied),
    0xA8 => ("TAY", Tay, 1, 2, Implied),
    0xBA => ("TSX", Tsx, 1, 2, Implied),
    0x8A => ("TXA", Txa, 1, 2, Implied),
    0x9A => ("TXS", Txs, 1, 2, Implied),
    0x98 => ("TYA", Tya, 1, 2, Implied),

    ///////////////////////////////////////
    // Compare
    ///////////////////////////////////////

    // BIT
    0x24 => ("BIT", Bit, 2, 3, ZeroPage),
    0x2C => ("BIT", Bit, 3, 4, Absolute),

    // CMP
    0xC9 => ("CMP", Cmp, 2, 2, Immediate),
    0xC5 => ("CMP", Cmp, 2, 3, ZeroPage),
    0xD5 => ("CMP", Cmp, 2, 4, ZeroPageOffsetX),
    0xCD => ("CMP", Cmp, 3, 4, Absolute),
    0xDD => ("CMP", Cmp, 3, 4 /* (+1) */, AbsoluteOffsetX),
    0xD9 => ("CMP", Cmp, 3, 4 /* (+1) */, AbsoluteOffsetY),
    0xC1 => ("CMP", Cmp, 2, 6, PreIndirectX),
    0xD1 => ("CMP", Cmp, 2, 5 /* (+1) */, PostIndirectY),

    // CPX
    0xE0 => ("CPX", Cpx, 2, 2, Immediate),
    0xE4 => ("CPX", Cpx, 2, 3, ZeroPage),
    0xEC => ("CPX", Cpx, 3, 4, Absolute),

    // CPY
    0xC0 => ("CPY", Cpy, 2, 2, Immediate),
    0xC4 => ("CPY", Cpy, 2, 3, ZeroPage),
    0xCC => ("CPY", Cpy, 3, 4, Absolute),

    ///////////////////////////////////////
    // Branch
    ///////////////////////////////////////

    // Branch iff
    0x90 => ("BCC", Bcc, 2, 2, PCOffset),
    0xB0 => ("BCS", Bcs, 2, 2, PCOffset),
    0xF0 => ("BEQ", Beq, 2, 2, PCOffset),
    0x30 => ("BMI", Bmi, 2, 2, PCOffset),
    0xD0 => ("BNE", Bne, 2, 2, PCOffset),
    0x10 => ("BPL", Bpl, 2, 2, PCOffset),
    0x50 => ("BVC", Bvc, 2, 2, PCOffset),
    0x70 => ("BVS", Bvs, 2, 2, PCOffset),

    // Jump
    0x4C => ("JMP", JmpAbs, 3, 3, Absolute),
    0x6C => ("JMP", JmpIndirect, 3, 5, Indirect),
    0x20 => ("JSR", Jsr, 3, 6, Absolute),

    // Return
    0x40 => ("RTI", Rti, 1, 6, Implied),
    0x60 => ("RTS", Rts, 1, 6, Implied),

    ///////////////////////////////////////
    // Bitwise
    ///////////////////////////////////////

    // AND
    0x29 => ("AND", And, 2, 2, Immediate),
    0x25 => ("AND", And, 2, 3, ZeroPage),
    0x35 => ("AND", And, 2, 4, ZeroPageOffsetX),
    0x2D => ("AND", And, 3, 4, Absolute),
    0x3D => ("AND", And, 3, 4 /* (+1) */, AbsoluteOffsetX),
    0x39 => ("AND", And, 3, 4 /* (+1) */, AbsoluteOffsetY),
    0x21 => ("AND", And, 2, 6, PreIndirectX),
    0x31 => ("AND", And, 2, 5 /* (+1) */, PostIndirectY),

    // ASL
    0x0A => ("ASL A", Asl, 1, 2, Implied),
    0x06 => ("ASL", Asl, 2, 5, ZeroPage),
    0x16 => ("ASL", Asl, 2, 6, ZeroPageOffsetX),
    0x0E => ("ASL", Asl, 3, 6, Absolute),
    0x1E => ("ASL", Asl, 3, 7, AbsoluteOffsetX),

    // EOR
    0x49 => ("EOR", Eor, 2, 2, Immediate),
    0x45 => ("EOR", Eor, 2, 3, ZeroPage),
    0x55 => ("EOR", Eor, 2, 4, ZeroPageOffsetX),
    0x4D => ("EOR", Eor, 3, 4, Absolute),
    0x5D => ("EOR", Eor, 3, 4 /* (+1) */, AbsoluteOffsetX),
    0x59 => ("EOR", Eor, 3, 4 /* (+1) */, AbsoluteOffsetY),
    0x41 => ("EOR", Eor, 2, 6, PreIndirectX),
    0x51 => ("EOR", Eor, 2, 5 /* (+1) */, PostIndirectY),

    // LSR
    0x4A => ("LSR A", Lsr, 1, 2, Implied),
    0x46 => ("LSR", Lsr, 2, 5, ZeroPage),
    0x56 => ("LSR", Lsr, 2, 6, ZeroPageOffsetX),
    0x4E => ("LSR", Lsr, 3, 6, Absolute),
    0x5E => ("LSR", Lsr, 3, 7, AbsoluteOffsetX),

    // ORA
    0x09 => ("ORA", Ora, 2, 2, Immediate),
    0x05 => ("ORA", Ora, 2, 3, ZeroPage),
    0x15 => ("ORA", Ora, 2, 4, ZeroPageOffsetX),
    0x0D => ("ORA", Ora, 3, 4, Absolute),
    0x1D => ("ORA", Ora, 3, 4 /* (+1) */, AbsoluteOffsetX),
    0x19 => ("ORA", Ora, 3, 4 /* (+1) */, AbsoluteOffsetY),
    0x01 => ("ORA", Ora, 2, 6, PreIndirectX),
    0x11 => ("ORA", Ora, 2, 5 /* (+1) */, PostIndirectY),

    // ROL
    0x2A => ("ROL A", Rol, 1, 2, Implied),
    0x26 => ("ROL", Rol, 2, 5, ZeroPage),
    0x36 => ("ROL", Rol, 2, 6, ZeroPageOffsetX),
    0x2E => ("ROL", Rol, 3, 6, Absolute),
    0x3E => ("ROL", Rol, 3, 7, AbsoluteOffsetX),

    // ROR
    0x6A => ("ROR A", Ror, 1, 2, Implied),
    0x66 => ("ROR", Ror, 2, 5, ZeroPage),
    0x76 => ("ROR", Ror, 2, 6, ZeroPageOffsetX),
    0x6E => ("ROR", Ror, 3, 6, Absolute),
    0x7E => ("ROR", Ror, 3, 7, AbsoluteOffsetX),

    ///////////////////////////////////////
    // Arithmetic
    ///////////////////////////////////////

    // ADC
    0x69 => ("ADC", Adc, 2, 2, Immediate),
    0x65 => ("ADC", Adc, 2, 3, ZeroPage),
    0x75 => ("ADC", Adc, 2, 4, ZeroPageOffsetX),
    0x6D => ("ADC", Adc, 3, 4, Absolute),
    0x7D => ("ADC", Adc, 3, 4 /* (+1) */, AbsoluteOffsetX),
    0x79 => ("ADC", Adc, 3, 4 /* (+1) */, AbsoluteOffsetY),
    0x61 => ("ADC", Adc, 2, 6, PreIndirectX),
    0x71 => ("ADC", Adc, 2, 5 /* (+1) */, PostIndirectY),

    // DEC
    0xC6 => ("DEC", Dec, 2, 5, ZeroPage),
    0xD6 => ("DEC", Dec, 2, 6, ZeroPageOffsetX),
    0xCE => ("DEC", Dec, 3, 6, Absolute),
    0xDE => ("DEC", Dec, 3, 7, AbsoluteOffsetX),

    0xCA => ("DEX", Dex, 1, 2, Implied),
    0x88 => ("DEY", Dey, 1, 2, Implied),

    // INC
    0xE6 => ("INC", Inc, 2, 5, ZeroPage),
    0xF6 => ("INC", Inc, 2, 6, ZeroPageOffsetX),
    0xEE => ("INC", Inc, 3, 6, Absolute),
    0xFE => ("INC", Inc, 3, 7, AbsoluteOffsetX),

    0xE8 => ("INX", Inx, 1, 2, Implied),
    0xC8 => ("INY", Iny, 1, 2, Implied),

    // SBC
    0xE9 => ("SBC", Sbc, 2, 2, Immediate),
    0xEB => ("SBC", Sbc, 2, 2, Immediate), // unofficial opcode same as 0xE9
    0xE5 => ("SBC", Sbc, 2, 3, ZeroPage),
    0xF5 => ("SBC", Sbc, 2, 4, ZeroPageOffsetX),
    0xED => ("SBC", Sbc, 3, 4, Absolute),
    0xFD => ("SBC", Sbc, 3, 4 /* (+1) */, AbsoluteOffsetX),
    0xF9 => ("SBC", Sbc, 3, 4 /* (+1) */, AbsoluteOffsetY),
    0xE1 => ("SBC", Sbc, 2, 6, PreIndirectX),
    0xF1 => ("SBC", Sbc, 2, 5 /* (+1) */, PostIndirectY)
});

impl Op {
    fn from(code: OpCode, len: u16, cycles: u16, bus: &mut Bus, reg_pc: u16,
                code_num: u8, name_str: &'static str, address_mode: OpAddressMode) -> Op {
        let p0 = if len > 1 { bus.read_byte_mut(reg_pc + 1) } else { 0 };
        let p1 = if len > 2 { bus.read_byte_mut(reg_pc + 2) } else { 0 };
        Op {
            code: code,
            len: len,
            base_cycles: cycles,

            address_mode: address_mode,
            param: OpParam(p0, p1),

            code_num: code_num,
            name_str: name_str,
        }
    }

    pub fn debug(&self, cpu: &::cpu::Cpu, bus: &Bus, reg_pc: u16) -> String {
        use cpu::opcode::OpAddressMode::*;
        let p = &self.param;

        let registers = cpu.registers();
        let register_view = format!(
            "A: {:02X}  X: {:02X}  Y: {:02X}  S: {:02X}",
            registers.a,
            registers.x,
            registers.y,
            registers.status.value(),
        );

        let branch_location = (reg_pc as i32).wrapping_add(p.0 as i32) as u16;

        let mem_view = match self.len {
            1 => format!("{:02X} ", self.code_num),
            2 => format!("{:02X} {:02X}", self.code_num, p.0),
            3 => format!("{:02X} {:02X} {:02X}", self.code_num, p.0, p.1),
            _ => panic!("Unknown op-code length: {}", self.len),
        };

        let addr = self.address_mode.debug_address(p, registers, bus);
        let value = bus.read_byte(addr);
        let instr_view = match self.len {
            1 => format!("{}", self.name_str),
            2 => {
                match self.address_mode {
                    Implied => unreachable!("{:?}", self),
                    Immediate => format!("{} #${:02X}", self.name_str, p.0),
                    Absolute => unreachable!("{:?}", self),
                    AbsoluteOffsetX => unreachable!("{:?}", self),
                    AbsoluteOffsetY => unreachable!("{:?}", self),
                    ZeroPage => format!("{} ${:02X}", self.name_str, p.0),
                    ZeroPageOffsetX => format!("{} ${:02X},X", self.name_str, p.0),
                    ZeroPageOffsetY => format!("{} ${:02X},Y", self.name_str, p.0),
                    PCOffset => format!("{} ${:04X}", self.name_str, branch_location),
                    Indirect => unreachable!("{:?}", self),
                    PreIndirectX => format!("{} (${:02X},X)", self.name_str, p.0),
                    PostIndirectY => format!("{} (${:02X}),Y", self.name_str, p.0),
                }
            },
            3 => {
                match self.address_mode {
                    Implied => unreachable!("{:?}", self),
                    Immediate => format!("{} #${:04X}", self.name_str, p.word()),
                    Absolute => {
                        match self.code {
                            OpCode::JmpAbs | OpCode::Jsr => format!("{} ${:04X}", self.name_str, p.word()),
                            _ => format!("{} ${:04X}", self.name_str, p.word()),
                        }
                    }
                    AbsoluteOffsetX => format!("{} ${:04X},X", self.name_str, p.word()),
                    AbsoluteOffsetY => format!("{} ${:04X},Y", self.name_str, p.word()),
                    ZeroPage => unreachable!("{:?}", self),
                    ZeroPageOffsetX => unreachable!("{:?}", self),
                    ZeroPageOffsetY => unreachable!("{:?}", self),
                    PCOffset => unreachable!("{:?}", self),
                    Indirect => format!("{} (${:04X})", self.name_str, p.word()),
                    PreIndirectX => unreachable!("{:?}", self),
                    PostIndirectY => unreachable!("{:?}", self),
                }
            },
            _ => panic!("Unknown op-code length: {}", self.len),
        };

        format!("{:<9} {:<18}  mem[{:04X}]={:02X}  {}", mem_view, instr_view, addr, value, register_view)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bus::Bus;
    use bus::TestBus;
    use cpu::registers::Registers;

    #[test]
    fn test_zero_page_wrapping() {
        let mut bus = TestBus::new();
        let mut registers = Registers::new();
        registers.x = 0xFF;

        // Write some test data to key addresses
        bus.write_byte(0x0000, 101);
        bus.write_byte(0x0100, 213);

        let params = OpParam(1, 0);
        let (page_boundary, val) =
            OpAddressMode::ZeroPageOffsetX.address_and_read_byte(&params, &registers, &mut bus);

        // Because this is zero page, the value should wrap to 0x00
        assert_eq!(101, val);
        assert_eq!(false, page_boundary);
    }
}

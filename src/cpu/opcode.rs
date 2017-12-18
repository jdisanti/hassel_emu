use bus::Bus;
use cpu::registers::Registers;

pub use hassel_lib6502::{OpParam, OpAddressMode, OpClass, OpCode, Op};

const ADDR_PAGE_MASK: u16 = 0xFF00;

pub trait CpuAddressMode {
    fn same_page(addr1: u16, addr2: u16) -> bool;
    fn offset(addr: u16, offset: u8) -> (bool, u16);
    fn debug_address(&self, param: &OpParam, reg: &Registers, bus: &Bus) -> u16;
    fn address(&self, param: &OpParam, reg: &Registers, bus: &mut Bus) -> (bool, u16);
    fn address_and_read_byte(&self, param: &OpParam, reg: &Registers, bus: &mut Bus) -> (bool, u8);
}

impl CpuAddressMode for OpAddressMode {
    #[inline]
    fn same_page(addr1: u16, addr2: u16) -> bool {
        (addr1 & ADDR_PAGE_MASK) == (addr2 & ADDR_PAGE_MASK)
    }

    #[inline]
    fn offset(addr: u16, offset: u8) -> (bool, u16) {
        let result = addr.wrapping_add(offset as u16);
        let different_page = !Self::same_page(result, addr);
        (different_page, result)
    }

    fn debug_address(&self, param: &OpParam, reg: &Registers, bus: &Bus) -> u16 {
        use cpu::opcode::OpAddressMode::*;
        let addr = match *self {
            Implied => 0,
            Immediate => param.as_u16(),
            Absolute => param.as_u16(),
            AbsoluteOffsetX => return OpAddressMode::offset(param.as_u16(), reg.x).1,
            AbsoluteOffsetY => return OpAddressMode::offset(param.as_u16(), reg.y).1,
            ZeroPage => param.as_u16(),
            ZeroPageOffsetX => param.as_u8().wrapping_add(reg.x) as u16,
            ZeroPageOffsetY => param.as_u8().wrapping_add(reg.y) as u16,
            PCOffset => (reg.pc & 0xFF00) + (param.as_u8().wrapping_add(reg.pc as u8) as u16),
            Indirect => Bus::read_word(bus, param.as_u16()),
            PreIndirectX => Bus::read_word_zero_page(bus, param.as_u8().wrapping_add(reg.x)),
            PostIndirectY => {
                let addr = Bus::read_word_zero_page(bus, param.as_u8());
                return OpAddressMode::offset(addr, reg.y).1;
            }
        };

        addr
    }

    fn address(&self, param: &OpParam, reg: &Registers, bus: &mut Bus) -> (bool, u16) {
        use cpu::opcode::OpAddressMode::*;
        let addr = match *self {
            Implied => 0,
            Immediate => param.as_u16(),
            Absolute => param.as_u16(),
            AbsoluteOffsetX => return OpAddressMode::offset(param.as_u16(), reg.x),
            AbsoluteOffsetY => return OpAddressMode::offset(param.as_u16(), reg.y),
            ZeroPage => param.as_u16(),
            ZeroPageOffsetX => param.as_u8().wrapping_add(reg.x) as u16,
            ZeroPageOffsetY => param.as_u8().wrapping_add(reg.y) as u16,
            PCOffset => unreachable!(),
            Indirect => Bus::read_word_mut(bus, param.as_u16()),
            PreIndirectX => Bus::read_word_zero_page_mut(bus, param.as_u8().wrapping_add(reg.x)),
            PostIndirectY => {
                let addr = Bus::read_word_zero_page_mut(bus, param.as_u8());
                return OpAddressMode::offset(addr, reg.y);
            }
        };

        (false, addr)
    }

    fn address_and_read_byte(&self, param: &OpParam, reg: &Registers, bus: &mut Bus) -> (bool, u8) {
        use cpu::opcode::OpAddressMode::*;
        let (different_page, addr) = self.address(param, reg, bus);
        match *self {
            Implied => (false, reg.a),
            PCOffset => unreachable!(),
            Immediate => (false, param.as_u8()),
            AbsoluteOffsetX | AbsoluteOffsetY | PostIndirectY => {
                (different_page, bus.read_byte_mut(addr))
            },
            _ => (false, bus.read_byte_mut(addr)),
        }
    }
}

pub fn decode_op(bus: &mut Bus, reg_pc: u16) -> Op {
    let op_code_value = bus.read_byte_mut(reg_pc);
    let op_code = OpCode::from_value(op_code_value).expect("invalid opcode");
    let op_param = match op_code.len {
        1 => OpParam::None,
        2 => OpParam::Byte(bus.read_byte_mut(reg_pc.wrapping_add(1))),
        3 => {
            let lo = bus.read_byte_mut(reg_pc.wrapping_add(1));
            let hi = bus.read_byte_mut(reg_pc.wrapping_add(2));
            OpParam::Word(((hi as u16) << 8) | (lo as u16))
        }
        _ => panic!("unexpected op-code length")
    };
    Op::new(op_code, op_param)
}

pub trait OpDebug {
    fn debug(&self, cpu: &::cpu::Cpu, bus: &Bus, reg_pc: u16) -> String;
}

impl OpDebug for Op {
    fn debug(&self, cpu: &::cpu::Cpu, bus: &Bus, reg_pc: u16) -> String {
        use self::OpAddressMode::*;
        let p = &self.param;

        let registers = cpu.registers();
        let register_view = format!(
            "A: {:02X}  X: {:02X}  Y: {:02X}  S: {:02X}",
            registers.a,
            registers.x,
            registers.y,
            registers.status.value(),
        );

        let mem_view = match self.code.len {
            1 => format!("{:02X} ", self.code.value),
            2 => format!("{:02X} {:02X}", self.code.value, self.param.as_u8()),
            3 => format!("{:02X} {:02X} {:02X}", self.code.value, self.param.high_byte(), self.param.low_byte()),
            _ => panic!("Unknown op-code length: {}", self.code.len),
        };

        let addr = self.code.address_mode.debug_address(p, registers, bus);
        let value = bus.read_byte(addr);
        let instr_view = match self.code.len {
            1 => format!("{}", self.code.name),
            2 => {
                match self.code.address_mode {
                    Implied => unreachable!("{:?}", self),
                    Immediate => format!("{} #${:02X}", self.code.name, self.param.as_u8()),
                    Absolute => unreachable!("{:?}", self),
                    AbsoluteOffsetX => unreachable!("{:?}", self),
                    AbsoluteOffsetY => unreachable!("{:?}", self),
                    ZeroPage => format!("{} ${:02X}", self.code.name, self.param.as_u8()),
                    ZeroPageOffsetX => format!("{} ${:02X},X", self.code.name, self.param.as_u8()),
                    ZeroPageOffsetY => format!("{} ${:02X},Y", self.code.name, self.param.as_u8()),
                    PCOffset => format!("{} ${:04X}", self.code.name, (reg_pc as i32).wrapping_add(p.as_u8() as i32) as u16),
                    Indirect => unreachable!("{:?}", self),
                    PreIndirectX => format!("{} (${:02X},X)", self.code.name, self.param.as_u8()),
                    PostIndirectY => format!("{} (${:02X}),Y", self.code.name, self.param.as_u8()),
                }
            },
            3 => {
                match self.code.address_mode {
                    Implied => unreachable!("{:?}", self),
                    Immediate => format!("{} #${:04X}", self.code.name, self.param.as_u16()),
                    Absolute => {
                        match self.code.class {
                            OpClass::JmpAbs | OpClass::Jsr => format!("{} ${:04X}", self.code.name, self.param.as_u16()),
                            _ => format!("{} ${:04X}", self.code.name, self.param.as_u16()),
                        }
                    }
                    AbsoluteOffsetX => format!("{} ${:04X},X", self.code.name, self.param.as_u16()),
                    AbsoluteOffsetY => format!("{} ${:04X},Y", self.code.name, self.param.as_u16()),
                    ZeroPage => unreachable!("{:?}", self),
                    ZeroPageOffsetX => unreachable!("{:?}", self),
                    ZeroPageOffsetY => unreachable!("{:?}", self),
                    PCOffset => unreachable!("{:?}", self),
                    Indirect => format!("{} (${:04X})", self.code.name, self.param.as_u16()),
                    PreIndirectX => unreachable!("{:?}", self),
                    PostIndirectY => unreachable!("{:?}", self),
                }
            },
            _ => panic!("Unknown op-code length: {}", self.code.len),
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

        let params = OpParam::Byte(1);
        let (page_boundary, val) =
            OpAddressMode::ZeroPageOffsetX.address_and_read_byte(&params, &registers, &mut bus);

        // Because this is zero page, the value should wrap to 0x00
        assert_eq!(101, val);
        assert_eq!(false, page_boundary);
    }
}

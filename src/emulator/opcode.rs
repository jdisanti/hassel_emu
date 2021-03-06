//
// Copyright 2017 hassel_lib6502 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use emulator::memory::MemoryMap;
use emulator::registers::Registers;

pub use hassel_lib6502::{Op, OpAddressMode, OpClass, OpCode, OpParam};

const ADDR_PAGE_MASK: u16 = 0xFF00;

pub trait CpuAddressMode {
    fn same_page(addr1: u16, addr2: u16) -> bool;
    fn offset(addr: u16, offset: u8) -> (bool, u16);
    fn address(&self, param: &OpParam, reg: &Registers, memory: &mut MemoryMap) -> (bool, u16);
    fn address_and_read_byte(&self, param: &OpParam, reg: &Registers, memory: &mut MemoryMap) -> (bool, u8);
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

    fn address(&self, param: &OpParam, reg: &Registers, memory: &mut MemoryMap) -> (bool, u16) {
        use emulator::opcode::OpAddressMode::*;
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
            Indirect => memory.read().word(param.as_u16()),
            PreIndirectX => memory
                .read()
                .word_zero_page(param.as_u8().wrapping_add(reg.x)),
            PostIndirectY => {
                let addr = memory.read().word_zero_page(param.as_u8());
                return OpAddressMode::offset(addr, reg.y);
            }
        };

        (false, addr)
    }

    fn address_and_read_byte(&self, param: &OpParam, reg: &Registers, memory: &mut MemoryMap) -> (bool, u8) {
        use emulator::opcode::OpAddressMode::*;
        let (different_page, addr) = self.address(param, reg, memory);
        match *self {
            Implied => (false, reg.a),
            PCOffset => unreachable!(),
            Immediate => (false, param.as_u8()),
            AbsoluteOffsetX | AbsoluteOffsetY | PostIndirectY => (different_page, memory.read().byte(addr)),
            _ => (false, memory.read().byte(addr)),
        }
    }
}

pub fn decode_op(memory: &mut MemoryMap, reg_pc: u16) -> Op {
    let op_code_value = memory.read().byte(reg_pc);
    let op_code = OpCode::from_value(op_code_value).expect("invalid opcode");
    let op_param = match op_code.len {
        1 => OpParam::None,
        2 => OpParam::Byte(memory.read().byte(reg_pc.wrapping_add(1))),
        3 => {
            let lo = memory.read().byte(reg_pc.wrapping_add(1));
            let hi = memory.read().byte(reg_pc.wrapping_add(2));
            OpParam::Word(((hi as u16) << 8) | (lo as u16))
        }
        _ => panic!("unexpected op-code length"),
    };
    Op::new(op_code, op_param)
}

#[cfg(test)]
mod tests {
    use super::*;
    use emulator::memory::MemoryMap;
    use emulator::registers::Registers;

    #[test]
    fn test_zero_page_wrapping() {
        let mut memory = MemoryMap::builder().ram(0x0000, 0xFFFF).build();
        let mut registers = Registers::new();
        registers.x = 0xFF;

        // Write some test data to key addresses
        memory.write().byte(0x0000, 101);
        memory.write().byte(0x0100, 213);

        let params = OpParam::Byte(1);
        let (page_boundary, val) =
            OpAddressMode::ZeroPageOffsetX.address_and_read_byte(&params, &registers, &mut memory);

        // Because this is zero page, the value should wrap to 0x00
        assert_eq!(101, val);
        assert_eq!(false, page_boundary);
    }
}

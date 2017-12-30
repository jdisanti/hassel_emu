//
// Copyright 2017 hassel_lib6502 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use cpu::bus::{Bus, BusDebugView};
use cpu::Cpu;

use std::cell::RefCell;
use std::rc::Rc;

//
//  Memory map
//  ----------
//
//  Two peripheral ports at 0xDFFE and 0xDFFF
//
//  ```
//  +-------------------+ 0x0000
//  |                   |
//  |                   |
//  |                   |
//  |                   |
//  |                   |
//  |                   |
//  |                   |
//  |                   |
//  |                   |
//  |                   |
//  |                   |
//  |        RAM        |
//  |                   |
//  |                   |
//  |                   |
//  |                   |
//  |                   |
//  |                   |
//  |                   |
//  |                   |
//  |        I/O        |
//  +-------------------+ 0xE000
//  |                   |
//  |        ROM        |
//  |                   |
//  +-------------------+
//  ```
//

const RAM_START: usize = 0x0000;
const RAM_END_INCL: usize = 0xDFFD;

const IO_START: usize = 0xDFFE;
const IO_END_INCL: usize = 0xDFFF;

const ROM_START: usize = IO_END_INCL + 1;
const ROM_END_INCL: usize = 0xFFFF;

const RAM_SIZE: usize = RAM_END_INCL + 1 - RAM_START;

pub struct CpuBus {
    ram: [u8; RAM_SIZE],
    rom: Vec<u8>,
    peripheral_bus: Rc<RefCell<Bus>>,
}

impl CpuBus {
    pub fn new(rom: Vec<u8>,
               peripheral_bus: Rc<RefCell<Bus>>) -> CpuBus {
        CpuBus {
            ram: [0u8; RAM_SIZE],
            rom: rom,
            peripheral_bus: peripheral_bus,
        }
    }
}

impl BusDebugView for CpuBus {
    fn read_byte(&self, addr: u16) -> u8 {
        let addr: usize = addr as usize;
        match addr {
            RAM_START...RAM_END_INCL => self.ram[addr],
            IO_START...IO_END_INCL => self.peripheral_bus.borrow().debug_view().read_byte(addr as u16),
            ROM_START...ROM_END_INCL => self.rom[addr - ROM_START],
            _ => { 0 }
        }
    }
}

impl Bus for CpuBus {
    fn debug_view(&self) -> &BusDebugView {
        self
    }

    fn read_byte(&mut self, addr: u16) -> u8 {
        let addr: usize = addr as usize;
        match addr {
            RAM_START...RAM_END_INCL => self.ram[addr],
            IO_START...IO_END_INCL => self.peripheral_bus.borrow_mut().read_byte(addr as u16),
            ROM_START...ROM_END_INCL => self.rom[addr - ROM_START],
            _ => { 0 }
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        let addr: usize = addr as usize;
        match addr {
            RAM_START...RAM_END_INCL => self.ram[addr] = val,
            IO_START...IO_END_INCL => self.peripheral_bus.borrow_mut().write_byte(addr as u16, val),
            ROM_START...ROM_END_INCL => panic!("Attempted to write to ROM location: 0x{:04X}", addr),
            _ => panic!("Unknown write memory location: 0x{:04X}", addr)
        }
    }

    fn step(&mut self, _cpu: &mut Cpu) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cpu::bus::*;

    use std::cell::RefCell;
    use std::rc::Rc;

    fn fake_bus() -> CpuBus {
        let peripheral_bus = Rc::new(RefCell::new(PlaceholderBus::new(String::from("IO"))));
        let rom = vec![0u8; 31744];
        CpuBus::new(rom, peripheral_bus)
    }

    #[test]
    fn test_read_word() {
        let mut bus = fake_bus();
        bus.write_byte(0x500, 0x01);
        bus.write_byte(0x501, 0x02);
        assert_eq!(0x0201, bus.read_word(0x500));
    }
}

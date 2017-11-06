use bus::Bus;

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

pub struct CpuBusDebugger {
    last_read: Vec<u16>,
    last_written: Vec<u16>,
}

impl CpuBusDebugger {
    pub fn new() -> CpuBusDebugger {
        CpuBusDebugger {
            last_read: Vec::new(),
            last_written: Vec::new(),
        }
    }

    pub fn read(&mut self, addr: u16) {
        self.last_read.push(addr);
    }

    pub fn write(&mut self, addr: u16) {
        self.last_written.push(addr);
    }

    pub fn last_written(&self) -> &Vec<u16> {
        &self.last_written
    }

    pub fn last_read(&self) -> &Vec<u16> {
        &self.last_read
    }

    pub fn clear(&mut self) {
        self.last_read.clear();
        self.last_written.clear();
    }
}

pub struct CpuBus {
    ram: [u8; RAM_SIZE],
    rom: Vec<u8>,
    peripheral_bus: Rc<RefCell<Bus>>,
    debugger: RefCell<CpuBusDebugger>,
}

impl CpuBus {
    pub fn new(rom: Vec<u8>,
               peripheral_bus: Rc<RefCell<Bus>>) -> CpuBus {
        CpuBus {
            ram: [0u8; RAM_SIZE],
            rom: rom,
            peripheral_bus: peripheral_bus,
            debugger: RefCell::new(CpuBusDebugger::new()),
        }
    }

    pub fn before_next_instruction(&self) {
        self.debugger.borrow_mut().clear();
    }

    pub fn debugger(&self) -> &RefCell<CpuBusDebugger> {
        &self.debugger
    }
}

impl Bus for CpuBus {
    fn read_byte(&self, addr: u16) -> u8 {
        self.debugger.borrow_mut().read(addr);

        let addr: usize = addr as usize;
        match addr {
            RAM_START...RAM_END_INCL => self.ram[addr],
            IO_START...IO_END_INCL => self.peripheral_bus.borrow().read_byte(addr as u16),
            ROM_START...ROM_END_INCL => self.rom[addr - ROM_START],
            _ => { 0 }
        }
    }

    fn read_byte_mut(&mut self, addr: u16) -> u8 {
        self.debugger.borrow_mut().read(addr);

        let addr: usize = addr as usize;
        match addr {
            RAM_START...RAM_END_INCL => self.ram[addr],
            IO_START...IO_END_INCL => self.peripheral_bus.borrow_mut().read_byte_mut(addr as u16),
            ROM_START...ROM_END_INCL => self.rom[addr - ROM_START],
            _ => { 0 }
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        self.debugger.borrow_mut().write(addr);

        let addr: usize = addr as usize;
        match addr {
            RAM_START...RAM_END_INCL => self.ram[addr] = val,
            IO_START...IO_END_INCL => self.peripheral_bus.borrow_mut().write_byte(addr as u16, val),
            ROM_START...ROM_END_INCL => panic!("Attempted to write to ROM location: 0x{:04X}", addr),
            _ => panic!("Unknown write memory location: 0x{:04X}", addr)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bus::*;

    use std::cell::RefCell;
    use std::rc::Rc;

    fn fake_bus() -> CpuBus {
        let peripheral_bus = Rc::new(RefCell::new(PlaceholderBus::new(String::from("IO"))));
        let rom = vec![0u8; 31744];
        CpuBus::new(&rom, peripheral_bus)
    }

    #[test]
    fn test_read_word() {
        let mut bus = fake_bus();
        bus.write_byte(0x500, 0x01);
        bus.write_byte(0x501, 0x02);
        assert_eq!(0x0201, Bus::read_word_mut(&mut bus, 0x500));
    }
}

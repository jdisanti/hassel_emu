use std::cell::RefCell;
use std::rc::Rc;

use graphics_bus::GraphicsBus;

pub trait Bus {
    // There's a mutable read for peripherals such as the PPU and Input
    // because reading some of their registers causes a state change.
    // For the most part, read_byte_mut should be used.
    fn read_byte(&self, addr: u16) -> u8;
    fn read_byte_mut(&mut self, addr: u16) -> u8;

    fn write_byte(&mut self, addr: u16, val: u8);
}

impl Bus {
    pub fn read_word(bus: &Bus, addr: u16) -> u16 {
        let lsb = bus.read_byte(addr);
        let msb = bus.read_byte(addr.wrapping_add(1));
        (msb as u16) << 8 | (lsb as u16)
    }

    pub fn read_word_zero_page(bus: &Bus, addr: u8) -> u16 {
        let lsb = bus.read_byte(addr as u16);
        let msb = bus.read_byte(addr.wrapping_add(1) as u16);
        (msb as u16) << 8 | (lsb as u16)
    }

    pub fn read_word_mut(bus: &mut Bus, addr: u16) -> u16 {
        let lsb = bus.read_byte_mut(addr);
        let msb = bus.read_byte_mut(addr.wrapping_add(1));
        (msb as u16) << 8 | (lsb as u16)
    }

    pub fn read_word_zero_page_mut(bus: &mut Bus, addr: u8) -> u16 {
        let lsb = bus.read_byte_mut(addr as u16);
        let msb = bus.read_byte_mut(addr.wrapping_add(1) as u16);
        (msb as u16) << 8 | (lsb as u16)
    }
}

const GRAPHICS_REGISTER_ADDRESS: u16 = 0xDFFE;
const IO_REGISTER_ADDRESS: u16 = 0xDFFF;

pub struct PeripheralBus {
    graphics_bus: Rc<RefCell<GraphicsBus>>,
}

impl PeripheralBus {
    pub fn new(graphics_bus: Rc<RefCell<GraphicsBus>>) -> PeripheralBus {
        PeripheralBus {
            graphics_bus: graphics_bus
        }
    }
}

impl Bus for PeripheralBus {
    fn read_byte(&self, _addr: u16) -> u8 {
        0
    }

    fn read_byte_mut(&mut self, addr: u16) -> u8 {
        if addr == GRAPHICS_REGISTER_ADDRESS {
            self.graphics_bus.borrow_mut().read_byte_mut(addr)
        } else if addr == IO_REGISTER_ADDRESS {
            // TODO
            0
        } else {
            println!("WARN: PeripheralBus called with non-peripheral address 0x{:04X}", addr);
            0
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        if addr == GRAPHICS_REGISTER_ADDRESS {
            self.graphics_bus.borrow_mut().write_byte(addr, val);
        } else if addr == IO_REGISTER_ADDRESS {
            // TODO
        } else {
            println!("WARN: PeripheralBus called with non-peripheral address 0x{:04X}", addr);
        }
    }
}

pub struct PlaceholderBus {
    name: String,
}

impl PlaceholderBus {
    pub fn new(name: String) -> PlaceholderBus {
        PlaceholderBus {
            name: name,
        }
    }
}

impl Bus for PlaceholderBus {
    fn read_byte(&self, _addr: u16) -> u8 {
        0
    }

    fn read_byte_mut(&mut self, addr: u16) -> u8 {
        println!("WARN: Read byte from placeholder {} bus at {:04X}", self.name, addr);
        0
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        println!("WARN: {:02X} written to placeholder {} bus at {:04X}", val, self.name, addr);
    }
}

#[cfg(test)]
pub struct TestBus {
    mem: Vec<u8>,
}

#[cfg(test)]
impl TestBus {
    pub fn new() -> TestBus {
        TestBus {
            mem: Vec::new(),
        }
    }
}

#[cfg(test)]
impl Bus for TestBus {
    fn read_byte(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        if self.mem.len() <= addr {
            0
        } else {
            self.mem[addr]
        }
    }

    fn read_byte_mut(&mut self, addr: u16) -> u8 {
        let addr = addr as usize;
        if self.mem.len() <= addr {
            self.mem.resize(addr + 1, 0u8);
        }
        self.mem[addr]
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        let addr = addr as usize;
        if self.mem.len() <= addr {
            self.mem.resize(addr + 1, 0u8);
        }
        self.mem[addr] = val;
    }
}

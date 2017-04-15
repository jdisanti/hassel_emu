use std::collections::HashSet;

pub trait Bus {
    // There's a mutable read for peripherals such as the PPU and Input
    // because reading some of their registers causes a state change.
    // For the most part, read_byte_mut should be used.
    fn read_byte(&self, addr: u16) -> u8;
    fn read_byte_mut(&mut self, addr: u16) -> u8;

    fn write_byte(&mut self, addr: u16, val: u8);
}

impl Bus {
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

    pub fn read_word_bugged_boundary_mut(bus: &mut Bus, addr: u16) -> u16 {
        let lsb = bus.read_byte_mut(addr as u16);
        // Replicate the bug on the 6502 where the high byte would not be
        // incremented when there was a page boundary
        let high_addr = if addr & 0xFF == 0xFF {
            addr & 0xFF00
        } else {
            addr.wrapping_add(1)
        };
        let msb = bus.read_byte_mut(high_addr);
        (msb as u16) << 8 | (lsb as u16)
    }
}

pub struct PlaceholderBus {
    name: String,
    unhandled_read: HashSet<u16>,
    unhandled_write: HashSet<u16>,
}

impl PlaceholderBus {
    pub fn new(name: String) -> PlaceholderBus {
        PlaceholderBus {
            name: name,
            unhandled_read: HashSet::new(),
            unhandled_write: HashSet::new(),
        }
    }
}

impl Bus for PlaceholderBus {
    fn read_byte(&self, _addr: u16) -> u8 {
        0
    }

    fn read_byte_mut(&mut self, addr: u16) -> u8 {
        if !self.unhandled_read.contains(&addr) {
            println!("WARN: Read byte from placeholder {} bus at {:04X}", self.name, addr);
            self.unhandled_read.insert(addr);
        }
        0
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        if !self.unhandled_write.contains(&addr) {
            println!("WARN: {:02X} written to placeholder {} bus at {:04X}", val, self.name, addr);
            self.unhandled_write.insert(addr);
        }
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

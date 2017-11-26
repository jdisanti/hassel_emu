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

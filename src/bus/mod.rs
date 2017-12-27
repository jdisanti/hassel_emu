use cpu::Cpu;

mod graphics_bus;
mod peripheral_bus;

pub use self::graphics_bus::GraphicsBus;
pub use self::peripheral_bus::PeripheralBus;

macro_rules! read_word {
    ($bus:ident, $addr:expr) => {
        {
            let lsb = $bus.read_byte($addr as u16);
            let msb = $bus.read_byte($addr.wrapping_add(1) as u16);
            (msb as u16) << 8 | (lsb as u16)
        }
    }
}

pub trait BusDebugView {
    fn read_byte(&self, addr: u16) -> u8;

    fn read_word(&self, addr: u16) -> u16 {
        read_word!(self, addr)
    }

    fn read_word_zero_page(&self, addr: u8) -> u16 {
        read_word!(self, addr)
    }
}

pub trait Bus {
    fn debug_view(&self) -> &BusDebugView;

    // There's a mutable read for peripherals because
    // reading their registers can cause a state change
    // as an expected part of how the hardware behaves
    fn read_byte(&mut self, addr: u16) -> u8;
    fn write_byte(&mut self, addr: u16, val: u8);
    fn step(&mut self, cpu: &mut Cpu);

    fn read_word(&mut self, addr: u16) -> u16 {
        read_word!(self, addr)
    }

    fn read_word_zero_page(&mut self, addr: u8) -> u16 {
        read_word!(self, addr)
    }
}

pub struct NullBusDebugView {
}

impl NullBusDebugView {
    pub fn new() -> NullBusDebugView {
        NullBusDebugView { }
    }
}

impl BusDebugView for NullBusDebugView {
    fn read_byte(&self, _addr: u16) -> u8 {
        0
    }
}

pub struct PlaceholderBus {
    debug_view: NullBusDebugView,
    name: String,
}

impl PlaceholderBus {
    pub fn new(name: String) -> PlaceholderBus {
        PlaceholderBus {
            debug_view: NullBusDebugView::new(),
            name: name,
        }
    }
}

impl Bus for PlaceholderBus {
    fn debug_view(&self) -> &BusDebugView {
        &self.debug_view
    }

    fn read_byte(&mut self, addr: u16) -> u8 {
        println!("WARN: Read byte from placeholder {} bus at {:04X}", self.name, addr);
        0
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        println!("WARN: {:02X} written to placeholder {} bus at {:04X}", val, self.name, addr);
    }

    fn step(&mut self, _cpu: &mut Cpu) {
    }
}

#[cfg(test)]
pub struct TestBus {
    debug_view: NullBusDebugView,
    mem: Vec<u8>,
}

#[cfg(test)]
impl TestBus {
    pub fn new() -> TestBus {
        TestBus {
            debug_view: NullBusDebugView::new(),
            mem: Vec::new(),
        }
    }
}

#[cfg(test)]
impl Bus for TestBus {
    fn debug_view(&self) -> &BusDebugView {
        &self.debug_view
    }

    fn read_byte(&mut self, addr: u16) -> u8 {
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

    fn step(&mut self, _cpu: &mut Cpu) {
    }
}
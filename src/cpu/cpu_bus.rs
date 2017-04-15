use bus::Bus;

use std::cell::RefCell;
use std::rc::Rc;

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
    ram: [u8; 2048],
    prg: Vec<u8>,
    apu: Rc<RefCell<Bus>>,
    ppu: Rc<RefCell<Bus>>,
    input: Rc<RefCell<Bus>>,
    debugger: RefCell<CpuBusDebugger>,
}

impl CpuBus {
    pub fn new(prg: &[u8],
               apu: Rc<RefCell<Bus>>,
               ppu: Rc<RefCell<Bus>>,
               input: Rc<RefCell<Bus>>) -> CpuBus {
        CpuBus {
            ram: [0u8; 2048],
            prg: Vec::from(prg),
            apu: apu,
            ppu: ppu,
            input: input,
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
        // Memory map: http://wiki.nesdev.com/w/index.php/CPU_memory_map
        match addr {
            0x0000...0x07FF => self.ram[addr],
            0x0800...0x0FFF => self.ram[addr - 0x0800],
            0x1000...0x17FF => self.ram[addr - 0x1000],
            0x1800...0x1FFF => self.ram[addr - 0x1800],
            0x2000...0x3FFF => self.ppu.borrow().read_byte(addr as u16),
            0x4000...0x4013 => self.apu.borrow().read_byte(addr as u16),
            0x4014 => self.ppu.borrow().read_byte(addr as u16),
            0x4015 => self.apu.borrow().read_byte(addr as u16),
            0x4016...0x4017 => self.input.borrow().read_byte(addr as u16),
            0x4018...0x401F => self.apu.borrow().read_byte(addr as u16),
            0x4020...0x7FFF => { 0 }
            0x8000...0xFFFF => {
                let offset = (addr - 0x8000) % self.prg.len();
                return self.prg[offset]
            },
            _ => { 0 }
        }
    }

    fn read_byte_mut(&mut self, addr: u16) -> u8 {
        self.debugger.borrow_mut().read(addr);

        let addr: usize = addr as usize;
        // Memory map: http://wiki.nesdev.com/w/index.php/CPU_memory_map
        match addr {
            0x0000...0x07FF => self.ram[addr],
            0x0800...0x0FFF => self.ram[addr - 0x0800],
            0x1000...0x17FF => self.ram[addr - 0x1000],
            0x1800...0x1FFF => self.ram[addr - 0x1800],
            0x2000...0x3FFF => self.ppu.borrow_mut().read_byte_mut(addr as u16),
            0x4000...0x4013 => self.apu.borrow_mut().read_byte_mut(addr as u16),
            0x4014 => self.ppu.borrow_mut().read_byte_mut(addr as u16),
            0x4015 => self.apu.borrow_mut().read_byte_mut(addr as u16),
            0x4016...0x4017 => self.input.borrow_mut().read_byte_mut(addr as u16),
            0x4018...0x401F => self.apu.borrow_mut().read_byte_mut(addr as u16),
            0x4020...0x7FFF => panic!("Unhandled read cartridge location: 0x{:04X}", addr),
            0x8000...0xFFFF => {
                let offset = (addr - 0x8000) % self.prg.len();
                return self.prg[offset]
            },
            _ => panic!("Unknown read memory location: 0x{:04X}", addr)
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        self.debugger.borrow_mut().write(addr);

        let addr: usize = addr as usize;
        // Memory map: http://wiki.nesdev.com/w/index.php/CPU_memory_map
        match addr {
            0x0000...0x07FF => self.ram[addr] = val,
            0x0800...0x0FFF => self.ram[addr - 0x0800] = val,
            0x1000...0x17FF => self.ram[addr - 0x1000] = val,
            0x1800...0x1FFF => self.ram[addr - 0x1800] = val,
            0x2000...0x3FFF => self.ppu.borrow_mut().write_byte(addr as u16, val),
            0x4000...0x4013 => self.apu.borrow_mut().write_byte(addr as u16, val),
            0x4014 => self.ppu.borrow_mut().write_byte(addr as u16, val),
            0x4015 => self.apu.borrow_mut().write_byte(addr as u16, val),
            0x4016...0x4017 => self.input.borrow_mut().write_byte(addr as u16, val),
            0x4018...0x401F => self.apu.borrow_mut().write_byte(addr as u16, val),
            0x4020...0xFFF9 => panic!("Unhandled write cartridge location: 0x{:04X}", addr),
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
        let apu = Rc::new(RefCell::new(PlaceholderBus::new(String::from("APU"))));
        let ppu = Rc::new(RefCell::new(PlaceholderBus::new(String::from("PPU"))));
        let input = Rc::new(RefCell::new(PlaceholderBus::new(String::from("IO"))));
        let prg = [0u8; 8 * 1024];
        CpuBus::new(&prg, apu, ppu, input)
    }

    #[test]
    fn test_internal_ram_boundaries_and_mirroring() {
        let mut bus = fake_bus();

        assert_eq!(0, bus.read_byte_mut(0x0000));
        assert_eq!(0, bus.read_byte_mut(0x07FF));
        bus.write_byte(0x0000, 0xDC);
        bus.write_byte(0x07FF, 0xCD);

        assert_eq!(0xDC, bus.read_byte_mut(0x0000));
        assert_eq!(0xDC, bus.read_byte_mut(0x0800));
        assert_eq!(0xDC, bus.read_byte_mut(0x1000));
        assert_eq!(0xDC, bus.read_byte_mut(0x1800));

        assert_eq!(0xCD, bus.read_byte_mut(0x07FF));
        assert_eq!(0xCD, bus.read_byte_mut(0x0FFF));
        assert_eq!(0xCD, bus.read_byte_mut(0x17FF));
        assert_eq!(0xCD, bus.read_byte_mut(0x1FFF));

        bus.write_byte(0x1100, 0xAC);
        assert_eq!(0xAC, bus.read_byte_mut(0x1100));
        assert_eq!(0xAC, bus.read_byte_mut(0x0900));
        assert_eq!(0xAC, bus.read_byte_mut(0x0100));
    }

    #[test]
    fn test_read_word() {
        let mut bus = fake_bus();
        bus.write_byte(0x500, 0x01);
        bus.write_byte(0x501, 0x02);
        assert_eq!(0x0201, Bus::read_word_mut(&mut bus, 0x500));
    }
}

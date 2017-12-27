use std::rc::Rc;
use std::cell::RefCell;

use cpu::Cpu;
use bus::Bus;

pub const REQUIRED_ROM_SIZE: usize = 0x2000;

pub struct Emulator {
    cpu: Cpu,
    peripheral_bus: Rc<RefCell<Bus>>,
    last_pc: u16,
}

impl Emulator {
    pub fn new(rom: Vec<u8>, peripheral_bus: Rc<RefCell<Bus>>) -> Emulator {
        assert!(rom.len() == REQUIRED_ROM_SIZE);
        Emulator {
            cpu: Cpu::new(rom, Rc::clone(&peripheral_bus)),
            peripheral_bus: peripheral_bus,
            last_pc: 0,
        }
    }

    pub fn cpu(&self) -> &Cpu {
        &self.cpu
    }

    pub fn is_halted(&self) -> bool {
        self.last_pc == self.cpu.reg_pc()
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.last_pc = 0;
    }

    pub fn step(&mut self) -> usize {
        self.last_pc = self.cpu.reg_pc();
        let cycles = self.cpu.next_instruction();
        self.peripheral_bus.borrow_mut().step(&mut self.cpu);
        cycles
    }
}
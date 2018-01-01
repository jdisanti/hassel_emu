//
// Copyright 2017 hassel_emu Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use cpu::Cpu;
use cpu::memory::MemoryMap;

pub struct Emulator {
    cpu: Cpu,
    last_pc: u16,
}

impl Emulator {
    pub fn new(memory_map: MemoryMap) -> Emulator {
        Emulator {
            cpu: Cpu::new(memory_map),
            last_pc: 0,
        }
    }

    pub fn cpu(&self) -> &Cpu {
        &self.cpu
    }

    pub fn is_halted(&self) -> bool {
        self.last_pc == self.cpu.registers().pc
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.last_pc = 0;
    }

    pub fn step(&mut self) -> usize {
        self.last_pc = self.cpu.registers().pc;
        let cycles = self.cpu.step();
        cycles
    }
}

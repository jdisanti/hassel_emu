//
// Copyright 2017 hassel_lib6502 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use cpu::memory::{Interrupt, MemoryMap};
use cpu::registers::Registers;
use cpu::instruction::Executor;
use cpu::instruction::InstructionResult;

const NMI_VECTOR: u16 = 0xFFFA;
const RESET_VECTOR: u16 = 0xFFFC;
const IRQ_VECTOR: u16 = 0xFFFE;

const STACK_ADDR: u16 = 0x0100;

pub struct Cpu {
    registers: Registers,
    memory: MemoryMap,
    cycle: usize,
    executor: Executor,
}

impl Cpu {
    pub fn new(memory: MemoryMap) -> Cpu {
        let mut cpu = Cpu {
            registers: Registers::new(),
            memory: memory,
            cycle: 0,
            executor: Executor::new(),
        };

        cpu.reset();
        cpu
    }

    pub fn reset(&mut self) {
        let entry_point = self.memory.read().word(RESET_VECTOR);
        self.registers.pc = entry_point;
        self.registers.status.set_interrupt_inhibit(true);
    }

    pub fn registers(&self) -> &Registers {
        &self.registers
    }

    pub fn memory(&self) -> &MemoryMap {
        &self.memory
    }

    pub fn memory_mut(&mut self) -> &mut MemoryMap {
        &mut self.memory
    }

    pub fn request_interrupt(&mut self) -> bool {
        if !self.registers.status.interrupt_inhibit() {
            let interrupt_addr = self.memory.read().word(IRQ_VECTOR);
            self.interrupt(interrupt_addr);
            true
        } else {
            false
        }
    }

    pub fn request_non_maskable_interrupt(&mut self) {
        let nmi_addr = self.memory.read().word(NMI_VECTOR);
        self.interrupt(nmi_addr);
    }

    pub fn step(&mut self) -> usize {
        let mut result = InstructionResult::new();
        result = self.executor
            .execute_instruction(&self.registers, &mut self.memory, result);

        for write in &result.writes {
            self.memory.write().byte(write.address, write.value);
        }

        self.registers = result.reg;
        self.cycle += result.cycles;

        match self.memory.step() {
            Some(Interrupt::Interrupt) => {
                self.request_interrupt();
            }
            Some(Interrupt::NonMaskableInterrupt) => {
                self.request_non_maskable_interrupt();
            }
            _ => {}
        }

        result.cycles
    }

    #[inline]
    fn push(&mut self, registers: &mut Registers, val: u8) {
        self.memory
            .write()
            .byte(STACK_ADDR + registers.sp as u16, val);
        registers.sp = registers.sp.wrapping_sub(1);
    }

    fn interrupt(&mut self, handler_address: u16) {
        let mut registers = self.registers;
        let cur_pc = registers.pc;
        let cur_status = registers.status.value();
        self.push(&mut registers, (cur_pc >> 8) as u8);
        self.push(&mut registers, (cur_pc & 0xFF) as u8);
        self.push(&mut registers, cur_status);
        registers.pc = handler_address;
        self.registers = registers;
    }
}

//
// Copyright 2017 hassel_lib6502 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use cpu::bus::Bus;
use cpu::cpu_bus::CpuBus;
use cpu::registers::Registers;
use cpu::instruction::Executor;
use cpu::instruction::InstructionResult;

use std::cell::RefCell;
use std::rc::Rc;

const NMI_VECTOR: u16 = 0xFFFA;
const RESET_VECTOR: u16 = 0xFFFC;
const IRQ_VECTOR: u16 = 0xFFFE;

const STACK_ADDR: u16 = 0x0100;

pub struct Cpu {
    registers: Registers,
    bus: CpuBus,

    cycle: usize,
    dma_buffer: Vec<u8>,

    executor: Executor,
}

impl Cpu {
    pub fn new(rom: Vec<u8>,
               peripheral_bus: Rc<RefCell<Bus>>) -> Cpu {
        let mut cpu = Cpu {
            registers: Registers::new(),
            bus: CpuBus::new(rom, peripheral_bus),
            cycle: 0,
            dma_buffer: Vec::new(),
            executor: Executor::new(),
        };

        cpu.reset();
        cpu
    }

    pub fn reset(&mut self) {
        let entry_point = self.bus.read_word(RESET_VECTOR);
        self.registers.pc = entry_point;
        self.registers.status.set_interrupt_inhibit(true);
    }

    pub fn registers(&self) -> &Registers {
        &self.registers
    }

    pub fn bus(&self) -> &Bus {
        &self.bus
    }

    pub fn request_interrupt(&mut self) -> bool {
        if !self.registers.status.interrupt_inhibit() {
            let interrupt_addr = Bus::read_word(&mut self.bus, IRQ_VECTOR);
            self.interrupt(interrupt_addr);
            true
        } else {
            false
        }
    }

    pub fn request_non_maskable_interrupt(&mut self) {
        let nmi_addr = Bus::read_word(&mut self.bus, NMI_VECTOR);
        self.interrupt(nmi_addr);
    }

    pub fn dma_slice(&mut self, dma_addr: u16, dma_size: u16) -> &[u8] {
        self.dma_buffer.clear();
        for i in 0..dma_size {
            let addr = dma_addr.wrapping_add(i);
            self.dma_buffer.push(self.bus.read_byte(addr));
        }
        &self.dma_buffer[..]
    }

    pub fn next_instruction(&mut self) -> usize {
        let mut result = InstructionResult::new();
        result = self.executor.execute_instruction(&self.registers, &mut self.bus, result);

        for write in &result.writes {
            self.bus.write_byte(write.address, write.value);
        }

        self.registers = result.reg;
        self.cycle += result.cycles;
        result.cycles
    }

    #[inline]
    fn push(&mut self, registers: &mut Registers, val: u8) {
        self.bus.write_byte(STACK_ADDR + registers.sp as u16, val);
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

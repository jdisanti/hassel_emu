//
// Copyright 2017 hassel_emu Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use std::cell::RefCell;
use std::rc::Rc;

use cpu::memory::{MemoryMap, MemoryMappedDevice, Interrupt};

use hassel::graphics_device::GraphicsDevice;
use hassel::io_device::IODevice;

pub struct Peripherals {
    pub graphics: Rc<RefCell<GraphicsDevice>>,
    pub io: Rc<RefCell<IODevice>>,
}

impl Peripherals {
    pub fn new(
        graphics: Rc<RefCell<GraphicsDevice>>,
        io: Rc<RefCell<IODevice>>
    ) -> Peripherals {
        Peripherals {
            graphics: graphics,
            io: io,
        }
    }
}

impl MemoryMappedDevice for Peripherals {
    fn read_byte(&self, _addr: u16) -> u8 {
        0
    }

    fn read_byte_mut(&mut self, addr: u16) -> u8 {
        match addr {
            0xDFFE => self.graphics.borrow_mut().read_byte_mut(addr),
            0xDFFF => self.io.borrow_mut().read_byte_mut(addr),
            _ => unreachable!()
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0xDFFE => self.graphics.borrow_mut().write_byte(addr, val),
            0xDFFF => self.io.borrow_mut().write_byte(addr, val),
            _ => unreachable!()
        }
    }

    fn requires_step(&self) -> bool {
        true
    }

    fn step(&mut self, memory: &mut MemoryMap) -> Option<Interrupt> {
        let graphics_interrupt = self.graphics.borrow_mut().step(memory);
        let io_interrupt = self.io.borrow_mut().step(memory);
        if io_interrupt.is_some() {
            io_interrupt
        } else {
            graphics_interrupt
        }
    }
}
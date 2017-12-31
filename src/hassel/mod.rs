//
// Copyright 2017 hassel_emu Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use cpu::memory::{MemoryMap, MemoryMappedDevice};

use std::cell::RefCell;
use std::rc::Rc;

mod graphics_device;
mod io_device;
mod key;
mod peripherals;

pub use self::key::Key;
pub use self::graphics_device::{GraphicsDevice, SCREEN_WIDTH_PIXELS, SCREEN_HEIGHT_PIXELS};
pub use self::io_device::IODevice;
use self::peripherals::Peripherals;

pub const REQUIRED_ROM_SIZE: usize = 0x2000;

pub struct HasselSystemBuilder {
    rom: Option<Vec<u8>>,
}

impl HasselSystemBuilder {
    pub fn new() -> HasselSystemBuilder {
        HasselSystemBuilder {
            rom: None
        }
    }

    pub fn rom(mut self, rom: Vec<u8>) -> Self {
        self.rom = Some(rom);
        self
    }

    pub fn build(self) -> (MemoryMap, Rc<RefCell<GraphicsDevice>>, Rc<RefCell<IODevice>>) {
        assert!(self.rom.is_some(), "HasselMemoryMapBuilder requires a rom");

        let graphics = Rc::new(RefCell::new(graphics_device::GraphicsDevice::new()));
        let io = Rc::new(RefCell::new(io_device::IODevice::new()));

        let peripherals: Rc<RefCell<MemoryMappedDevice>> =
            Rc::new(RefCell::new(Peripherals::new(Rc::clone(&graphics), Rc::clone(&io))));

        let memory_map = MemoryMap::builder()
            .ram(0x0000, 0xDFFD)
            .peripheral(0xDFFE, 0xDFFF, peripherals)
            .rom(0xE000, 0xFFFF, self.rom.unwrap())
            .build();
        
        (memory_map, graphics, io)
    }
}
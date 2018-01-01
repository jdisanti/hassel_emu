//
// Copyright 2017 hassel_emu Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

//! The hassel_emu crate is a generic MOS 6502 emulation library with additional (optional)
//! code provided to emulate the hardware specifics of the homebrew Hasseldorf Computer.
//!
//! If you just want to test some 6502 code, it's trivial to setup a system that has no
//! IO devices to run your code and check the result:
//!
//! ```rust
//! # extern crate hassel_emu;
//! # use hassel_emu::*;
//! # fn main() {
//! // This is a simple ROM that has two instructions:
//! //   LDA #3
//! //   STA $00
//! let rom: Vec<u8> = vec![
//!     0xA9, 0x03, 0x85, 0x00, 0x4C, 0xF6, 0xFF,
//!     0x00, 0xF6, 0xFF, 0xF2, 0xFF, 0xF6, 0xFF
//! ];
//!
//! // Here, we define our architecture
//! // We can add IO peripherals also, but we're not doing that in this example
//! let memory_map = MemoryMap::builder()
//!     .ram(0x0000, 0xFFF1)
//!     .rom(0xFFF2, 0xFFFF, rom)
//!     .build();
//!
//! // Then create our emulator and step it twice
//! let mut cpu = Cpu::new(memory_map);
//! cpu.step();
//! cpu.step();
//!
//! // We should see 3 at address 0x0000
//! assert_eq!(3u8, cpu.memory().debug_read().byte(0x0000));
//! # }
//! ```
//!
//! To create your own memory-mapped hardware peripheral, you just need to
//! implement the MemoryMappedDevice trait on a struct, and then add it to
//! the memory map using the MemoryMapBuilder.

extern crate hassel_lib6502;

pub mod emulator;

#[cfg(feature = "hassel_arch")]
#[macro_use]
extern crate enum_primitive;

#[cfg(feature = "hassel_arch")]
pub mod hassel;

pub use emulator::Cpu;
pub use emulator::{MemoryMap, MemoryMappedDevice};

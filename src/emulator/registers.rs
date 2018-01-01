//
// Copyright 2017 hassel_lib6502 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use emulator::register_status::RegisterStatus;

const REG_SP_INIT: u8 = 0xFF;

/// Holds all of the 6502 register values
#[derive(Copy, Clone)]
pub struct Registers {
    /// Accumulator
    pub a: u8,

    /// X Register
    pub x: u8,

    /// Y Register
    pub y: u8,

    /// Program counter
    pub pc: u16,

    /// Stack pointer
    pub sp: u8,

    /// Status register
    pub status: RegisterStatus,
}

impl Registers {
    /// Creates a new register set
    pub fn new() -> Registers {
        Registers {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: REG_SP_INIT,
            status: RegisterStatus::new(),
        }
    }

    /// Sets the accumulator and also updates the status flag based on its value
    #[inline]
    pub fn set_reg_a(&mut self, val: u8) {
        self.a = val;
        self.status.set_nz_from(val);
    }

    /// Sets the X register and also updates the status flag based on its value
    #[inline]
    pub fn set_reg_x(&mut self, val: u8) {
        self.x = val;
        self.status.set_nz_from(val);
    }

    /// Sets the Y register and also updates the status flag based on its value
    #[inline]
    pub fn set_reg_y(&mut self, val: u8) {
        self.y = val;
        self.status.set_nz_from(val);
    }
}

//
// Copyright 2017 hassel_lib6502 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use cpu::register_status::RegisterStatus;

const REG_SP_INIT: u8 = 0xFF;

#[derive(Copy, Clone)]
pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub sp: u8,
    pub status: RegisterStatus,
}

impl Registers {
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

    #[inline]
    pub fn set_reg_a(&mut self, val: u8) {
        self.a = val;
        self.status.set_nz_from(val);
    }

    #[inline]
    pub fn set_reg_x(&mut self, val: u8) {
        self.x = val;
        self.status.set_nz_from(val);
    }

    #[inline]
    pub fn set_reg_y(&mut self, val: u8) {
        self.y = val;
        self.status.set_nz_from(val);
    }
}

//
// Copyright 2017 hassel_lib6502 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

const MASK_NEGATIVE: u8 = 0x80;
const MASK_OVERFLOW: u8 = 0x40;
const MASK_BIT_5: u8 = 0x20;
const MASK_BRK: u8 = 0x10;
const MASK_DECIMAL: u8 = 0x08;
const MASK_INTERRUPT_INHIBIT: u8 = 0x04;
const MASK_ZERO: u8 = 0x02;
const MASK_CARRY: u8 = 0x01;

/// Abstract struct version of the status register value
#[derive(Copy, Clone)]
pub struct RegisterStatus {
    value: u8,
}

impl RegisterStatus {
    /// Creates a new default status register value
    pub fn new() -> RegisterStatus {
        RegisterStatus {
            // bit 5 is always set
            value: 0x20,
        }
    }

    #[inline(always)]
    fn bit(&self, bit_mask: u8) -> bool {
        self.value & bit_mask != 0
    }
    #[inline(always)]
    fn set_bit(&mut self, bit_mask: u8, value: bool) {
        self.value = self.value & !bit_mask;
        if value {
            self.value = self.value | bit_mask;
        }
    }

    /// Returns the negative flag
    #[inline(always)]
    pub fn negative(&self) -> bool {
        self.bit(MASK_NEGATIVE)
    }
    /// Sets the negative flag
    #[inline(always)]
    pub fn set_negative(&mut self, val: bool) {
        self.set_bit(MASK_NEGATIVE, val);
    }

    /// Returns the overflow fag
    #[inline(always)]
    pub fn overflow(&self) -> bool {
        self.bit(MASK_OVERFLOW)
    }
    /// Sets the overflow fag
    #[inline(always)]
    pub fn set_overflow(&mut self, val: bool) {
        self.set_bit(MASK_OVERFLOW, val);
    }

    /// Returns the interrupt (BRK) flag
    #[inline(always)]
    pub fn brk(&self) -> bool {
        self.bit(MASK_BRK)
    }
    /// Sets the interrupt (BRK) flag
    #[inline(always)]
    pub fn set_brk(&mut self, val: bool) {
        self.set_bit(MASK_BRK, val);
    }

    /// Returns the decimal flag
    #[inline(always)]
    pub fn decimal(&self) -> bool {
        self.bit(MASK_DECIMAL)
    }
    /// Sets the decimal flag
    #[inline(always)]
    pub fn set_decimal(&mut self, val: bool) {
        self.set_bit(MASK_DECIMAL, val);
    }

    /// Returns the inhibit interrupt flag
    #[inline(always)]
    pub fn interrupt_inhibit(&self) -> bool {
        self.bit(MASK_INTERRUPT_INHIBIT)
    }
    /// Sets the inhibit interrupt flag
    #[inline(always)]
    pub fn set_interrupt_inhibit(&mut self, val: bool) {
        self.set_bit(MASK_INTERRUPT_INHIBIT, val);
    }

    /// Returns the zero flag
    #[inline(always)]
    pub fn zero(&self) -> bool {
        self.bit(MASK_ZERO)
    }
    /// Sets the zero flag
    #[inline(always)]
    pub fn set_zero(&mut self, val: bool) {
        self.set_bit(MASK_ZERO, val);
    }

    /// Returns the carry flag
    #[inline(always)]
    pub fn carry(&self) -> bool {
        self.bit(MASK_CARRY)
    }
    /// Sets the carry flag
    #[inline(always)]
    pub fn set_carry(&mut self, val: bool) {
        self.set_bit(MASK_CARRY, val);
    }

    /// Sets the negative and zero flags based on the passed in value
    #[inline(always)]
    pub fn set_nz_from(&mut self, val: u8) {
        self.set_negative(val & 0x80 > 0);
        self.set_zero(val == 0);
    }

    /// Returns the byte value of the status register
    pub fn value(&self) -> u8 {
        self.value
    }

    /// Sets the status register from a byte value
    pub fn set_value(&mut self, value: u8) {
        // The BRK flag can't be set, and bit 5 must always be on
        let mut value = value & !MASK_BRK | MASK_BIT_5;
        if self.brk() {
            value = value | MASK_BRK;
        }
        self.value = value;
    }
}

#[cfg(test)]
#[test]
fn test_set_value() {
    let mut reg = RegisterStatus::new();
    assert_eq!(MASK_BIT_5, reg.value());

    reg.set_value(0);
    assert_eq!(MASK_BIT_5, reg.value());

    reg.set_value(MASK_BRK);
    assert_eq!(MASK_BIT_5, reg.value());

    reg.set_brk(true);
    reg.set_value(0);
    assert_eq!(MASK_BRK | MASK_BIT_5, reg.value());
}

#[cfg(test)]
#[test]
fn test_decimal() {
    let mut reg = RegisterStatus::new();
    assert_eq!(MASK_BIT_5, reg.value());

    reg.set_decimal(true);
    assert!(reg.decimal());

    assert_eq!(MASK_DECIMAL | MASK_BIT_5, reg.value());

    reg.set_value(0xFF);
    reg.set_brk(true);
    reg.set_decimal(false);
    assert!(!reg.decimal());
    assert_eq!(!MASK_DECIMAL, reg.value());
}

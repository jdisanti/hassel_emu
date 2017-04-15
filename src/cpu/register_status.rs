const MASK_NEGATIVE: u8 = 0x80;
const MASK_OVERFLOW: u8 = 0x40;
const MASK_BIT_5: u8 = 0x20;
const MASK_BRK: u8 = 0x10;
const MASK_DECIMAL: u8 = 0x08;
const MASK_INTERRUPT_INHIBIT: u8 = 0x04;
const MASK_ZERO: u8 = 0x02;
const MASK_CARRY: u8 = 0x01;

#[derive(Copy, Clone)]
pub struct RegisterStatus {
    value: u8,
}

impl RegisterStatus {
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

    #[inline(always)]
    pub fn negative(&self) -> bool { self.bit(MASK_NEGATIVE) }
    #[inline(always)]
    pub fn set_negative(&mut self, val: bool) { self.set_bit(MASK_NEGATIVE, val); }

    #[inline(always)]
    pub fn overflow(&self) -> bool { self.bit(MASK_OVERFLOW) }
    #[inline(always)]
    pub fn set_overflow(&mut self, val: bool) { self.set_bit(MASK_OVERFLOW, val); }

    #[inline(always)]
    pub fn brk(&self) -> bool { self.bit(MASK_BRK) }
    #[inline(always)]
    pub fn set_brk(&mut self, val: bool) { self.set_bit(MASK_BRK, val); }

    #[inline(always)]
    pub fn decimal(&self) -> bool { self.bit(MASK_DECIMAL) }
    #[inline(always)]
    pub fn set_decimal(&mut self, val: bool) { self.set_bit(MASK_DECIMAL, val); }

    #[inline(always)]
    pub fn interrupt_inhibit(&self) -> bool { self.bit(MASK_INTERRUPT_INHIBIT) }
    #[inline(always)]
    pub fn set_interrupt_inhibit(&mut self, val: bool) { self.set_bit(MASK_INTERRUPT_INHIBIT, val); }

    #[inline(always)]
    pub fn zero(&self) -> bool { self.bit(MASK_ZERO) }
    #[inline(always)]
    pub fn set_zero(&mut self, val: bool) { self.set_bit(MASK_ZERO, val); }

    #[inline(always)]
    pub fn carry(&self) -> bool { self.bit(MASK_CARRY) }
    #[inline(always)]
    pub fn set_carry(&mut self, val: bool) { self.set_bit(MASK_CARRY, val); }

    // Sets the negative and zero flags based on the passed in value
    #[inline(always)]
    pub fn set_nz_from(&mut self, val: u8) {
        self.set_negative(val & 0x80 > 0);
        self.set_zero(val == 0);
    }

    pub fn value(&self) -> u8 {
        self.value
    }

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

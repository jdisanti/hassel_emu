//
// Copyright 2017 hassel_emu Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

extern crate hassel_lib6502;

pub mod cpu;
pub mod emulator;

#[cfg(feature = "hassel_arch")]
#[macro_use]
extern crate enum_primitive;

#[cfg(feature = "hassel_arch")]
pub mod hassel;

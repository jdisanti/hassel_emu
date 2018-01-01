//
// Copyright 2017 hassel_emu Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

#[macro_use]
mod common;
mod executor;

mod arithmetic;
mod bitwise;
mod branch;
mod compare;
mod flag;
mod interrupt;
mod load;
mod nop;
mod stack;
mod store;
mod transfer;

pub use emulator::instruction::executor::Executor;
pub use emulator::instruction::executor::InstructionResult;
pub use emulator::instruction::executor::Write;

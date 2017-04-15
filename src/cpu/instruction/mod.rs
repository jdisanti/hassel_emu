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

pub use cpu::instruction::executor::Executor;
pub use cpu::instruction::executor::InstructionResult;
pub use cpu::instruction::executor::Write;

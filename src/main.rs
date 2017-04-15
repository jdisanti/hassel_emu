mod cpu;
mod bus;

use cpu::Cpu;
use std::env;

pub struct Emulator {
    cpu: Box<Cpu>,
}

impl Emulator {
    pub fn new(rom_path: &str) -> Emulator {
        unimplemented!()
    }

    pub fn step(&mut self) {
        println!("{}", self.cpu.debug_next_instruction());
        self.cpu.next_instruction();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: emulator rom-file");
        return
    }

    let rom_path = &args[1];
    println!("Loading rom named {}...", rom_path);
    let emulator = Emulator::new(rom_path);
}

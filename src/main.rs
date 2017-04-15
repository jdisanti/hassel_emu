mod cpu;
mod bus;

use bus::PlaceholderBus;
use cpu::Cpu;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use std::thread;
use std::time::Duration;

pub const ROM_SIZE: usize = 32752;

pub struct Emulator {
    cpu: Box<Cpu>,
}

impl Emulator {
    pub fn new(rom_path: &str) -> Result<Emulator, String> {
        let mut rom_file = File::open(rom_path)
            .map_err(|e| format!("Failed to load ROM: {}", rom_path))?;

        let mut rom = Vec::new();
        rom_file.read_to_end(&mut rom)
            .map_err(|e| format!("Failed to read ROM: {}", rom_path))?;

        if rom.len() != ROM_SIZE {
            return Err(format!("ROM has unexpected size ({}); should be {} bytes.", rom.len(), ROM_SIZE))
        }

        let peripheral_bus = Rc::new(RefCell::new(PlaceholderBus::new(String::from("Peripherals"))));
        Ok(Emulator {
            cpu: Box::new(Cpu::new(rom, peripheral_bus)),
        })
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn step(&mut self) {
        self.cpu.next_instruction();
        println!("{}", self.cpu.debug_next_instruction());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: emulator rom-file");
        return
    }

    let rom_path = &args[1];
    println!("Loading rom named \"{}\"...", rom_path);

    let mut emulator = match Emulator::new(rom_path) {
        Ok(emulator) => emulator,
        Err(err) => {
            println!("{}", err);
            return
        }
    };

    emulator.reset();
    loop {
        emulator.step();
        thread::sleep(Duration::from_millis(200u64));
    }
}

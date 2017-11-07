extern crate minifb;

mod cpu;
mod bus;
mod graphics;

use bus::Bus;
use cpu::Cpu;
use graphics::GraphicsBus;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use std::thread;
use std::time::Duration;

pub const ROM_SIZE: usize = 0x2000;

pub struct PeripheralBus {
    pub graphics_bus: Box<GraphicsBus>,
}

impl Bus for PeripheralBus {
    fn read_byte(&self, addr: u16) -> u8 {
        if addr == 0xDFFE {
            self.graphics_bus.read_byte(addr)
        } else {
            // TODO: Non-graphics peripherals
            0
        }
    }

    fn read_byte_mut(&mut self, addr: u16) -> u8 {
        if addr == 0xDFFE {
            self.graphics_bus.read_byte_mut(addr)
        } else {
            // TODO: Non-graphics peripherals
            0
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        if addr == 0xDFFE {
            self.graphics_bus.write_byte(addr, val);
        }
        // TODO: Non-graphics peripherals
    }
}

pub struct Emulator {
    cpu: Box<Cpu>,
    peripheral_bus: Rc<RefCell<PeripheralBus>>,
    last_pc: u16,
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

        let graphics_bus = Box::new(GraphicsBus::new());
        let peripheral_bus = Rc::new(RefCell::new(PeripheralBus {
            graphics_bus: graphics_bus,
        }));

        Ok(Emulator {
            cpu: Box::new(Cpu::new(rom, peripheral_bus.clone())),
            peripheral_bus: peripheral_bus,
            last_pc: 0,
        })
    }

    pub fn is_good(&self) -> bool {
        self.peripheral_bus.borrow().graphics_bus.is_good()
    }

    pub fn is_halted(&self) -> bool {
        self.last_pc == self.cpu.reg_pc()
    }

    pub fn draw(&mut self) {
        self.peripheral_bus.borrow_mut().graphics_bus.draw();
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn step(&mut self) -> usize {
        //println!("{}", self.cpu.debug_next_instruction());
        self.last_pc = self.cpu.reg_pc();
        let cycles = self.cpu.next_instruction();
        self.peripheral_bus.borrow_mut().graphics_bus.execute_peripheral_operations(&mut *self.cpu);
        cycles
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

    let start_time = Instant::now();
    let mut total_cycles: usize = 0;
    let mut last_render = Instant::now();
    let mut last_instruction = Instant::now();
    while emulator.is_good() && !emulator.is_halted() {
        let time_last_render = Instant::now().duration_since(last_render);
        if time_last_render.subsec_nanos() > 13_000_000u32 {
            emulator.draw();
            last_render = Instant::now();
        }

        let cycles = emulator.step() as u32;
        total_cycles += cycles as usize;

        // Slow down so that we're running at approximately 6 MHz
        loop {
            let time_last_instruction = Instant::now().duration_since(last_instruction);
            if time_last_instruction.subsec_nanos() > cycles * 167u32 {
                last_instruction = Instant::now();
                break;
            }
        }
    }

    let end_time = Instant::now().duration_since(start_time);

    println!("Halted after {} cycles. Took {} seconds. Leaving screen open to view results.",
        total_cycles, end_time.as_secs() as f64 + end_time.subsec_nanos() as f64 / 1_000_000_000f64);
    while emulator.is_good() {
        emulator.draw();
        thread::sleep(Duration::from_millis(10u64));
    }
}

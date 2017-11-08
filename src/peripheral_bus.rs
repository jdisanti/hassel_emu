use minifb::{Window, WindowOptions};

use bus::Bus;
use cpu::Cpu;
use graphics_bus::{self, GraphicsBus};

pub struct PeripheralBus {
    graphics_bus: GraphicsBus,
    window: Window,
}

impl PeripheralBus {
    pub fn new() -> PeripheralBus {
        let window = Window::new("Hasseldorf Emulator",
                graphics_bus::SCREEN_WIDTH_PIXELS,
                graphics_bus::SCREEN_HEIGHT_PIXELS,
                WindowOptions::default()).unwrap_or_else(|e| {
            panic!("Failed to create a window: {}", e);
        });

        PeripheralBus {
            graphics_bus: GraphicsBus::new(),
            window: window,
        }
    }

    pub fn is_good(&self) -> bool {
        self.window.is_open()
    }

    pub fn draw(&mut self) {
        self.graphics_bus.draw(&mut self.window);
    }

    pub fn execute_peripheral_operations(&mut self, cpu: &mut Cpu) {
        self.graphics_bus.execute_peripheral_operations(cpu);
    }
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
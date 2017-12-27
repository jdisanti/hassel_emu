use std::cell::RefCell;
use std::rc::Rc;

use cpu::Cpu;
use super::{Bus, BusDebugView, NullBusDebugView};
use super::graphics_bus::GraphicsBus;

const GRAPHICS_REGISTER_ADDRESS: u16 = 0xDFFE;
const IO_REGISTER_ADDRESS: u16 = 0xDFFF;

pub struct PeripheralBus {
    debug_view: NullBusDebugView,
    graphics_bus: Rc<RefCell<GraphicsBus>>,
}

impl PeripheralBus {
    pub fn new(graphics_bus: Rc<RefCell<GraphicsBus>>) -> PeripheralBus {
        PeripheralBus {
            debug_view: NullBusDebugView::new(),
            graphics_bus: graphics_bus
        }
    }
}

impl Bus for PeripheralBus {
    fn debug_view(&self) -> &BusDebugView {
        &self.debug_view
    }

    fn read_byte(&mut self, addr: u16) -> u8 {
        if addr == GRAPHICS_REGISTER_ADDRESS {
            self.graphics_bus.borrow_mut().read_byte(addr)
        } else if addr == IO_REGISTER_ADDRESS {
            // TODO
            0
        } else {
            println!("WARN: PeripheralBus called with non-peripheral address 0x{:04X}", addr);
            0
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        if addr == GRAPHICS_REGISTER_ADDRESS {
            self.graphics_bus.borrow_mut().write_byte(addr, val);
        } else if addr == IO_REGISTER_ADDRESS {
            // TODO
        } else {
            println!("WARN: PeripheralBus called with non-peripheral address 0x{:04X}", addr);
        }
    }

    fn step(&mut self, cpu: &mut Cpu) {
        self.graphics_bus.borrow_mut().step(cpu);
    }
}
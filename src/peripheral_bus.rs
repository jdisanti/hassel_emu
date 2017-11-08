use minifb::{Key, Window, WindowOptions};

use bus::Bus;
use cpu::Cpu;
use graphics_bus::{self, GraphicsBus};

const GRAPHICS_REGISTER_ADDRESS: u16 = 0xDFFE;
const IO_REGISTER_ADDRESS: u16 = 0xDFFF;

const KEY_DOWN_INTERRUPT: u8 = 0x01;
const KEY_UP_INTERRUPT: u8 = 0x02;

const MAX_RESPONSE_QUEUE_SIZE: usize = 32;

pub struct PeripheralBus {
    graphics_bus: GraphicsBus,
    window: Window,
    previous_keys: Vec<Key>,
    response_queue: Vec<u8>,
    last_interrupt_size: usize,
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
            previous_keys: Vec::new(),
            response_queue: Vec::new(),
            last_interrupt_size: 0,
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

        if let Some(keys_down) = self.window.get_keys() {
            for key in &keys_down {
                if !self.previous_keys.contains(&key) {
                    PeripheralBus::push_response(&mut self.response_queue, &[KEY_DOWN_INTERRUPT, key_to_keycode(key)]);
                }
            }
            for key in &self.previous_keys {
                if !keys_down.contains(&key) {
                    PeripheralBus::push_response(&mut self.response_queue, &[KEY_UP_INTERRUPT, key_to_keycode(key)]);
                }
            }
            self.previous_keys = keys_down;
        }

        if !self.response_queue.is_empty() && self.last_interrupt_size != self.response_queue.len() {
            cpu.interrupt_request();
            self.last_interrupt_size = self.response_queue.len();
        }
    }

    fn push_response(response_queue: &mut Vec<u8>, values: &[u8]) {
        // If our queue is full, we will start dropping responses
        if response_queue.len() + values.len() < MAX_RESPONSE_QUEUE_SIZE {
            response_queue.extend(values);
        }
    }
}

impl Bus for PeripheralBus {
    fn read_byte(&self, _addr: u16) -> u8 {
        0
    }

    fn read_byte_mut(&mut self, addr: u16) -> u8 {
        if addr == GRAPHICS_REGISTER_ADDRESS {
            self.graphics_bus.read_byte_mut(addr)
        } else if addr == IO_REGISTER_ADDRESS {
            if self.response_queue.is_empty() {
                0
            } else {
                // Avoid re-interrupting
                self.last_interrupt_size = self.response_queue.len() - 1;
                self.response_queue.remove(0)
            }
        } else {
            println!("WARN: PeripheralBus called with non-peripheral address 0x{:04X}", addr);
            0
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        if addr == GRAPHICS_REGISTER_ADDRESS {
            self.graphics_bus.write_byte(addr, val);
        } else if addr == IO_REGISTER_ADDRESS {
            // TODO: IO peripherals
        } else {
            println!("WARN: PeripheralBus called with non-peripheral address 0x{:04X}", addr);
        }
    }
}

fn key_to_keycode(key: &Key) -> u8 {
    match *key {
        Key::Key0 => '0' as u8,
        Key::Key1 => '1' as u8,
        Key::Key2 => '2' as u8,
        Key::Key3 => '3' as u8,
        Key::Key4 => '4' as u8,
        Key::Key5 => '5' as u8,
        Key::Key6 => '6' as u8,
        Key::Key7 => '7' as u8,
        Key::Key8 => '8' as u8,
        Key::Key9 => '9' as u8,

        Key::A => 'A' as u8,
        Key::B => 'B' as u8,
        Key::C => 'C' as u8,
        Key::D => 'D' as u8,
        Key::E => 'E' as u8,
        Key::F => 'F' as u8,
        Key::G => 'G' as u8,
        Key::H => 'H' as u8,
        Key::I => 'I' as u8,
        Key::J => 'J' as u8,
        Key::K => 'K' as u8,
        Key::L => 'L' as u8,
        Key::M => 'M' as u8,
        Key::N => 'N' as u8,
        Key::O => 'O' as u8,
        Key::P => 'P' as u8,
        Key::Q => 'Q' as u8,
        Key::R => 'R' as u8,
        Key::S => 'S' as u8,
        Key::T => 'T' as u8,
        Key::U => 'U' as u8,
        Key::V => 'V' as u8,
        Key::W => 'W' as u8,
        Key::X => 'X' as u8,
        Key::Y => 'Y' as u8,
        Key::Z => 'Z' as u8,

        Key::Space => ' ' as u8,
        Key::Tab => '\t' as u8,

        Key::Backslash => '\\' as u8,
        Key::Comma => ',' as u8,
        Key::Equal => '=' as u8,
        Key::LeftBracket => '[' as u8,
        Key::Minus => '-' as u8,
        Key::Period => '.' as u8,
        Key::RightBracket => ']' as u8,
        Key::Semicolon => ';' as u8,

        Key::Slash => '/' as u8,
        Key::Enter => '\n' as u8,

        Key::Backspace => 128,
        Key::Delete => 129,
        Key::End => 130,

        Key::F1 => 131,
        Key::F2 => 132,
        Key::F3 => 133,
        Key::F4 => 134,
        Key::F5 => 135,
        Key::F6 => 136,
        Key::F7 => 137,
        Key::F8 => 138,
        Key::F9 => 139,
        Key::F10 => 140,
        Key::F11 => 141,
        Key::F12 => 142,
        Key::F13 => 143,
        Key::F14 => 144,
        Key::F15 => 145,

        Key::Down => 146,
        Key::Left => 147,
        Key::Right => 148,
        Key::Up => 149,
        Key::Apostrophe => 150,
        Key::Backquote => 151,

        Key::Escape => 152,

        Key::Home => 153,
        Key::Insert => 154,
        Key::Menu => 155,

        Key::PageDown => 156,
        Key::PageUp => 157,

        Key::Pause => 158,
        Key::NumLock => 159,
        Key::CapsLock => 160,
        Key::ScrollLock => 161,
        Key::LeftShift => 162,
        Key::RightShift => 163,
        Key::LeftCtrl => 164,
        Key::RightCtrl => 165,

        Key::NumPad0 => 166,
        Key::NumPad1 => 167,
        Key::NumPad2 => 168,
        Key::NumPad3 => 169,
        Key::NumPad4 => 170,
        Key::NumPad5 => 171,
        Key::NumPad6 => 172,
        Key::NumPad7 => 173,
        Key::NumPad8 => 174,
        Key::NumPad9 => 175,
        Key::NumPadDot => 176,
        Key::NumPadSlash => 177,
        Key::NumPadAsterisk => 178,
        Key::NumPadMinus => 179,
        Key::NumPadPlus => 180,
        Key::NumPadEnter => 181,

        Key::LeftAlt => 182,
        Key::RightAlt => 183,

        Key::LeftSuper => 184,
        Key::RightSuper => 185,

        Key::Unknown => 255,
        Key::Count => 255,
    }
}
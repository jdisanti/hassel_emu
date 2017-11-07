use minifb::{Window, WindowOptions};

use bus::Bus;
use cpu::Cpu;

const CHAR_WIDTH: usize = 9;
const CHAR_HEIGHT: usize = 16;
const SCREEN_WIDTH_CHARS: usize = 71;
const SCREEN_HEIGHT_CHARS: usize = 30;
const SCREEN_WIDTH_PIXELS: usize = CHAR_WIDTH * SCREEN_WIDTH_CHARS;
const SCREEN_HEIGHT_PIXELS: usize = CHAR_HEIGHT * SCREEN_HEIGHT_CHARS;
const FRAME_BUFFER_SIZE: usize = SCREEN_WIDTH_PIXELS * SCREEN_HEIGHT_PIXELS;

const FONT: &'static [u8] = include_bytes!("./vga_font_glyph9x16_image288x128_monochrome.data");
const FONT_WIDTH: usize = 288;
const FONT_CHARS_PER_ROW: usize = FONT_WIDTH / CHAR_WIDTH;
const DEFAULT_COLOR: u32 = 0xFFADAAAD;

enum IOState {
    Listening,

    ClearScreen,
    SetMode { mode: Option<u8> },
    SetPosition { x: Option<u8>, y: Option<u8> },
    SetColor { color: Option<u8> },
    SetValue { value: Option<u8> },
    SetValuesDma { high: Option<u8>, low: Option<u8>, length: Option<u8> },
}

pub struct GraphicsBus {
    frame_buffer: Vec<u32>,
    window: Window,
    next_command: IOState,
    cursor_x: u8,
    cursor_y: u8,
}

impl Bus for GraphicsBus {
    fn read_byte(&self, addr: u16) -> u8 {
        0
    }

    fn read_byte_mut(&mut self, addr: u16) -> u8 {
        0
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        self.next_command = match self.next_command {
            IOState::Listening => {
                match val {
                    0x01 => IOState::ClearScreen,
                    0x02 => IOState::SetMode { mode: None },
                    0x03 => IOState::SetPosition { x: None, y: None },
                    0x04 => IOState::SetColor { color: None },
                    0x05 => IOState::SetValue { value: None },
                    0x06 => IOState::SetValuesDma { high: None, low: None, length: None },
                    _ => IOState::Listening,
                }
            },
            IOState::ClearScreen => IOState::Listening,
            IOState::SetMode { mode } => IOState::SetMode { mode: Some(val) },
            IOState::SetPosition { x, y } => {
                if x.is_none() {
                    IOState::SetPosition { x: Some(val), y: y }
                } else {
                    IOState::SetPosition { x: x, y: Some(val) }
                }
            },
            IOState::SetColor { color } => IOState::SetColor { color: Some(val) },
            IOState::SetValue { value } => IOState::SetValue { value: Some(val) },
            IOState::SetValuesDma { high, low, length } => {
                if high.is_none() {
                    IOState::SetValuesDma { high: Some(val), low: low, length: length }
                } else if low.is_none() {
                    IOState::SetValuesDma { high: high, low: Some(val), length: length }
                } else {
                    IOState::SetValuesDma { high: high, low: low, length: Some(val) }
                }
            }
        };
    }
}

impl GraphicsBus {
    pub fn new() -> GraphicsBus {
        let mut frame_buffer: Vec<u32> = vec![0u32; FRAME_BUFFER_SIZE];
        let mut window = Window::new("Hasseldorf Emulator",
                SCREEN_WIDTH_PIXELS,
                SCREEN_HEIGHT_PIXELS,
                WindowOptions::default()).unwrap_or_else(|e| {
            panic!("Failed to create a window: {}", e);
        });

        let mut bus = GraphicsBus {
            frame_buffer: frame_buffer,
            window: window,
            next_command: IOState::Listening,
            cursor_x: 0,
            cursor_y: 0,
        };
        bus
    }

    pub fn execute_peripheral_operations(&mut self, cpu: &mut Cpu) {
        match self.next_command {
            IOState::Listening => { },
            IOState::ClearScreen => {
                println!("Cleared screen");
                self.frame_buffer = vec![0u32; FRAME_BUFFER_SIZE];
                self.next_command = IOState::Listening;
            },
            IOState::SetMode { mode } => {
                if mode.is_some() {
                    // TODO: set the mode
                    println!("Set graphics mode to {}", mode.unwrap());
                    self.next_command = IOState::Listening;
                }
            },
            IOState::SetPosition { x, y } => {
                if x.is_some() && y.is_some() {
                    self.cursor_x = x.unwrap();
                    self.cursor_y = y.unwrap();
                    println!("Set position to {}, {}", self.cursor_x, self.cursor_y);
                    self.next_command = IOState::Listening;
                }
            },
            IOState::SetColor { color } => {
                if color.is_some() {
                    // TODO: set the color
                    println!("Set color to {}", color.unwrap());
                    self.next_command = IOState::Listening;
                }
            },
            IOState::SetValue { value } => {
                if value.is_some() {
                    println!("Put character '{}' at {}, {}", value.unwrap() as char, self.cursor_x, self.cursor_y);
                    self.put_chr(value.unwrap());
                    self.next_command = IOState::Listening;
                }
            },
            IOState::SetValuesDma { high, low, length } => {
                if high.is_some() && low.is_some() && length.is_some() {
                    // TODO: DMA and display
                    let addr = ((high.unwrap() as u16) << 8) + low.unwrap() as u16;
                    println!("DMAing {} bytes from 0x{:04X} to VRAM", length.unwrap(), addr);
                    let slice = cpu.dma_slice(addr, length.unwrap() as u16);
                    for chr in slice {
                        self.put_chr(*chr);
                    }
                    self.next_command = IOState::Listening;
                }
            },
        }
    }

    pub fn is_good(&self) -> bool {
        self.window.is_open()
    }

    pub fn draw(&mut self) {
        self.window.update_with_buffer(&self.frame_buffer).unwrap();
    }

    fn put_chr(&mut self, code_point: u8) {
        if code_point == '\n' as u8 {
            self.cursor_x = 0;
            self.cursor_y += 1;
        } else if code_point == '\r' as u8 {
            self.cursor_x = 0;
        } else {
            let (x, y) = (self.cursor_x, self.cursor_y);
            self.blit_chr(x, y, code_point);
            self.cursor_x += 1;
            if self.cursor_x as usize >= SCREEN_WIDTH_CHARS {
                self.cursor_x = 0;
                self.cursor_y += 1;
            }
        }
    }

    fn blit_chr(&mut self, x_chr: u8, y_chr: u8, code_point: u8) {
        let src_start_x = (code_point as usize % FONT_CHARS_PER_ROW) * CHAR_WIDTH;
        let src_start_y = (code_point as usize / FONT_CHARS_PER_ROW) * CHAR_HEIGHT;

        let dst_start_x = x_chr as usize * CHAR_WIDTH;
        let dst_start_y = y_chr as usize * CHAR_HEIGHT;

        for row in 0..CHAR_HEIGHT {
            for col in 0..CHAR_WIDTH {
                let src_index = (src_start_y + row) * FONT_WIDTH + src_start_x + col;
                let dst_index = (dst_start_y + row) * SCREEN_WIDTH_PIXELS + dst_start_x + col;
                self.frame_buffer[dst_index as usize] = match FONT[src_index as usize] {
                    0 => 0,
                    _ => DEFAULT_COLOR,
                };
            }
        }
    }
}
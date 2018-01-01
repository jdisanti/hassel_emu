//
// Copyright 2017 hassel_emu Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use cpu::memory::{Interrupt, MemoryMap, MemoryMappedDevice};

const CHAR_WIDTH: usize = 9;
const CHAR_HEIGHT: usize = 16;
const SCREEN_WIDTH_CHARS: usize = 71;
const SCREEN_HEIGHT_CHARS: usize = 30;
pub const SCREEN_WIDTH_PIXELS: usize = 640;
pub const SCREEN_HEIGHT_PIXELS: usize = 480;
const FRAME_BUFFER_SIZE: usize = SCREEN_WIDTH_PIXELS * SCREEN_HEIGHT_PIXELS;

const FONT: &'static [u8] = include_bytes!("./vga_font_glyph9x16_image288x128_monochrome.data");
const FONT_WIDTH: usize = 288;
const FONT_CHARS_PER_ROW: usize = FONT_WIDTH / CHAR_WIDTH;
const DEFAULT_COLOR: u32 = 0xFFADAAAD;
const DEFAULT_BG_COLOR: u32 = 0xFF000000;

const CMD_CLEAR_SCREEN: u8 = 1;
const CMD_SET_MODE: u8 = 2;
const CMD_SET_POSITION: u8 = 3;
const CMD_SET_COLOR: u8 = 4;
const CMD_SET_VALUE: u8 = 5;
const CMD_SET_VALUES_DMA: u8 = 6;

enum IOState {
    Listening,

    ClearScreen,
    SetMode {
        mode: Option<u8>,
    },
    SetPosition {
        x: Option<u8>,
        y: Option<u8>,
    },
    SetColor {
        color: Option<u8>,
    },
    SetValue {
        value: Option<u8>,
    },
    SetValuesDma {
        high: Option<u8>,
        low: Option<u8>,
        length: Option<u8>,
    },
}

pub struct GraphicsDevice {
    frame_buffer: Vec<u32>,
    next_command: IOState,
    cursor_x: u8,
    cursor_y: u8,
}

impl GraphicsDevice {
    pub fn new() -> GraphicsDevice {
        let frame_buffer: Vec<u32> = vec![DEFAULT_BG_COLOR; FRAME_BUFFER_SIZE];

        let bus = GraphicsDevice {
            frame_buffer: frame_buffer,
            next_command: IOState::Listening,
            cursor_x: 0,
            cursor_y: 0,
        };
        bus
    }

    pub fn frame_buffer(&self) -> &[u32] {
        &self.frame_buffer
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
                if self.cursor_y as usize >= SCREEN_HEIGHT_CHARS {
                    self.cursor_y = 0;
                }
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
                self.frame_buffer[dst_index] = match FONT[src_index] {
                    0 => DEFAULT_BG_COLOR,
                    _ => DEFAULT_COLOR,
                };
            }
        }
    }
}

impl MemoryMappedDevice for GraphicsDevice {
    fn read_byte(&self, _addr: u16) -> u8 {
        0
    }

    fn read_byte_mut(&mut self, _addr: u16) -> u8 {
        0
    }

    fn write_byte(&mut self, _addr: u16, val: u8) {
        self.next_command = match self.next_command {
            IOState::Listening => match val {
                CMD_CLEAR_SCREEN => IOState::ClearScreen,
                CMD_SET_MODE => IOState::SetMode { mode: None },
                CMD_SET_POSITION => IOState::SetPosition { x: None, y: None },
                CMD_SET_COLOR => IOState::SetColor { color: None },
                CMD_SET_VALUE => IOState::SetValue { value: None },
                CMD_SET_VALUES_DMA => IOState::SetValuesDma {
                    high: None,
                    low: None,
                    length: None,
                },
                _ => IOState::Listening,
            },
            IOState::ClearScreen => IOState::Listening,
            IOState::SetMode { .. } => IOState::SetMode { mode: Some(val) },
            IOState::SetPosition { x, y } => {
                if x.is_none() {
                    IOState::SetPosition { x: Some(val), y: y }
                } else {
                    IOState::SetPosition { x: x, y: Some(val) }
                }
            }
            IOState::SetColor { .. } => IOState::SetColor { color: Some(val) },
            IOState::SetValue { .. } => IOState::SetValue { value: Some(val) },
            IOState::SetValuesDma { high, low, length } => {
                if high.is_none() {
                    IOState::SetValuesDma {
                        high: Some(val),
                        low: low,
                        length: length,
                    }
                } else if low.is_none() {
                    IOState::SetValuesDma {
                        high: high,
                        low: Some(val),
                        length: length,
                    }
                } else {
                    IOState::SetValuesDma {
                        high: high,
                        low: low,
                        length: Some(val),
                    }
                }
            }
        };
    }

    fn requires_step(&self) -> bool {
        true
    }

    fn step(&mut self, memory: &mut MemoryMap) -> Option<Interrupt> {
        match self.next_command {
            IOState::Listening => {}
            IOState::ClearScreen => {
                for i in 0..self.frame_buffer.len() {
                    self.frame_buffer[i] = DEFAULT_BG_COLOR;
                }
                self.next_command = IOState::Listening;
            }
            IOState::SetMode { mode } => {
                if mode.is_some() {
                    // TODO: set the mode
                    self.next_command = IOState::Listening;
                }
            }
            IOState::SetPosition { x, y } => {
                if x.is_some() && y.is_some() {
                    self.cursor_x = x.unwrap();
                    self.cursor_y = y.unwrap();
                    self.next_command = IOState::Listening;
                }
            }
            IOState::SetColor { color } => {
                if color.is_some() {
                    // TODO: set the color
                    self.next_command = IOState::Listening;
                }
            }
            IOState::SetValue { value } => {
                if value.is_some() {
                    self.put_chr(value.unwrap());
                    self.next_command = IOState::Listening;
                }
            }
            IOState::SetValuesDma { high, low, length } => {
                // TODO XXX
                if high.is_some() && low.is_some() && length.is_some() {
                    let addr = ((high.unwrap() as u16) << 8) + low.unwrap() as u16;
                    let mut buffer = Vec::with_capacity(length.unwrap() as usize);
                    memory.read().dma_slice(&mut buffer, addr);
                    for chr in buffer {
                        self.put_chr(chr);
                    }
                    self.next_command = IOState::Listening;
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_render_out_of_bounds() {
        // Expectation: writing out of bounds should not crash
        let address_doesnt_matter = 0;
        let mut bus = GraphicsDevice::new();
        for y in 0..256 {
            for x in 0..256 {
                bus.write_byte(address_doesnt_matter, CMD_SET_POSITION);
                bus.write_byte(address_doesnt_matter, x as u8);
                bus.write_byte(address_doesnt_matter, y as u8);
                bus.write_byte(address_doesnt_matter, CMD_SET_VALUE);
                bus.write_byte(address_doesnt_matter, 'h' as u8);
            }
        }

        bus.write_byte(address_doesnt_matter, CMD_SET_POSITION);
        bus.write_byte(address_doesnt_matter, 0);
        bus.write_byte(address_doesnt_matter, 0);

        for _ in 0..(SCREEN_WIDTH_CHARS * SCREEN_HEIGHT_CHARS + 1) {
            bus.write_byte(address_doesnt_matter, CMD_SET_VALUE);
            bus.write_byte(address_doesnt_matter, 'h' as u8);
        }
    }
}

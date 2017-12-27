use cpu::Cpu;

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
    next_command: IOState,
    cursor_x: u8,
    cursor_y: u8,
}

impl GraphicsBus {
    pub fn new() -> GraphicsBus {
        let frame_buffer: Vec<u32> = vec![DEFAULT_BG_COLOR; FRAME_BUFFER_SIZE];

        let bus = GraphicsBus {
            frame_buffer: frame_buffer,
            next_command: IOState::Listening,
            cursor_x: 0,
            cursor_y: 0,
        };
        bus
    }

    pub fn read_byte(&self, _addr: u16) -> u8 {
        0
    }

    pub fn read_byte_mut(&mut self, _addr: u16) -> u8 {
        0
    }

    pub fn write_byte(&mut self, _addr: u16, val: u8) {
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
            IOState::SetMode { .. } => IOState::SetMode { mode: Some(val) },
            IOState::SetPosition { x, y } => {
                if x.is_none() {
                    IOState::SetPosition { x: Some(val), y: y }
                } else {
                    IOState::SetPosition { x: x, y: Some(val) }
                }
            },
            IOState::SetColor { .. } => IOState::SetColor { color: Some(val) },
            IOState::SetValue { .. } => IOState::SetValue { value: Some(val) },
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

    pub fn execute_peripheral_operations(&mut self, cpu: &mut Cpu) {
        match self.next_command {
            IOState::Listening => { },
            IOState::ClearScreen => {
                self.frame_buffer = vec![DEFAULT_BG_COLOR; FRAME_BUFFER_SIZE];
                self.next_command = IOState::Listening;
            },
            IOState::SetMode { mode } => {
                if mode.is_some() {
                    // TODO: set the mode
                    self.next_command = IOState::Listening;
                }
            },
            IOState::SetPosition { x, y } => {
                if x.is_some() && y.is_some() {
                    self.cursor_x = x.unwrap();
                    self.cursor_y = y.unwrap();
                    self.next_command = IOState::Listening;
                }
            },
            IOState::SetColor { color } => {
                if color.is_some() {
                    // TODO: set the color
                    self.next_command = IOState::Listening;
                }
            },
            IOState::SetValue { value } => {
                if value.is_some() {
                    self.put_chr(value.unwrap());
                    self.next_command = IOState::Listening;
                }
            },
            IOState::SetValuesDma { high, low, length } => {
                if high.is_some() && low.is_some() && length.is_some() {
                    // TODO: DMA and display
                    let addr = ((high.unwrap() as u16) << 8) + low.unwrap() as u16;
                    let slice = cpu.dma_slice(addr, length.unwrap() as u16);
                    for chr in slice {
                        self.put_chr(*chr);
                    }
                    self.next_command = IOState::Listening;
                }
            },
        }
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
                    0 => DEFAULT_BG_COLOR,
                    _ => DEFAULT_COLOR,
                };
            }
        }
    }
}
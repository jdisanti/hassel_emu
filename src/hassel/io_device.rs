//
// Copyright 2017 hassel_emu Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use cpu::memory::{MemoryMap, MemoryMappedDevice, Interrupt};
use hassel::key::Key;

const KEY_DOWN_INTERRUPT: u8 = 0x01;
const KEY_UP_INTERRUPT: u8 = 0x02;

const MAX_RESPONSE_QUEUE_SIZE: usize = 32;

pub struct IODevice {
    response_queue: Vec<u8>,
    last_interrupt_size: usize,
}

impl IODevice {
    pub fn new() -> IODevice {
        IODevice {
            response_queue: Vec::new(),
            last_interrupt_size: 0,
        }
    }

    pub fn key_down(&mut self, key: Key) {
        self.push_response(&[KEY_DOWN_INTERRUPT, key.into()]);
    }

    pub fn key_up(&mut self, key: Key) {
        self.push_response(&[KEY_UP_INTERRUPT, key.into()]);
    }

    fn push_response(&mut self, values: &[u8]) {
        // If our queue is full, we will start dropping responses
        if self.response_queue.len() + values.len() < MAX_RESPONSE_QUEUE_SIZE {
            self.response_queue.extend(values);
        }
    }
}

impl MemoryMappedDevice for IODevice {
    fn read_byte(&self, _addr: u16) -> u8 {
        0
    }

    fn read_byte_mut(&mut self, _addr: u16) -> u8 {
        if self.response_queue.is_empty() {
            0
        } else {
            // Avoid re-interrupting
            self.last_interrupt_size = self.response_queue.len() - 1;
            self.response_queue.remove(0)
        }
    }

    fn write_byte(&mut self, _addr: u16, _val: u8) {
        // No-op for now
    }

    fn requires_step(&self) -> bool {
        true
    }

    fn step(&mut self, _memory: &mut MemoryMap) -> Option<Interrupt> {
        if !self.response_queue.is_empty() && self.last_interrupt_size != self.response_queue.len() {
            self.last_interrupt_size = self.response_queue.len();
            Some(Interrupt::Interrupt)
        } else {
            None
        }
    }
}
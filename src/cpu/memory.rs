//
// Copyright 2017 hassel_emu Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use std::rc::Rc;
use std::cell::{Ref, RefCell, RefMut};
use std::mem;

macro_rules! read_word {
    ($memory:ident, $addr:expr) => {
        {
            let lsb = $memory.byte($addr as u16);
            let msb = $memory.byte($addr.wrapping_add(1) as u16);
            (msb as u16) << 8 | (lsb as u16)
        }
    }
}

pub trait ReadMemory {
    fn byte(&self, addr: u16) -> u8;

    fn word(&self, addr: u16) -> u16 {
        read_word!(self, addr)
    }

    fn word_zero_page(&self, addr: u8) -> u16 {
        read_word!(self, addr)
    }

    fn dma_slice(&self, into: &mut [u8], dma_addr: u16) {
        for i in 0..into.len() {
            let addr = dma_addr.wrapping_add(i as u16);
            into[i] = self.byte(addr);
        }
    }
}

pub trait ReadMemoryMut {
    fn byte(&mut self, addr: u16) -> u8;

    fn word(&mut self, addr: u16) -> u16 {
        read_word!(self, addr)
    }

    fn word_zero_page(&mut self, addr: u8) -> u16 {
        read_word!(self, addr)
    }

    fn dma_slice(&mut self, into: &mut [u8], dma_addr: u16) {
        for i in 0..into.len() {
            let addr = dma_addr.wrapping_add(i as u16);
            into[i] = self.byte(addr);
        }
    }
}

pub trait WriteMemory {
    fn byte(&mut self, addr: u16, val: u8);
}

pub struct DebugMemoryView<'a> {
    device: Ref<'a, MemoryMappedDevice>,
}

impl<'a> ReadMemory for DebugMemoryView<'a> {
    fn byte(&self, addr: u16) -> u8 {
        self.device.read_byte(addr)
    }
}

pub struct NormalMemoryView<'a> {
    device: RefMut<'a, MemoryMappedDevice>,
}

impl<'a> ReadMemoryMut for NormalMemoryView<'a> {
    fn byte(&mut self, addr: u16) -> u8 {
        self.device.read_byte_mut(addr)
    }
}

impl<'a> WriteMemory for NormalMemoryView<'a> {
    fn byte(&mut self, addr: u16, val: u8) {
        self.device.write_byte(addr, val);
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Interrupt {
    Interrupt,
    NonMaskableInterrupt,
}

pub trait MemoryMappedDevice {
    fn read_byte(&self, addr: u16) -> u8;

    /// There's a mutable read to support peripherals because
    /// reading their registers can cause a state change
    /// as an expected part of how the hardware behaves
    fn read_byte_mut(&mut self, addr: u16) -> u8;

    fn write_byte(&mut self, addr: u16, val: u8);

    fn requires_step(&self) -> bool;

    fn step(&mut self, memory: &mut MemoryMap) -> Option<Interrupt>;
}

pub struct RAMDevice {
    start: u16,
    memory: Vec<u8>,
}

impl RAMDevice {
    pub fn new(start: u16, size: usize) -> RAMDevice {
        RAMDevice {
            start: start,
            memory: vec![0u8; size],
        }
    }
}

impl MemoryMappedDevice for RAMDevice {
    fn read_byte(&self, addr: u16) -> u8 {
        let offset = addr.wrapping_sub(self.start);
        self.memory[offset as usize]
    }

    fn read_byte_mut(&mut self, addr: u16) -> u8 {
        self.read_byte(addr)
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        let offset = addr.wrapping_sub(self.start);
        self.memory[offset as usize] = val;
    }

    fn requires_step(&self) -> bool {
        false
    }

    fn step(&mut self, _memory: &mut MemoryMap) -> Option<Interrupt> {
        None
    }
}

pub struct ROMDevice {
    start: u16,
    memory: Vec<u8>,
}

impl ROMDevice {
    pub fn new(start: u16, memory: Vec<u8>) -> ROMDevice {
        ROMDevice {
            start: start,
            memory: memory,
        }
    }
}

impl MemoryMappedDevice for ROMDevice {
    fn read_byte(&self, addr: u16) -> u8 {
        let offset = addr.wrapping_sub(self.start);
        self.memory[offset as usize]
    }

    fn read_byte_mut(&mut self, addr: u16) -> u8 {
        self.read_byte(addr)
    }

    fn write_byte(&mut self, _addr: u16, _val: u8) {}

    fn requires_step(&self) -> bool {
        false
    }

    fn step(&mut self, _memory: &mut MemoryMap) -> Option<Interrupt> {
        None
    }
}

struct NullDevice {}

impl NullDevice {
    fn new() -> NullDevice {
        NullDevice {}
    }
}

impl MemoryMappedDevice for NullDevice {
    fn read_byte(&self, _addr: u16) -> u8 {
        0
    }

    fn read_byte_mut(&mut self, _addr: u16) -> u8 {
        0
    }

    fn write_byte(&mut self, _addr: u16, _val: u8) {}

    fn requires_step(&self) -> bool {
        false
    }

    fn step(&mut self, _memory: &mut MemoryMap) -> Option<Interrupt> {
        None
    }
}

#[derive(Clone)]
pub struct MemorySegment {
    start: u16,
    end_inclusive: u16,
    device: Rc<RefCell<MemoryMappedDevice>>,
    requires_step: bool,
}

impl MemorySegment {
    fn new(start: u16, end_inclusive: u16, device: Rc<RefCell<MemoryMappedDevice>>) -> MemorySegment {
        let requires_step = device.borrow().requires_step();
        MemorySegment {
            start: start,
            end_inclusive: end_inclusive,
            device: device,
            requires_step: requires_step,
        }
    }

    pub fn debug<'a>(&'a self) -> DebugMemoryView<'a> {
        DebugMemoryView {
            device: self.device.borrow(),
        }
    }

    pub fn normal<'a>(&'a self) -> NormalMemoryView<'a> {
        NormalMemoryView {
            device: self.device.borrow_mut(),
        }
    }

    pub fn step(&self, memory_map: &mut MemoryMap) -> Option<Interrupt> {
        self.device.borrow_mut().step(memory_map)
    }

    fn requires_step(&self) -> bool {
        self.requires_step
    }

    fn with_device(&self, device: Rc<RefCell<MemoryMappedDevice>>) -> MemorySegment {
        MemorySegment::new(self.start, self.end_inclusive, device)
    }
}

pub struct MemoryMapBuilder {
    segments: Vec<MemorySegment>,
}

impl MemoryMapBuilder {
    pub fn new() -> MemoryMapBuilder {
        MemoryMapBuilder {
            segments: Vec::new(),
        }
    }

    pub fn ram(self, start: u16, end_inclusive: u16) -> Self {
        assert!(end_inclusive >= start);
        let length = (end_inclusive as usize + 1) - start as usize;
        self.peripheral(
            start,
            end_inclusive,
            Rc::new(RefCell::new(RAMDevice::new(start, length))),
        )
    }

    pub fn rom(self, start: u16, end_inclusive: u16, data: Vec<u8>) -> Self {
        assert!(end_inclusive >= start);
        let length = (end_inclusive as usize + 1) - start as usize;
        assert_eq!(
            length,
            data.len(),
            "given rom is not the right size for the built memory map"
        );
        self.peripheral(
            start,
            end_inclusive,
            Rc::new(RefCell::new(ROMDevice::new(start, data))),
        )
    }

    pub fn peripheral(mut self, start: u16, end_inclusive: u16, device: Rc<RefCell<MemoryMappedDevice>>) -> Self {
        self.segments
            .push(MemorySegment::new(start, end_inclusive, device));
        self
    }

    pub fn build(mut self) -> MemoryMap {
        self.segments.sort_by(|lhs, rhs| lhs.start.cmp(&rhs.start));

        let mut address: usize = 0;
        for segment in &self.segments {
            assert!(
                address == segment.start as usize,
                "found a gap in built memory map"
            );
            address = segment.end_inclusive as usize + 1;
        }
        assert!(
            address == 0x10000,
            "built memory map doesn't cover the full address range"
        );

        MemoryMap::new(self.segments)
    }
}

struct MemoryMapInner {
    segments: Vec<MemorySegment>,
}

impl MemoryMapInner {
    fn segment(&self, addr: u16) -> &MemorySegment {
        for segment in &self.segments {
            if segment.start <= addr && segment.end_inclusive >= addr {
                return segment;
            }
        }
        unreachable!()
    }
}

impl ReadMemory for MemoryMapInner {
    fn byte(&self, addr: u16) -> u8 {
        self.segment(addr).debug().byte(addr)
    }
}

impl ReadMemoryMut for MemoryMapInner {
    fn byte(&mut self, addr: u16) -> u8 {
        ReadMemoryMut::byte(&mut self.segment(addr).normal(), addr)
    }
}

impl WriteMemory for MemoryMapInner {
    fn byte(&mut self, addr: u16, val: u8) {
        WriteMemory::byte(&mut self.segment(addr).normal(), addr, val);
    }
}

pub struct MemoryMap {
    inner: MemoryMapInner,
    working_segment_cache: Option<Vec<MemorySegment>>,
    null_device_cache: Option<Rc<RefCell<MemoryMappedDevice>>>,
}

impl MemoryMap {
    fn new(segments: Vec<MemorySegment>) -> MemoryMap {
        MemoryMap {
            inner: MemoryMapInner { segments: segments },
            working_segment_cache: None,
            null_device_cache: None,
        }
    }

    pub fn builder() -> MemoryMapBuilder {
        MemoryMapBuilder::new()
    }

    pub fn debug_read(&self) -> &ReadMemory {
        &self.inner
    }

    pub fn read(&mut self) -> &mut ReadMemoryMut {
        &mut self.inner
    }

    pub fn write(&mut self) -> &mut WriteMemory {
        &mut self.inner
    }

    pub fn step(&mut self) -> Option<Interrupt> {
        if self.working_segment_cache.is_none() {
            self.working_segment_cache = Some(self.inner.segments.clone());
        }
        if self.null_device_cache.is_none() {
            self.null_device_cache = Some(Rc::new(RefCell::new(NullDevice::new())));
        }

        let mut working_segments = self.working_segment_cache.take().unwrap();
        let null_device = self.null_device_cache.as_ref().unwrap();

        let mut result = None;
        for i in 0..working_segments.len() {
            if self.inner.segments[i].requires_step() {
                // Swap in a null device in place of the device we're stepping
                let mut current_segment = working_segments[i].with_device(Rc::clone(&null_device));
                mem::swap(&mut current_segment, &mut working_segments[i]);

                // Step the device
                let mut partial_memory_map = MemoryMap::new(working_segments);
                if let Some(interrupt) = current_segment.step(&mut partial_memory_map) {
                    if result != Some(Interrupt::NonMaskableInterrupt) {
                        result = Some(interrupt);
                    }
                }
                working_segments = partial_memory_map.relinquish();

                // Swap the device back into the list so it can be accessed
                // for the next device's step
                mem::swap(&mut current_segment, &mut working_segments[i]);
            }
        }
        self.working_segment_cache = Some(working_segments);
        result
    }

    fn relinquish(self) -> Vec<MemorySegment> {
        self.inner.segments
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interfaces() {
        let mut memory = MemoryMap::builder()
            .ram(0x0000, 0xCFFF)
            .rom(0xD000, 0xFFFF, vec![0xEA; 0x10000 - 0xD000])
            .build();

        assert_eq!(0x00, memory.read().byte(0x0005));
        memory.write().byte(0x0005, 0xFF);
        assert_eq!(0xFF, memory.read().byte(0x0005));

        assert_eq!(0xEA, memory.read().byte(0xD000));
        memory.write().byte(0xD000, 0x00);
        assert_eq!(0xEA, memory.read().byte(0xD000));

        assert_eq!(None, memory.step());
    }
}

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

use emulator::cpu::InterruptType;

macro_rules! read_word {
    ($memory:ident, $addr:expr) => {
        {
            let lsb = $memory.byte($addr as u16);
            let msb = $memory.byte($addr.wrapping_add(1) as u16);
            (msb as u16) << 8 | (lsb as u16)
        }
    }
}

/// Exposes only non-mutable read operations.
/// Useful for debugging or introspection where you don't
/// want to modify the behavior of the system by observing it
/// (since reading a peripheral's memory address can change its
/// state under normal conditions). This shouldn't be used by
/// the emulator or its devices.
pub trait ReadMemory {
    /// Reads a byte at the requested address
    fn byte(&self, addr: u16) -> u8;

    /// Reads a 16-bit word at the requested address
    fn word(&self, addr: u16) -> u16 {
        read_word!(self, addr)
    }

    /// Reads a 16-bit word at the requested zero-page offset
    fn word_zero_page(&self, addr: u8) -> u16 {
        read_word!(self, addr)
    }

    /// Reads an entire slice from memory into the given buffer
    fn dma_slice(&self, into: &mut [u8], dma_addr: u16) {
        for i in 0..into.len() {
            let addr = dma_addr.wrapping_add(i as u16);
            into[i] = self.byte(addr);
        }
    }
}

/// Read memory operations
pub trait ReadMemoryMut {
    /// Reads a byte at the requested address
    fn byte(&mut self, addr: u16) -> u8;

    /// Reads a 16-bit word at the requested address
    fn word(&mut self, addr: u16) -> u16 {
        read_word!(self, addr)
    }

    /// Reads a 16-bit word at the requested zero-page offset
    fn word_zero_page(&mut self, addr: u8) -> u16 {
        read_word!(self, addr)
    }

    /// Reads an entire slice from memory into the given buffer
    fn dma_slice(&mut self, into: &mut [u8], dma_addr: u16) {
        for i in 0..into.len() {
            let addr = dma_addr.wrapping_add(i as u16);
            into[i] = self.byte(addr);
        }
    }
}

/// Write memory operations
pub trait WriteMemory {
    /// Writes a byte to the requested address
    fn byte(&mut self, addr: u16, val: u8);
}

/// A non-mutable view into a memory mapped device.
/// This exists mostly to work around lifetime issues with RefCell.
pub struct DebugMemoryView<'a> {
    device: Ref<'a, MemoryMappedDevice>,
}

impl<'a> ReadMemory for DebugMemoryView<'a> {
    fn byte(&self, addr: u16) -> u8 {
        self.device.read_byte(addr)
    }
}

/// A mutable view into a memory mapped device
/// This exists mostly to work around lifetime issues with RefCell.
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

/// Any device that is attached to the system bus, including (but not limited to):
///
/// * RAM
/// * ROM
/// * Graphics cards
/// * Sound cards
/// * IO devices
/// * Printers
pub trait MemoryMappedDevice {
    /// Immutably reads a byte from the device. For RAM and ROM,
    /// this can just return that part of memory. For peripherals, however,
    /// this may not be able to return the actual value that the peripheral
    /// would ordinarily return. This method should only be used for debugging
    /// and introspection.
    fn read_byte(&self, addr: u16) -> u8;

    /// Mutably reads a byte from this device.
    /// There's a mutable read to support peripherals because
    /// reading their registers can cause a state change
    /// as an expected part of how the hardware behaves
    fn read_byte_mut(&mut self, addr: u16) -> u8;

    /// Writes a byte to this device
    fn write_byte(&mut self, addr: u16, val: u8);

    /// Tells the MemoryMap whether or not this device actually
    /// does anything in its step method. This value is cached,
    /// so it should be constant. This is merely an optimization.
    fn requires_step(&self) -> bool;

    /// Provides a mechanism for the device to update itself
    /// that is called on every instruction execution. This should probably
    /// take a cycle count to support peripheral timings, but it doesn't currently.
    fn step(&mut self, memory: &mut MemoryMap) -> Option<InterruptType>;
}

/// Random access memory device
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

    fn step(&mut self, _memory: &mut MemoryMap) -> Option<InterruptType> {
        None
    }
}

/// Readonly memory device
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

    fn step(&mut self, _memory: &mut MemoryMap) -> Option<InterruptType> {
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

    fn step(&mut self, _memory: &mut MemoryMap) -> Option<InterruptType> {
        None
    }
}

#[derive(Clone)]
struct MemorySegment {
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

    pub fn step(&self, memory_map: &mut MemoryMap) -> Option<InterruptType> {
        self.device.borrow_mut().step(memory_map)
    }

    fn requires_step(&self) -> bool {
        self.requires_step
    }

    fn with_device(&self, device: Rc<RefCell<MemoryMappedDevice>>) -> MemorySegment {
        MemorySegment::new(self.start, self.end_inclusive, device)
    }
}

/// Builder interface for constructing a memory map
pub struct MemoryMapBuilder {
    segments: Vec<MemorySegment>,
}

impl MemoryMapBuilder {
    /// Creates a new builder
    pub fn new() -> MemoryMapBuilder {
        MemoryMapBuilder {
            segments: Vec::new(),
        }
    }

    /// Adds RAM to the memory map
    pub fn ram(self, start: u16, end_inclusive: u16) -> Self {
        assert!(end_inclusive >= start);
        let length = (end_inclusive as usize + 1) - start as usize;
        self.peripheral(
            start,
            end_inclusive,
            Rc::new(RefCell::new(RAMDevice::new(start, length))),
        )
    }

    /// Adds ROM to the memory map
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

    /// Adds a peripheral to the memory map
    pub fn peripheral(mut self, start: u16, end_inclusive: u16, device: Rc<RefCell<MemoryMappedDevice>>) -> Self {
        self.segments
            .push(MemorySegment::new(start, end_inclusive, device));
        self
    }

    /// Constructs and validates the memory map. Checks for
    /// gaps and overlaps in the configured memory segments.
    /// Also verifies the entire address space is covered by
    /// devices.
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

/// A memory mapping representing a system architecture
/// for the MOS 6502 processor.
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

    /// Returns a new builder for to create a new memory map
    pub fn builder() -> MemoryMapBuilder {
        MemoryMapBuilder::new()
    }

    /// Returns a debug read view that won't affect the system when reading
    pub fn debug_read(&self) -> &ReadMemory {
        &self.inner
    }

    /// Returns a normal read view that may change peripheral state when reading their address ranges
    pub fn read(&mut self) -> &mut ReadMemoryMut {
        &mut self.inner
    }

    /// Returns a write view into memory
    pub fn write(&mut self) -> &mut WriteMemory {
        &mut self.inner
    }

    /// Updates all attached peripherals. This should get called by the Cpu.
    pub fn step(&mut self) -> Option<InterruptType> {
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
                    if result != Some(InterruptType::NonMaskable) {
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

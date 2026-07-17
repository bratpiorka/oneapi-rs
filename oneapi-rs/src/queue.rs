//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

use bytemuck::Pod;
use oneapi_rs_sys::queue::ffi;

use crate::{buffer::{Buffer, BufferInProgress}, device::Device, event::Event, usm::{HostAllocator, SharedAllocator, UsmAlloc, UsmAllocator}};

/// The `Queue` connects a host program to a single device. Programs submit tasks to a device via the
/// `Queue` and may monitor the `Queue` for completion. A program initiates the task by submitting
/// a kernel.
pub struct Queue(pub(crate) cxx::UniquePtr<ffi::Queue>);

impl Queue {
    /// Construct a `Queue` based on the device returned from the default selector.
    pub fn new() -> Self {
        Self(ffi::new_queue())
    }

    /// Construct an immediate `Queue` based on the device returned from the default selector.
    pub fn new_immediate() -> Self {
        Self(ffi::new_queue_immediate())
    }

    /// Allocates zeroed memory and creates a host-side [`Buffer`] that can store an array of T.
    pub fn alloc_host<T: Pod>(&mut self, len: usize) -> BufferInProgress<T, UsmAllocator<HostAllocator>> {
        unsafe {
            let mut buffer = self.alloc_uninit_host(len);
            let event = self.memset(&mut buffer, 0);
            BufferInProgress::new(buffer, event)
        }
    }

    /// Allocates zeroed memory and creates a shared [`Buffer`] that can store an array of T.
    pub fn alloc_shared<T: Pod>(&mut self, len: usize) -> BufferInProgress<T, UsmAllocator<SharedAllocator>> {
        unsafe {
            let mut buffer = self.alloc_uninit_shared(len);
            let event = self.memset(&mut buffer, 0);
            BufferInProgress::new(buffer, event)
        }
    }

    /// Allocates memory and creates a host-side [`Buffer`] that can store an array of T.
    /// Safety: the buffer contents are uninitialized.
    pub unsafe fn alloc_uninit_host<T>(&self, len: usize) -> Buffer<T, UsmAllocator<HostAllocator>> {
        let allocator = UsmAllocator::from(self);
        unsafe { Buffer::new(allocator, len) }
    }

    /// Allocates memory and creates a shared [`Buffer`] that can store an array of T.
    /// Safety: the buffer contents are uninitialized.
    pub unsafe fn alloc_uninit_shared<T>(&self, len: usize) -> Buffer<T, UsmAllocator<SharedAllocator>> {
        let allocator = UsmAllocator::from(self);
        unsafe { Buffer::new(allocator, len) }
    }

    pub unsafe fn memset<T, A: UsmAlloc>(&mut self, buffer: &mut Buffer<T, A>, value: i32) -> Event {
        let ptr = buffer.get_byte_ptr();
        let num_bytes = buffer.get_byte_size();
        unsafe { ffi::memset(&mut self.0, ptr, value, num_bytes) }.into()
    }

    pub fn barrier(&mut self) -> Event {
        ffi::barrier(&mut self.0).into()
    }

    pub fn wait(&mut self) {
        ffi::wait(&mut self.0);
    }
}

impl From<&Device> for Queue {
    fn from(value: &Device) -> Self {
        Self(ffi::new_queue_from_device(&value.0))
    }
}

impl From<cxx::UniquePtr<ffi::Queue>> for Queue {
    fn from(value: cxx::UniquePtr<ffi::Queue>) -> Self {
        Self(value)
    }
}

impl Clone for Queue {
    fn clone(&self) -> Self {
        ffi::clone(&self.0).into()
    }
}

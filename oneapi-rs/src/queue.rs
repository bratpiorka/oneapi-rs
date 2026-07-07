//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

use oneapi_rs_sys::queue::ffi;

use crate::{buffer::Buffer, device::Device, usm::{HostAllocator, SharedAllocator, UsmAllocator}};

/// The `Queue` connects a host program to a single device. Programs submit tasks to a device via the
/// `Queue` and may monitor the `Queue` for completion. A program initiates the task by submitting
/// a kernel.
pub struct Queue(pub(crate) cxx::UniquePtr<ffi::Queue>);

impl Queue {
    /// Construct a `Queue` based on the device returned from the default selector.
    pub fn new() -> Self {
        Self(ffi::new_queue())
    }

    pub fn alloc_host<T, const N: usize>(&self) -> Buffer<T, N, UsmAllocator<'_, HostAllocator>> {
        let allocator = UsmAllocator::from(self);
        Buffer::new(allocator)
    }

    pub fn alloc_shared<T, const N: usize>(&self) -> Buffer<T, N, UsmAllocator<'_, SharedAllocator>> {
        let allocator = UsmAllocator::from(self);
        Buffer::new(allocator)
    }
}

impl From<&Device> for Queue {
    fn from(value: &Device) -> Self {
        Self(ffi::new_queue_from_device(&value.0))
    }
}
